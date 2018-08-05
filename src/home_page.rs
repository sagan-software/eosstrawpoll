use poll_form::PollForm;
use router;
use router::Route;
use yew::prelude::*;

pub struct HomePage {
    sub_path: Option<String>,
    router: Box<Bridge<router::Router<()>>>,
}

pub enum Msg {
    Navigate(Vec<Msg>), // Navigate after performing other actions
    HandleRoute(Route<()>),
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        HomePage {
            sub_path: None,
            router,
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Apparently change MUST be implemented in this case, even though no props were changed
        true
    }
}

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Home" }</h1>
                <PollForm: />
            </div>
        }
    }
}
