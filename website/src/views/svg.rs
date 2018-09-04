use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

fn svg<T: Component>(contents: &str) -> Html<T> {
    let html = contents.replace('\n', "");
    match Node::from_html(html.as_str()) {
        Ok(node) => VNode::VRef(node),
        Err(error) => html! { { error }},
    }
}

macro_rules! svg {
    ($func_name:ident) => {
        pub fn $func_name<T: Component>() -> Html<T> {
            svg(include_str!(concat!(
                "../../static/svg/",
                stringify!($func_name),
                ".svg"
            )))
        }
    };
}

svg!(twitter);
svg!(trash);
svg!(scatter_large);
svg!(piggy_bank);
svg!(megaphone);
svg!(roadmap);
svg!(checklist);
svg!(logout);
svg!(gear);
svg!(head);
svg!(check_circle);
svg!(warning);
svg!(github);
svg!(telegram);
svg!(plug_cross);
svg!(link_cross);
svg!(eos);
svg!(chevron_down);
