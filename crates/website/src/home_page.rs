use crate::context::Context;
use crate::route::Route;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Node};

pub fn create(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("main")?;
    el.set_class_name("home_page");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(page_title(ctx)?.as_ref())?;
    node.append_child(poll_form(ctx)?.as_ref())?;

    Ok(el)
}

fn page_title(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("h1")?;
    el.set_class_name("page_title");

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("Real-Time Polls on EOS"));

    Ok(el)
}

fn poll_form(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("form")?;
    el.set_class_name("poll_form");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(poll_form_title(ctx)?.as_ref());
    node.append_child(poll_form_options(ctx)?.as_ref());
    node.append_child(poll_form_submit(ctx)?.as_ref());

    Ok(el)
}

fn poll_form_title(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("input")?;
    el.set_class_name("poll_form_title");
    el.set_attribute("placeholder", "What is your question?");
    el.set_attribute("required", "true");
    Ok(el)
}

fn poll_form_options(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("div")?;
    el.set_class_name("poll_form_options");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(poll_form_option(ctx, 1)?.as_ref());
    node.append_child(poll_form_option(ctx, 2)?.as_ref());
    node.append_child(poll_form_option(ctx, 3)?.as_ref());

    Ok(el)
}

fn poll_form_option(ctx: &Context, index: usize) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("div")?;
    el.set_class_name("poll_form_option");

    let node = AsRef::<Node>::as_ref(&el);
    node.append_child(poll_form_option_input(ctx, index)?.as_ref());
    node.append_child(poll_form_option_button(ctx, index)?.as_ref());

    Ok(el)
}

fn poll_form_option_input(ctx: &Context, index: usize) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("input")?;
    el.set_class_name("poll_form_option_input");
    el.set_attribute("placeholder", "Option");
    Ok(el)
}

fn poll_form_option_button(ctx: &Context, index: usize) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("button")?;
    el.set_class_name("poll_form_option_button");

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("×"));

    // let onclick = Closure::wrap(Box::new(move || {
    //     let parent = node.parent_node().expect("expected parent");
    //     let grandparent = parent.parent_node().expect("expected grandparent");
    //     grandparent
    //         .remove_child(&parent)
    //         .expect("to remove poll option");
    // }) as Box<FnMut()>);

    // el.dyn_ref::<HtmlElement>()
    //     .expect(".poll_form_option_button be an `HtmlElement`")
    //     .set_onclick(Some(onclick.as_ref().unchecked_ref()));

    // onclick.forget();

    Ok(el)
}

fn poll_form_submit(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.doc.create_element("button")?;
    el.set_class_name("poll_form_submit");

    let node = AsRef::<Node>::as_ref(&el);
    node.set_text_content(Some("Create Poll"));

    Ok(el)
}
