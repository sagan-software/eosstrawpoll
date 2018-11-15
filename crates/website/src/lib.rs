mod app;
mod context;
mod home_page;
mod i18n;
mod poll_page;
mod route;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Node};

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let context = crate::context::Context::new()?;
    let app_container = app::create_container(&context)?;

    let node = AsRef::<Node>::as_ref(&context.body);
    node.append_child(app_container.as_ref())?;

    Ok(())
}
