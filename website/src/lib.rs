#![recursion_limit = "500"]
#![warn(clippy)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate yew;
extern crate http;
#[macro_use]
extern crate stdweb;
extern crate rand;

mod agents;
mod context;
mod contract;
mod global_config;
mod home_page;
mod poll_form;
mod services;
mod svg;

use agents::router::{self, Route};
use context::Context;
use contract::Contract;
use home_page::HomePage;
use services::eos::{self, EosConfig, EosService};
use services::scatter::{self, Identity, ScatterError, ScatterService};
use stdweb::traits::IEvent;
use yew::prelude::*;

pub enum Page {
    Loading,
    Home,
    Profile(String),
    Poll(String, String),
    PollResults(String, String),
    NotFound(String),
}

const DEVNET: &str = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f";

const EOS_MAINNET: &str = "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906";

const TELOS_TESTNET: &str = "9e46127b78e0a7f6906f549bba3d23b264c70ee6ec781aed9d4f1b72732f34fc";

pub struct Model {
    page: Page,
    router: Box<Bridge<agents::router::Router<()>>>,
    context: Context,
    link: ComponentLink<Model>,
}

pub enum Msg {
    Ignore,
    NavigateTo(Page),
    HandleRoute(Route<()>),
    Login,
    Logout,
    GotIdentity(Result<Identity, ScatterError>),
    ScatterConnected(Option<ScatterService>),
    GotInfo(Result<eos::Info, String>),
    GotContract(Result<Contract, String>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::HandleRoute);
        let mut router = router::Router::bridge(callback);
        router.send(router::Request::GetCurrentRoute);
        {
            let callback = link.send_back(Msg::ScatterConnected);
            ScatterService::connect("eosstrawpoll", callback);
        }

        let eos = EosService::new(EosConfig {
            chain_id: Some(DEVNET.to_string()),
            key_provider: None,
            http_endpoint: Some("http://localhost:8888".to_string()),
            expire_in_seconds: None,
            broadcast: None,
            verbose: None,
            debug: None,
            sign: None,
        });

        {
            let callback = link.send_back(Msg::GotInfo);
            eos.get_info(callback);
        }

        let mut context = Context::default();
        context.eos = Some(Box::new(eos));

        Model {
            page: Page::Loading,
            router,
            context,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
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
                info!("Routing: {:#?}", route.path_segments);
                let path_segments = route
                    .path_segments
                    .into_iter()
                    .filter(|ref i| !i.is_empty())
                    .collect::<Vec<_>>();
                self.page = match &path_segments[..] {
                    [] => Page::Home,
                    [account] => Page::Profile(account.to_string()),
                    [poll_creator, poll_name] => {
                        Page::Poll(poll_creator.to_string(), poll_name.to_string())
                    }
                    [poll_creator, poll_name, results] => {
                        if results == "results" {
                            Page::PollResults(poll_creator.to_string(), poll_name.to_string())
                        } else {
                            Page::NotFound("".into())
                        }
                    }
                    _ => Page::NotFound("".into()),
                };

                true
            }
            Msg::Login => match self.context.scatter {
                Some(ref scatter) => {
                    info!("attempting to login");
                    let callback = self.link.send_back(Msg::GotIdentity);
                    let chain_id = DEVNET.to_string();
                    scatter.get_identity_for_chain(chain_id, callback);
                    true
                }
                None => {
                    info!("no scatter");
                    false
                }
            },
            Msg::Logout => match self.context.scatter {
                Some(ref scatter) => {
                    let callback = self.link.send_back(|_| Msg::Ignore);
                    scatter.forget_identity(callback);
                    self.context.identity = None;
                    true
                }
                None => false,
            },
            Msg::GotIdentity(result) => {
                info!("got identity {:#?}", result);
                self.context.identity = Some(result);

                let scatter = self.context.scatter.as_ref().unwrap();
                let mut eos_config = EosConfig::default();
                eos_config.chain_id = Some(DEVNET.to_string());
                let eos = scatter.eos(
                    scatter::Network {
                        chain_id: Some(DEVNET.to_string()),
                        protocol: Some("http".to_string()),
                        blockchain: Some("eos".to_string()),
                        host: Some("localhost".to_string()),
                        port: Some(8888),
                    },
                    eos_config,
                );
                let callback = self.link.send_back(Msg::GotContract);
                Contract::from_eos(
                    eos,
                    scatter::RequiredFields {
                        accounts: Some(vec![scatter::Network {
                            chain_id: Some(DEVNET.to_string()),
                            protocol: Some("http".to_string()),
                            blockchain: Some("eos".to_string()),
                            host: Some("localhost".to_string()),
                            port: Some(8888),
                        }]),
                    },
                    callback,
                );
                true
            }
            Msg::ScatterConnected(scatter) => match scatter {
                Some(scatter) => {
                    info!("connected to scatter: {:#?}", scatter);
                    if let Some(identity) = scatter.identity() {
                        self.context.identity = Some(Ok(identity));
                    }
                    self.context.scatter = Some(Box::new(scatter));
                    true
                }
                None => {
                    warn!("couldn't connect to scatter");
                    false
                }
            },
            Msg::GotInfo(result) => {
                info!("got info {:#?}", result);
                true
            }
            Msg::GotContract(result) => {
                info!("got contract {:#?}", result);
                match result {
                    Ok(contract) => {
                        self.context.endpoint = self.context.endpoint.clone(); // do this to force updating props
                        self.context.contract = Some(Box::new(contract));
                        true
                    }
                    Err(error) => {
                        error!("error getting contract: {:#?}", error);
                        false
                    }
                }
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="app", >
                { self.view_header() }
                { self.view_content() }
                { self.view_footer() }
            </div>
        }
    }
}

