use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(PartialEq, Clone, Debug)]
pub enum SvgSymbol {
    Trash,
    ScatterFull,
    PiggyBank,
    Megaphone,
    Map,
    Checklist,
    Exit,
    Gear,
    Head,
    CheckCircle,
    Warning,
    Github,
    Telegram,
    Twitter,
}

impl Default for SvgSymbol {
    fn default() -> SvgSymbol {
        SvgSymbol::Trash
    }
}

// const TRASH: &str = "<svg width='8' height='8' viewBox='0 0 8 8'>  <path d='M3 0c-.55 0-1 .45-1 1h-1c-.55 0-1 .45-1 1h7c0-.55-.45-1-1-1h-1c0-.55-.45-1-1-1h-1zm-2 3v4.81c0 .11.08.19.19.19h4.63c.11 0 .19-.08.19-.19v-4.81h-1v3.5c0 .28-.22.5-.5.5s-.5-.22-.5-.5v-3.5h-1v3.5c0 .28-.22.5-.5.5s-.5-.22-.5-.5v-3.5h-1z' /></svg>";
const TRASH: &str = include_str!("../../static/svg/trash.svg");
const SCATTER_FULL: &str = include_str!("../../static/svg/scatter-large.svg");
const PIGGY_BANK: &str = include_str!("../../static/svg/piggy-bank.svg");
const MEGAPHONE: &str = include_str!("../../static/svg/megaphone.svg");
const MAP: &str = include_str!("../../static/svg/map2.svg");
const CHECKLIST: &str = include_str!("../../static/svg/checklist.svg");
const EXIT: &str = include_str!("../../static/svg/exit.svg");
const GEAR: &str = include_str!("../../static/svg/gear2.svg");
const HEAD: &str = include_str!("../../static/svg/head.svg");
const CHECK_CIRCLE: &str = include_str!("../../static/svg/check-circle.svg");
const WARNING: &str = include_str!("../../static/svg/warning.svg");
const GITHUB: &str = include_str!("../../static/svg/github.svg");
const TELEGRAM: &str = include_str!("../../static/svg/telegram.svg");
const TWITTER: &str = include_str!("../../static/svg/twitter.svg");

impl SvgSymbol {
    fn as_str(&self) -> &str {
        match &self {
            SvgSymbol::Trash => TRASH,
            SvgSymbol::ScatterFull => SCATTER_FULL,
            SvgSymbol::PiggyBank => PIGGY_BANK,
            SvgSymbol::Megaphone => MEGAPHONE,
            SvgSymbol::Map => MAP,
            SvgSymbol::Checklist => CHECKLIST,
            SvgSymbol::Exit => EXIT,
            SvgSymbol::Gear => GEAR,
            SvgSymbol::Head => HEAD,
            SvgSymbol::CheckCircle => CHECK_CIRCLE,
            SvgSymbol::Warning => WARNING,
            SvgSymbol::Github => GITHUB,
            SvgSymbol::Telegram => TELEGRAM,
            SvgSymbol::Twitter => TWITTER,
        }
    }
}

impl ToString for SvgSymbol {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Svg {
    pub symbol: SvgSymbol,
}

impl Component for Svg {
    type Message = ();
    type Properties = Svg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl<T: Component> Renderable<T> for Svg {
    fn view(&self) -> Html<T> {
        let html = self.symbol.as_str();
        let html = html.replace('\n', "");
        match Node::from_html(html.as_str()) {
            Ok(node) => VNode::VRef(node),
            Err(error) => html! { { error }},
        }
    }
}
