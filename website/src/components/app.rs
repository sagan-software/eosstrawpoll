use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
// use contract::Contract;
use pages::{HomePage, PopularPollsPage};
use route::{Route, RouteError};
// use services::eos::{self, EosConfig, EosService};
use stdweb::traits::IEvent;
use yew::prelude::*;

const DEVNET: &str = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f";

const EOS_MAINNET: &str = "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906";

const TELOS_TESTNET: &str = "9e46127b78e0a7f6906f549bba3d23b264c70ee6ec781aed9d4f1b72732f34fc";

pub struct App {
    route: Option<Result<Route, RouteError>>,
    router: Box<Bridge<RouterAgent<()>>>,
    scatter: Box<Bridge<ScatterAgent>>,
    context: Context,
    link: ComponentLink<App>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
}

pub enum Msg {
    Router(RouterOutput<()>),
    Scatter(ScatterOutput),
    Ignore,
    NavigateTo(Route),
    Login,
    Logout,
    // GotInfo(Result<eos::Info, String>),
    // GotContract(Result<Contract, String>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let mut router = RouterAgent::bridge(callback);
        router.send(RouterInput::GetCurrentRoute);

        let callback = link.send_back(Msg::Scatter);
        let mut scatter = ScatterAgent::bridge(callback);
        scatter.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        // let eos = EosService::new(EosConfig {
        //     chain_id: Some(DEVNET.to_string()),
        //     key_provider: None,
        //     http_endpoint: Some("http://localhost:8888".to_string()),
        //     expire_in_seconds: None,
        //     broadcast: None,
        //     verbose: None,
        //     debug: None,
        //     sign: None,
        // });

        // {
        //     let callback = link.send_back(Msg::GotInfo);
        //     eos.get_info(callback);
        // }

        let mut context = Context::default();
        // context.eos = Some(Box::new(eos));

        App {
            route: None,
            router,
            scatter,
            context,
            link,
            scatter_connected: None,
            scatter_identity: None,
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
            Msg::Router(output) => {
                self.route = Some(Route::from_string(output.pathname));
                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    info!("got identity {:#?}", result);
                    self.scatter_identity = Some(result);
                    true
                }
                ScatterOutput::ForgotIdentity(_result) => {
                    self.scatter_identity = None;
                    true
                }
                ScatterOutput::Connected(result) => {
                    self.scatter_connected = Some(result);
                    true
                }
            },
            Msg::Login => {
                let chain_id = DEVNET.to_string();
                let network = scatter::Network {
                    chain_id: Some(chain_id),
                    protocol: None,
                    blockchain: None,
                    host: None,
                    port: None,
                };
                let required_fields = scatter::RequiredFields {
                    accounts: Some(vec![network]),
                };
                let scatter_msg = ScatterInput::GetIdentity(required_fields);
                self.scatter.send(scatter_msg);
                false
            }
            Msg::Logout => {
                self.scatter.send(ScatterInput::ForgetIdentity);
                false
            } // Msg::GotInfo(result) => {
              //     info!("got info {:#?}", result);
              //     true
              // }
              // Msg::GotContract(result) => {
              //     info!("got contract {:#?}", result);
              //     match result {
              //         Ok(contract) => {
              //             self.context.endpoint = self.context.endpoint.clone(); // do this to force updating props
              //             self.context.contract = Some(Box::new(contract));
              //             true
              //         }
              //         Err(error) => {
              //             error!("error getting contract: {:#?}", error);
              //             false
              //         }
              //     }
              // }
        }
    }
}

impl Renderable<App> for App {
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

impl App {
    fn view_header(&self) -> Html<Self> {
        html! {
            <header class="app__header", >
                <div class="app__container", >
                    <a
                        class="app__logo",
                        href="/",
                        onclick=|e| {
                            e.prevent_default();
                            Msg::NavigateTo(Route::Home)
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
                { self.view_nav_link(Route::PopularPolls, "Popular") }
                { self.view_nav_link(Route::NewPolls, "New") }
                { self.view_nav_link(Route::Donors, "Donors") }
                // <button onclick=|_| Msg::NavigateTo(Page::Home),>{ "Go to Home" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::Profile("balls".into())),>{ "Go to Profile" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::Poll("balls".into(), "balls".into())),>{ "Go to Poll" }</button>
                // <button onclick=|_| Msg::NavigateTo(Page::PollResults("balls".into(), "balls".into())),>{ "Go to Poll Results" }</button>
            </nav>
        }
    }

    fn view_nav_link(&self, route: Route, text: &str) -> Html<Self> {
        html! {
            <a
            class="app__link",
            href=route.to_string(),
            onclick=|e| {
                e.prevent_default();
                Msg::NavigateTo(route.clone())
            },
            >
                { text }
            </a>
        }
    }

    fn view_user(&self) -> Html<Self> {
        let view = match &self.scatter_identity {
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

    fn view_user_ok(&self, identity: &scatter::Identity) -> Html<Self> {
        let account_name = identity.account_name().unwrap_or("Anon".to_string());
        let profile_route = Route::Profile(account_name.clone());
        html! {
            <p>
                { "Logged in as" }
                <a
                    href=profile_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(profile_route.clone())
                    },
                >
                    { account_name }
                </a>
            </p>
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

    fn view_page(&self) -> Html<App> {
        info!("RENDERING PAGE");
        match &self.route {
            Some(result) => match result {
                Ok(route) => match route {
                    Route::Home => html! {
                        <HomePage: context=&self.context, />
                    },
                    Route::PopularPolls => html! {
                        <PopularPollsPage: context=&self.context, />
                    },
                    Route::NewPolls => html! {
                        <>
                            { "New polls"}
                        </>
                    },
                    Route::Donors => html! {
                        <>
                            { "Donors"}
                        </>
                    },
                    Route::Profile(ref account) => html! {
                        <>
                            {format!("Profile page {}", account)}
                        </>
                    },
                    Route::Poll(ref poll_creator, ref poll_name) => html! {
                        <>
                            {format!("Poll page: '{}' '{}'", poll_creator, poll_name)}
                        </>
                    },
                    Route::PollResults(ref poll_creator, ref poll_name) => html! {
                        <>
                            {format!("Poll results page: '{}' '{}'", poll_creator, poll_name)}
                        </>
                    },
                },
                Err(error) => match error {
                    RouteError::NotFound(url) => html! {
                        <>
                            { format!("404 not found: {}", url)}
                        </>
                    },
                    RouteError::SecurityError(_error) => html! {
                        <>
                            { "something bad happened"}
                        </>
                    },
                },
            },
            None => html! {
                <>
                    { "loading..."}
                </>
            },
        }
    }
}
