use components::*;
use pages::*;
use prelude::*;
use router::{RouterAgent, RouterInput, RouterOutput};
use scatter::*;
use stdweb::traits::IEvent;
use views::svg;

pub struct App {
    route: Option<Result<Route, RouteError>>,
    router: Box<Bridge<RouterAgent>>,
    scatter: Box<Bridge<ScatterAgent>>,
    context: Context,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<ScatterIdentity, ScatterError>>,
}

pub enum Msg {
    Ignore,
    Router(RouterOutput),
    Scatter(ScatterOutput),
    NavigateTo(Route),
    Login,
    Logout,
    SelectChain(Chain),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
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
                self.router.send(RouterInput::ChangeRoute(url));
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
                ScatterOutput::PushedTransaction(_result) => false,
            },
            Msg::Login => {
                let required_fields = self.context.selected_chain.to_scatter_required_fields();
                let scatter_msg = ScatterInput::GetIdentity(required_fields);
                self.scatter.send(scatter_msg);
                false
            }
            Msg::Logout => {
                self.scatter.send(ScatterInput::ForgetIdentity);
                false
            }
            Msg::SelectChain(chain) => {
                if chain != self.context.selected_chain {
                    self.scatter.send(ScatterInput::ForgetIdentity);
                    self.context.selected_chain = chain;
                    true
                } else {
                    false
                }
            }
            Msg::Ignore => false,
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

fn app_logo() -> Html<App> {
    html! {
        <a class="app_logo",
            href="/",
            onclick=|e| {
                e.prevent_default();
                Msg::NavigateTo(Route::default())
            },
        >
            { "EOS Straw Poll" }
            <span class="app_version", >
                { "PRE-ALPHA" }
            </span>
        </a>
    }
}

impl App {
    fn view_header(&self) -> Html<Self> {
        html! {
            <header class="app_header", >
                <div class="app_container", >
                    { app_logo() }
                    { self.view_nav() }
                    { self.view_chains_dropdown() }
                    { self.view_user() }
                </div>
            </header>
        }
    }

    fn view_nav(&self) -> Html<Self> {
        html! {
            <nav class="app_nav", >
                <a class="app_link app_link_roadmap",
                    href="https://github.com/sagan-software/eosstrawpoll/projects/1",
                    target="_blank",
                >
                    { svg::roadmap() }
                    { "Roadmap" }
                </a>
                <a class="app_link app_link_feedback",
                    href="https://eos-forum.org/#/e/eosstrawpoll",
                    target="_blank",
                >
                    { svg::megaphone() }
                    { "Feedback" }
                </a>
            </nav>
        }
    }

    fn view_chains_dropdown(&self) -> Html<Self> {
        html! {
            <div class="chain_dropdown", >
                <div class="available_chains", >
                    { for self.context.available_chains.iter().map(|chain| {
                        self.view_chain_dropdown(chain.clone())
                    })}
                </div>
                <div class="selected_chain", >
                    { &self.context.selected_chain.long_name }
                    { svg::chevron_down() }
                </div>
            </div>
        }
    }

    fn view_chain_dropdown(&self, chain: Chain) -> Html<Self> {
        let c = chain.clone();
        let is_selected = chain == self.context.selected_chain;
        let selected_class = if is_selected { "-selected" } else { "" };
        html! {
            <div class=format!("available_chain {}", selected_class),
                onclick=|e| {
                    e.prevent_default();
                    Msg::SelectChain(chain.clone())
                },
            >
                { &c.long_name }
            </div>
        }
    }

    fn view_user(&self) -> Html<Self> {
        let view = match (&self.scatter_connected, &self.scatter_identity) {
            (None, _) => self.view_user_loading(),
            (Some(Err(_)), _) => self.view_user_install_scatter(),
            (_, None) => self.view_user_none(),
            (_, Some(Ok(identity))) => self.view_user_ok(identity),
            (_, Some(Err(error))) => self.view_user_err(error),
        };
        html! {
            <nav class="app_user", >
                { view }
            </nav>
        }
    }

    fn view_user_loading(&self) -> Html<Self> {
        html! {
            <span
                class="app_login btn btn-primary btn-lg -loading",
            >
                { "Loading..." }
            </span>
        }
    }

