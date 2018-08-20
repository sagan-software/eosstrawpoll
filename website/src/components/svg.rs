use std::fmt::Display;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(PartialEq, Clone, Debug)]
pub enum Symbol {
    Trash,
    Login,
    Logout,
    Calendar,
    Github,
    Twitter,
    Telegram,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Symbol::Trash
    }
}

const TRASH: &str = "<svg width='8' height='8' viewBox='0 0 8 8'>  <path d='M3 0c-.55 0-1 .45-1 1h-1c-.55 0-1 .45-1 1h7c0-.55-.45-1-1-1h-1c0-.55-.45-1-1-1h-1zm-2 3v4.81c0 .11.08.19.19.19h4.63c.11 0 .19-.08.19-.19v-4.81h-1v3.5c0 .28-.22.5-.5.5s-.5-.22-.5-.5v-3.5h-1v3.5c0 .28-.22.5-.5.5s-.5-.22-.5-.5v-3.5h-1z' /></svg>";

pub fn symbol_to_string(symbol: Symbol) -> String {
    TRASH.to_string()
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Svg {
    pub symbol: Symbol,
}

impl Component for Svg {
    type Message = ();
    type Properties = Svg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl<T: Component> Renderable<T> for Svg {
    fn view(&self) -> Html<T> {
        match Node::from_html(TRASH) {
            Ok(node) => VNode::VRef(node),
            Err(error) => html! { { error }},
        }
    }
}
