use prelude::*;
use router::{RouterAgent, RouterInput, RouterOutput};
use stdweb::traits::IEvent;

pub struct Link {
    router: Box<Bridge<RouterAgent>>,
    route: Route,
    text: String,
    class: String,
}

pub enum Msg {
    Router(RouterOutput),
    NavigateTo,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub route: Route,
    pub text: String,
    pub class: String,
}

impl Component for Link {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router = RouterAgent::bridge(link.send_back(Msg::Router));
        Link {
            router,
            route: props.route,
            text: props.text,
            class: props.class,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
            Msg::NavigateTo => {
                let url = self.route.to_string();
                self.router.send(RouterInput::ChangeRoute(url));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.route = props.route;
        self.text = props.text;
        self.class = props.class;
        true
    }
}

impl Renderable<Link> for Link {
    fn view(&self) -> Html<Self> {
        html! {
            <a class=format!("internal_link {}", &self.class),
                href=self.route.to_string(),
                onclick=|e| {
                    e.prevent_default();
                    Msg::NavigateTo
                },
            >
                { &self.text }
            </a>
        }
    }
}
