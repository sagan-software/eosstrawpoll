use std::fmt::Display;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Debug)]
pub enum Icon {
    Trash,
    Login,
    Logout,
    Calendar,
    Github,
    Twitter,
    Telegram,
}

impl<T: Component> Renderable<T> for Icon {
    fn view(&self) -> Html<T> {
        let node = Node::from_html("<svg></svg>").unwrap();
        VNode::VRef(node)
    }
}
