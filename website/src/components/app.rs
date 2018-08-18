use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
use pages::*;
use route::{Route, RouteError};
use stdweb::traits::IEvent;
use yew::prelude::*;

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

        App {
            route: None,
            router,
            scatter,
            context: Context::default(),
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
                self.route = Some(output.pathname.parse());
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
                ScatterOutput::PushedActions(_result) => false,
            },
            Msg::Login => {
                let required_fields = self.context.required_fields();
                let scatter_msg = ScatterInput::GetIdentity(required_fields);
                self.scatter.send(scatter_msg);
                false
            }
            Msg::Logout => {
                self.scatter.send(ScatterInput::ForgetIdentity);
                false
            }
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
            <header class="app_header", >
                <div class="app_container", >
                    <a class="app_logo",
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
            <>
                <nav class="app_nav -primary", >
                    { self.view_nav_link(Route::PopularPolls, "Popular") }
                    { self.view_nav_link(Route::NewPolls, "New") }
                    { self.view_nav_link(Route::Donors, "Donors") }
                </nav>
                <nav class="app_nav -secondary", >
                    { self.view_nav_link(Route::Home, "EOS MainNet") }
                </nav>
            </>
        }
    }

    fn view_nav_link(&self, route: Route, text: &str) -> Html<Self> {
        html! {
            <a
                class="app_link",
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
            <nav class="app_user", >
                { view }
            </nav>
        }
    }

    fn view_user_none(&self) -> Html<Self> {
        html! {
            <button
                class="app_login",
                onclick=|_| Msg::Login,
            >
                { "Login with Scatter" }
            </button>
        }
    }

    fn view_user_ok(&self, identity: &scatter::Identity) -> Html<Self> {
        let account_name = identity
            .account_name()
            .unwrap_or_else(|| "Anon".to_string());
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
                class="app_logout",
                onclick=|_| Msg::Logout,
            >
                { "Logout" }
            </button>
        }
    }

    fn view_user_err(&self, _error: &ScatterError) -> Html<Self> {
        html! {
            <button
                class="app_login",
                onclick=|_| Msg::Login,
            >
                { "Login (error)" }
            </button>
        }
    }

    fn view_content(&self) -> Html<Self> {
        html! {
            <div class="app_content", >
                {self.view_page()}
            </div>
        }
    }

    fn view_footer(&self) -> Html<Self> {
        html!{
            <footer class="app_footer", >
                <div class="app_container", >
                    <p>
                        <a href="#", >{ "Github" }</a>
                        <a href="#", >{ "Twitter" }</a>
                        <a href="#", >{ "Telegram" }</a>
                        <a href="#", >{ "Steem" }</a>
                    </p>
                </div>
            </footer>
        }
    }

    fn view_page(&self) -> Html<App> {
        debug!("RENDERING PAGE");
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
                        <NewPollsPage: context=&self.context, />
                    },
                    Route::Donors => html! {
                        <>
                            { "Donors"}
                        </>
                    },
                    Route::Profile(ref account) => html! {
                        <ProfilePage: context=&self.context, account=account, />
                    },
                    Route::Poll(ref creator, ref slug) => html! {
                        <PollPage: context=&self.context, creator=creator, slug=slug, />
                    },
                    Route::PollResults(ref creator, ref slug) => html! {
                        <PollResultsPage: context=&self.context, creator=creator, slug=slug, />
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
