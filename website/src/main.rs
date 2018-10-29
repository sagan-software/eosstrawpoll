extern crate log;
extern crate web_logger;
extern crate website;
extern crate yew;

use website::Model;
use yew::prelude::*;

fn main() {
    web_logger::init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
