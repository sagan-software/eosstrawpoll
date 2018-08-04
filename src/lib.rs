#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate yew;
extern crate stdweb;

mod home_page;
mod router;
mod routing;

use home_page::HomePage;
use router::Route;
use yew::prelude::*;

pub enum Page {
    Loading,
    Home,
    Profile(String),
    Poll(String, String),
    PollResults(String, String),
    NotFound(String),
}

pub struct Model {
    page: Page,
    router: Box<Bridge<router::Router<()>>>,
}

pub enum Msg {
    NavigateTo(Page),
    HandleRoute(Route<()>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        Model {
            page: Page::Loading,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(page) => {
                let path_segments = match page {
                    Page::Loading => vec![],
                    Page::Home => vec![],
                    Page::Profile(account) => vec![account],
                    Page::Poll(poll_creator, poll_name) => vec![poll_creator, poll_name],
                    Page::PollResults(poll_creator, poll_name) => {
                        vec![poll_creator, poll_name, "results".to_string()]
                    }
                    Page::NotFound(_) => vec!["404".into()],
                };

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
                info!("Routing: {}", route);
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.page = match route.path_segments.as_slice() {
                    [] => Page::Home,
                    [account] => Page::Profile(account.to_string()),
                    [poll_creator, poll_name] => {
                        Page::Poll(poll_creator.to_string(), poll_name.to_string())
                    }
                    // [poll_creator, poll_name, "results"] => {
                    //     Page::PollResults(poll_creator.to_string(), poll_name.to_string())
                    // }
                    other => Page::NotFound("".into()),
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::NavigateTo(Page::Home),>{ "Go to Home" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Profile("balls".into())),>{ "Go to Profile" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Poll("balls".into(), "balls".into())),>{ "Go to Poll" }</button>
                </nav>
                <div>
                    {self.page.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for Page {
    fn view(&self) -> Html<Model> {
        match *self {
            Page::Home => html! {
                <>
                    {"Home page"}
                    <HomePage: />
                </>
            },
            Page::Profile(ref account) => html! {
                <>
                    {format!("Profile page {}", account)}
                </>
            },
            Page::Poll(ref poll_creator, ref poll_name) => html! {
                <>
                    {format!("Poll page: '{}' '{}'", poll_creator, poll_name)}
                </>
            },
            _ => html! {
                <>
                    {format!("Invalid path: '{}'", "")}
                </>
            },
        }
    }
}
