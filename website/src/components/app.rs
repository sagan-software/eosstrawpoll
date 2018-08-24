use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use components::*;
use context::Context;
use route::{Route, RouteError};
use stdweb::traits::IEvent;
use yew::prelude::*;

pub struct App {
    route: Option<Result<Route, RouteError>>,
    router: Box<Bridge<RouterAgent<()>>>,
    scatter: Box<Bridge<ScatterAgent>>,
    context: Context,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
}

pub enum Msg {
    Router(RouterOutput<()>),
    Scatter(ScatterOutput),
    NavigateTo(Route),
    Login,
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let mut router = RouterAgent::bridge(callback);
        router.send(RouterInput::GetCurrentRoute);
        let scatter = ScatterAgent::new("eosstrawpoll".into(), 10000, link.send_back(Msg::Scatter));

        App {
            route: None,
            router,
            scatter,
            context: Context::default(),
            scatter_connected: None,
            scatter_identity: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                        <span class="app_version", >
                            { "PRE-ALPHA" }
                        </span>
                    </a>
                    { self.view_nav() }
                    { self.view_user() }
                </div>
            </header>
        }
    }

    fn view_nav(&self) -> Html<Self> {
        html! {
            <nav class="app_nav", >
                { self.view_nav_link(Route::Polls, "Polls", "polls", Symbol::Checklist) }
                { self.view_nav_link(Route::Donations, "Donations", "donations", Symbol::PiggyBank) }
                <a class="app_link app_link_roadmap",
                    href="https://github.com/sagan-software/eosstrawpoll/projects/1",
                    target="_blank",
                >
                    <Svg: symbol=Symbol::Map, />
                    { "Roadmap" }
                </a>
                <a class="app_link app_link_feedback",
                    href="https://eos-forum.org/#/e/eosstrawpoll",
                    target="_blank",
                >
                    <Svg: symbol=Symbol::Megaphone, />
                    { "Feedback" }
                </a>
            </nav>
        }
    }

    fn view_nav_link(&self, route: Route, text: &str, class: &str, symbol: Symbol) -> Html<Self> {
        html! {
            <a class=format!("app_link app_link_{}", class),
                href=route.to_string(),
                onclick=|e| {
                    e.prevent_default();
                    Msg::NavigateTo(route.clone())
                },
            >
                <Svg: symbol=symbol, />
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
                { "Login with " }
                <Svg: symbol=Symbol::ScatterFull, />
            </button>
        }
    }

    fn view_user_ok(&self, identity: &scatter::Identity) -> Html<Self> {
        let account_name = identity
            .account_name()
            .unwrap_or_else(|| "Anon".to_string());
        let profile_route = Route::Profile(account_name.clone());
        html! {
            <div class="app_user_actions", >
                <a
                    class="app_user_account",
                    href=profile_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(profile_route.clone())
                    },
                >
                    <Svg: symbol=Symbol::Head, />
                    { account_name }
                </a>
                <a class="app_user_settings", href="/settings",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(Route::Home)
                    },
                >
                    <Svg: symbol=Symbol::Gear, />
                </a>
                <button class="app_user_logout",
                    onclick=|_| Msg::Logout,
                >
                    <Svg: symbol=Symbol::Exit, />
                </button>
            </div>
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
        html!{
            <div class="app_content", >
                { self.view_page() }
            </div>
        }
    }

    fn view_footer(&self) -> Html<Self> {
        html!{
            <footer class="app_footer", >
                <div class="app_container", >
                    <p class="app_footer_text", >
                        { "Created by " }
                        <a href="//www.sagan.software", >{ "sagan.software" }</a>
                        { " © 2018" }
                    </p>
                    <p class="app_footer_links", >
                        <a href="//www.github.com/sagan-software/eosstrawpoll", >{ "Github" }</a>
                        <a href="//www.twitter.com/SaganSoftware", >{ "Twitter" }</a>
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
                    Route::Polls => html! {
                        <PollsPage: context=&self.context, />
                    },
                    Route::Donations => html! {
                        <DonationsPage: context=&self.context, />
                    },
                    Route::Profile(ref account) => html! {
                        <ProfilePage: context=&self.context, account=account, />
                    },
                    Route::Poll(ref creator, ref slug) => html! {
                        <PollPage: context=&self.context, creator=creator, slug=slug, show_results=false, />
                    },
                    Route::PollResults(ref creator, ref slug) => html! {
                        <PollPage: context=&self.context, creator=creator, slug=slug, show_results=true, />
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
