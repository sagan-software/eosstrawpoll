use crate::context::Context;
use crate::route::Route;
use futures::{future, Future};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Document, Element, Event, EventTarget, Node};

pub fn create_container(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("div")?;
    el.set_class_name("app_container");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(app_header(ctx)?.as_ref())?;
    node.append_child(app_main(ctx)?.as_ref())?;
    node.append_child(app_footer(ctx)?.as_ref())?;

    Ok(el)
}

fn app_header(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("header")?;
    el.set_class_name("app_header");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(app_logo(ctx)?.as_ref())?;
    node.append_child(app_nav(ctx)?.as_ref())?;

    Ok(el)
}

fn app_logo(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("a")?;
    el.set_class_name("app_logo");

    let route = Route::Home;
    el.set_attribute("href", &route.to_string())?;

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("EOS Straw Poll"));
    Ok(el)
}

fn app_nav(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("nav")?;
    el.set_class_name("app_nav");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(settings_link(ctx)?.as_ref())?;
    node.append_child(login_button(ctx)?.as_ref())?;

    Ok(el)
}

mod scatter {
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "scatterjs-core")]
    extern "C" {
        pub fn plugins(plugin: &JsValue);

        #[wasm_bindgen(js_namespace = scatter)]
        pub fn connect(label: &str) -> Promise;
    }
}

#[wasm_bindgen(module = "static-dir/bindgen")]
extern "C" {
    pub type ScatterEOS;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ScatterEOS;
}

fn login_button(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("button")?;
    el.set_class_name("login_button");

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("Login"));

    // scatter::plugins(&ScatterEOS::new().into());
    // scatter::connect("balls");
    eosio_rpc::chain::get_info("https://localhost:8889");

    // .and_then(|info| {
    // let v = JsValue::from_serde(&branch_info).unwrap();
    // console::log_1(&v);
    // });

    let onclick = Closure::wrap(Box::new(move |e: Event| {
        e.prevent_default();
        console::log_1(&"CLICKED LOGIN".into());
    }) as Box<FnMut(_)>);

    let el_et: web_sys::EventTarget = el.into();
    el_et.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref());

    onclick.forget();

    Ok(el_et.dyn_into::<Element>().expect(""))
}

fn settings_link(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("a")?;
    el.set_class_name("settings_link");

    let route = Route::Settings;
    el.set_attribute("href", &route.to_string())?;

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("Settings"));

    Ok(el)
}

fn app_main(ctx: &Context) -> Result<Element, JsValue> {
    let el = crate::home_page::create(ctx)?;
    el.class_list().add_1("app_main");
    Ok(el)
}

fn app_footer(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("footer")?;
    el.set_class_name("app_footer");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(company_link(ctx)?.as_ref())?;
    node.append_child(social_links(ctx)?.as_ref())?;

    Ok(el)
}

fn company_link(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("a")?;
    el.set_class_name("company_link");
    el.set_attribute("href", "//sagan.software")?;

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("sagan.software"));
    Ok(el)
}

fn social_links(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("nav")?;
    el.set_class_name("social_links");

    let node = AsRef::<Node>::as_ref(&el);

    let github = social_link(ctx, "Github", "https://github.com/sagan-software")?;
    node.append_child(github.as_ref())?;

    let twitter = social_link(ctx, "Twitter", "https://twitter.com/SaganSoftware")?;
    node.append_child(twitter.as_ref())?;

    let telegram = social_link(ctx, "Telegram", "https://t.me/SaganSoftware")?;
    node.append_child(telegram.as_ref())?;

    Ok(el)
}

fn social_link(ctx: &Context, text: &str, href: &str) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("a")?;
    el.set_class_name("social_link");
    el.set_attribute("href", href)?;

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some(text));
    Ok(el)
}
