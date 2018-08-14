use agents::router::{RouterAgent, RouterInput};
use context::Context;
use route::Route;
use stdweb::traits::IEvent;
use yew::prelude::*;

pub struct PopularPollsPage {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

pub enum Msg {
    Ignore,
    NavigateTo(Route),
}

impl Component for PopularPollsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::Ignore);
        let router = RouterAgent::bridge(callback);
        PopularPollsPage {
            router,
            context: props.context,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PopularPollsPage> for PopularPollsPage {
    fn view(&self) -> Html<Self> {
        let test_route = Route::Poll("alice".into(), "balls".into());
        html! {
            <div>
                <h1>{ "Popular Polls" }</h1>
                <a
                    href=test_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(test_route.clone())
                    },
                >
                    { "Go to poll" }
                </a>
            </div>
        }
    }
}
