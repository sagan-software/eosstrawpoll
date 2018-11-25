#![recursion_limit = "500"]
#![warn(clippy)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate yew;
extern crate http;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate failure;

#[macro_use]
mod macros;

mod app;
mod chains;
mod components;
mod context;
mod eos;
mod pages;
mod prelude;
mod router;
mod scatter;
mod seo;
mod traits;
mod views;

pub use crate::app::App as Model;