impl Model {
    fn view_header(&self) -> Html<Self> {
        html! {
            <header class="app__header", >
                <div class="app__container", >
                    <a
                        class="app__logo",
                        href="/",
                        onclick=|e| {
                            e.prevent_default();
                            Msg::NavigateTo(Page::Home)
                        },
                    >
                        { "EOS Straw Poll" }
                    </a>
                    { self.view_nav() }
                    { self.view_user() }
                </div>
            </header>
        }
    }

    fn view_nav(&self) -> Html<Self> {
        html! {
            <nav class="app__nav", >
                { self.view_nav_link("Popular") }
                { self.view_nav_link("Recent") }
                // <button onclick=|_| Msg::NavigateTo(Page::Home),>{ "Go to Home" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::Profile("balls".into())),>{ "Go to Profile" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::Poll("balls".into(), "balls".into())),>{ "Go to Poll" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::PollResults("balls".into(), "balls".into())),>{ "Go to Poll Results" }</button>
            </nav>
        }
    }

    fn view_nav_link(&self, text: &str) -> Html<Self> {
        html! {
            <a class="app__link", href="/", >
                { text }
            </a>
        }
    }

    fn view_user(&self) -> Html<Self> {
        let view = match &self.context.identity {
            None => self.view_user_none(),
            Some(Ok(identity)) => self.view_user_ok(identity),
            Some(Err(error)) => self.view_user_err(error),
        };
        html! {
            <nav class="app__user", >
                { view }
            </nav>
        }
    }

    fn view_user_none(&self) -> Html<Self> {
        html! {
            <button
                class="app__login",
                onclick=|_| Msg::Login,
            >
                { "Login" }
            </button>
        }
    }

    fn view_user_ok(&self, _identity: &Identity) -> Html<Self> {
        let account_name = self.context.account_name().unwrap_or("Anon".to_string());
        html! {
            <p>{ "Logged in as " }{ account_name }</p>
            <button
                class="app__logout",
                onclick=|_| Msg::Logout,
            >
                { "Logout" }
            </button>
        }
    }

    fn view_user_err(&self, _error: &ScatterError) -> Html<Self> {
        html! {
            <button
                class="app__login",
                onclick=|_| Msg::Login,
            >
                { "Login (error)" }
            </button>
        }
    }

    fn view_content(&self) -> Html<Self> {
        html! {
            <div class="app__content", >
                {self.view_page()}
            </div>
        }
    }

    fn view_footer(&self) -> Html<Self> {
        html!{
            <footer class="app__footer", >
                <div class="app__container", >
                    { "Footer" }
                </div>
            </footer>
        }
    }

    fn view_page(&self) -> Html<Model> {
        info!("RENDERING PAGE");
        match self.page {
            Page::Home => html! {
                <HomePage: context=&self.context, />
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
            Page::PollResults(ref poll_creator, ref poll_name) => html! {
                <>
                    {format!("Poll results page: '{}' '{}'", poll_creator, poll_name)}
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