    fn view_user_install_scatter(&self) -> Html<Self> {
        html! {
            <a
                class="app_login btn btn-primary btn-lg -install",
                href="https://get-scatter.com",
            >
                { "Install " }
                { svg::scatter_large() }
            </a>
        }
    }

    fn view_user_none(&self) -> Html<Self> {
        html! {
            <button
                class="app_login btn btn-primary btn-lg",
                onclick=|_| Msg::Login,
            >
                { "Login with " }
                { svg::scatter_large() }
            </button>
        }
    }

    fn view_user_ok(&self, identity: &ScatterIdentity) -> Html<Self> {
        let account_name = identity
            .account_name()
            .unwrap_or_else(|| "Anon".to_string());
        let profile_route = Route::Profile(
            self.context.selected_chain.to_chain_id_prefix(),
            account_name.clone(),
        );
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
                    { svg::head() }
                    { account_name }
                </a>
                <button class="app_user_logout",
                    onclick=|_| Msg::Logout,
                >
                    { svg::logout() }
                </button>
            </div>
        }
    }

    fn view_user_err(&self, _error: &ScatterError) -> Html<Self> {
        html! {
            <button
                class="app_login btn btn-danger btn-lg",
                onclick=|_| Msg::Login,
            >
                { "Login with " }
                { svg::scatter_large() }
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
                        <a href="//www.github.com/sagan-software", >
                            { svg::github() }
                            { "Github" }
                        </a>
                        <a href="//www.twitter.com/SaganSoftware", >
                            { svg::twitter() }
                            { "Twitter" }
                        </a>
                        <a href="https://t.me/SaganSoftware", >
                            { svg::telegram() }
                            { "Telegram" }
                        </a>
                    </p>
                </div>
            </footer>
        }
    }

    fn view_unknown_chain(&self, chain_id_prefix: &ChainIdPrefix) -> Html<Self> {
        html! {
            <div class="page error_page", >
                <header class="page_header", >
                    <div class="app_container", >
                        <h1 class="page_title", >
                            { "Unknown Chain"}
                        </h1>
                    </div>
                </header>
                <main class="page_content", >
                    <div class="app_container", >
                        { svg::link_cross() }
                        <p>{ format!("Couldn't find an EOS blockchain with chain ID prefix '{}'", chain_id_prefix.to_string())}</p>
                    </div>
                </main>
            </div>
        }
    }

    fn view_page(&self) -> Html<App> {
        match &self.route {
            Some(result) => match result {
                Ok(route) => match route {
                    Route::Home(chain_id_prefix) => match chain_id_prefix {
                        Some(chain_id_prefix) => match self.context.find_chain(chain_id_prefix) {
                            Some(chain) => html! {
                                <HomePage:
                                    context=&self.context,
                                    chain=Some(chain),
                                />
                            },
                            None => self.view_unknown_chain(chain_id_prefix),
                        },
                        None => html! {
                            <HomePage:
                                context=&self.context,
                                chain=None,
                            />
                        },
                    },
                    Route::Profile(chain_id_prefix, ref account) => {
                        match self.context.find_chain(chain_id_prefix) {
                            Some(chain) => html! {
                                <ProfilePage:
                                    context=&self.context,
                                    account=account,
                                    chain=chain,
                                />
                            },
                            None => self.view_unknown_chain(chain_id_prefix),
                        }
                    }
                    Route::Poll(chain_id_prefix, ref creator, ref slug) => {
                        match self.context.find_chain(chain_id_prefix) {
                            Some(chain) => html! {
                                <PollVotingPage:
                                    context=&self.context,
                                    creator=creator,
                                    slug=slug,
                                    chain=chain,
                                />
                            },
                            None => self.view_unknown_chain(chain_id_prefix),
                        }
                    }
                    Route::PollResults(chain_id_prefix, ref creator, ref slug) => {
                        match self.context.find_chain(chain_id_prefix) {
                            Some(chain) => html! {
                                <PollResultsPage:
                                    context=&self.context,
                                    creator=creator,
                                    slug=slug,
                                    chain=chain,
                                />
                            },
                            None => self.view_unknown_chain(chain_id_prefix),
                        }
                    }
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
