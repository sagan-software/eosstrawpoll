use crate::route::Route;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Location, Window};

pub struct Context {
    pub window: Window,
    pub doc: Document,
    pub body: HtmlElement,
    pub route: Route,
}

impl Context {
    pub fn new() -> Result<Context, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let doc = window.document().expect("should have a document on window");
        let body = doc.body().expect("document should have a body");
        let route = Route::default();
        let context = Context {
            window,
            doc,
            body,
            route,
        };
        Ok(context)
    }
}
