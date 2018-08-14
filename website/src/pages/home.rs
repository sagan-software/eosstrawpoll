use agents::router::{RouterAgent, RouterOutput};
// use route::Route;
use components::PollForm;
use context::Context;
use yew::prelude::*;

pub struct HomePage {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
}

pub enum Msg {
    Router(RouterOutput<()>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let mut router = RouterAgent::bridge(callback);
        HomePage {
            router,
            context: props.context,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        true
    }
}

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="home_page", >
                <header class="page__header app__container", >
                    <h1 class="page__title", >
                        { "Create real-time polls on EOS blockchains" }
                    </h1>
                </header>
                <div class="app__container", >
                    <PollForm: context=&self.context, />
                </div>
                <div class="app__container", >
                    <div class="popular_polls", >
                        <h2> { "Popular Polls" } </h2>
                        <ul>
                            <li>
                                <a href="#", >{ "Poll title" }</a>
                            </li>
                        </ul>
                    </div>
                    <div class="recent_polls", >
                        <h2> { "Recent Polls" } </h2>
                        <ul>
                            <li>
                                <a href="#", >{ "Poll title" }</a>
                            </li>
                        </ul>
                    </div>
                    <div class="recent_votes", >
                        <h2> { "Recent Votes" } </h2>
                        <ul>
                            <li>
                                <a href="#", >{ "Poll title" }</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}
