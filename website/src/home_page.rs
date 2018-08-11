use context::Context;
use poll_form::PollForm;
use router;
use router::Route;
use yew::prelude::*;

pub struct HomePage {
    sub_path: Option<String>,
    router: Box<Bridge<router::Router<()>>>,
    context: Box<Context>,
}

pub enum Msg {
    Navigate(Vec<Msg>), // Navigate after performing other actions
    HandleRoute(Route<()>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Box<Context>,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        HomePage {
            sub_path: None,
            router,
            context: props.context,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(msgs) => {
                // Perform the wrapped action first
                for msg in msgs {
                    self.update(msg);
                }

                // The path dictating that this component be instantiated must be provided
                let mut path_segments = vec!["b".into()];
                if let Some(ref sub_path) = self.sub_path {
                    path_segments.push(sub_path.clone())
                }

                let route = router::Route {
                    path_segments,
                    query: None,
                    fragment: None,
                    state: (),
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.sub_path = route.path_segments.get(1).map(String::clone);
                true
            }
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
