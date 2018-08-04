extern crate eosstrawpoll;
extern crate yew;

// use counter::Model;
use eosstrawpoll::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
