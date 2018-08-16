use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
use route::Route;
use serde_json;
use std::cmp::{max, min};
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;
use stdweb::web::Date;
use types::*;
use yew::prelude::*;

pub struct PollForm {
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub use_whitelist: bool,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub min_num_choices: usize,
    pub max_num_choices: usize,
    pub open_time: u32,
    pub close_time: u32,
    pub submitting: bool,
    pub context: Context,
    pub global_config: GlobalConfig,
    pub active_bps: Vec<String>,
    pub standby_bps: Vec<String>,
    pub link: ComponentLink<PollForm>,
    pub scatter_agent: Box<Bridge<ScatterAgent>>,
    pub scatter_connected: Option<Result<(), ScatterError>>,
    pub scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pub pushed_poll: Option<Result<scatter::PushedTransaction, ScatterError>>,
    pub router: Box<Bridge<RouterAgent<()>>>,
}

pub enum Msg {
    NoOp,
    Submit,
    SetTitle(String),
    AddOption,
    SetOption(usize, String),
    DelOption(usize),
    SetWhitelist(String),
    SetOnlyBPs,
    SetOnlyActiveBPs,
    SetOnlyStandbyBPs,
    SetBlacklist(String),
    SetMinChoices(String),
    SetMaxChoices(String),
    SetOpenTime(String),
    SetCloseTime(String),
    Scatter(ScatterOutput),
    Router(RouterOutput<()>),
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Props {
    pub context: Context,
}

const EOS_MAINNET: &'static str =
    "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906";

const TELOS_TESTNET: &'static str =
    "9e46127b78e0a7f6906f549bba3d23b264c70ee6ec781aed9d4f1b72732f34fc";

impl Component for PollForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);
        let slug: String = js! {
            var text = "";
            var possible = "abcdefghijklmnopqrstuvwxyz12345";
            for (var i = 0; i < 12; i++) {
                text += possible.charAt(Math.floor(Math.random() * possible.length));
            }
            return text;
        }.try_into()
        .unwrap();

        PollForm {
            slug,
            title: "".to_string(),
            options: vec!["".to_string(), "".to_string()],
            use_whitelist: true,
            whitelist: vec![],
            blacklist: vec![],
            min_num_choices: 1,
            max_num_choices: 1,
            open_time: 0,
            close_time: 0,
            submitting: false,
            context: props.context,
            global_config: GlobalConfig::default(),
            active_bps: vec![
                "starteosiobp".to_string(),
                "eoscanadacom".to_string(),
                "eosnewyorkio".to_string(),
            ],
            standby_bps: vec![
                "eoshuobipool".to_string(),
                "zbeosbp11111".to_string(),
                "libertyblock".to_string(),
            ],
            link,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed_poll: None,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => false,
            Msg::NoOp => false,
            Msg::SetTitle(value) => {
                self.title = value;
                false
            }
            Msg::AddOption => {
                info!("add option");
                self.options.push("".to_string());
                true
            }
            Msg::SetOption(i, value) => {
                info!("setting option {} to {}", i, value);
                if i < self.options.len() {
                    self.options[i] = value;
                }
                false
            }
            Msg::DelOption(i) => {
                let options_len = self.options.len();
                if i < options_len && options_len > 2 {
                    self.options.remove(i);
                    let options_len = self.options.len();
                    self.max_num_choices = min(self.max_num_choices, options_len);
                    self.min_num_choices = min(self.max_num_choices, self.min_num_choices);
                    true
                } else {
                    false
                }
            }
            Msg::SetWhitelist(_value) => true,
            Msg::SetOnlyBPs => {
                self.whitelist = self.active_bps.clone();
                self.whitelist.append(&mut self.standby_bps.clone());
                true
            }
            Msg::SetOnlyActiveBPs => {
                self.whitelist = self.active_bps.clone();
                true
            }
            Msg::SetOnlyStandbyBPs => {
                self.whitelist = self.standby_bps.clone();
                true
            }
            Msg::SetBlacklist(_value) => true,
            Msg::SetMinChoices(value) => {
                let num = value.parse::<usize>();
                match num {
                    Ok(num) => {
                        let options_len = self.options.len();
                        self.min_num_choices = min(max(1, num), options_len);
                        self.max_num_choices = max(self.min_num_choices, self.max_num_choices);
                        true
                    }
                    Err(_) => false,
                }
            }
            Msg::SetMaxChoices(value) => {
                let num = value.parse::<usize>();
                match num {
                    Ok(num) => {
                        let options_len = self.options.len();
                        self.max_num_choices = min(max(1, num), options_len);
                        self.min_num_choices = min(self.min_num_choices, self.max_num_choices);
                        true
                    }
                    Err(_) => false,
                }
            }
            Msg::SetOpenTime(value) => {
                let date = Date::parse(&value);
                self.open_time = (date / 1000.) as u32;
                // TODO change close time based on global config
                true
            }
            Msg::SetCloseTime(value) => {
                let date = Date::parse(&value);
                self.close_time = (date / 1000.) as u32;
                // TODO change open time based on global config
                true
            }
            Msg::Submit => {
                info!("submitting form");

                let creator = match self.creator() {
                    Some(creator) => creator,
                    None => return false,
                };

                let chain_id =
                    "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f".to_string();
                let network = scatter::Network {
                    blockchain: Some("eos".into()),
                    chain_id: Some(chain_id.clone()),
                    protocol: Some("http".into()),
                    host: Some("localhost".into()),
                    port: Some(8888),
                };

                let config = scatter::EosConfig {
                    chain_id: Some(chain_id.clone()),
                    key_provider: None,
                    http_endpoint: None,
                    expire_in_seconds: None,
                    broadcast: None,
                    verbose: None,
                    debug: None,
                    sign: None,
                };

                let action_data = CreatePollAction {
                    creator: creator.to_string(),
                    slug: self.slug.clone(),
                    title: self.title.clone(),
                    options: self.options.clone(),
                    min_num_choices: self.min_num_choices,
                    max_num_choices: self.max_num_choices,
                    whitelist: self.whitelist.clone(),
                    blacklist: self.blacklist.clone(),
                    open_time: self.open_time,
                    close_time: self.close_time,
                    metadata: "".to_string(),
                };

                let data = serde_json::to_value(action_data).unwrap();

                let action = scatter::Action {
                    account: "eosstrawpoll".into(),
                    name: "createpoll".into(),
                    authorization: vec![scatter::Authorization {
                        actor: creator.to_string(),
                        permission: "active".into(),
                    }],
                    data,
                };

                let actions = vec![action];

                self.submitting = true;
                self.scatter_agent
                    .send(ScatterInput::PushActions(network, config, actions));
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
                    if result.is_ok() {
                        self.scatter_agent.send(ScatterInput::CurrentIdentity);
                    }
                    self.scatter_connected = Some(result);
                    true
                }
                ScatterOutput::PushedActions(result) => {
                    self.pushed_poll = Some(result.clone());
                    if let (Ok(_), Some(creator)) = (result, self.creator()) {
                        let route = Route::Poll(creator, self.slug.clone());
                        let url = route.to_string();
                        self.router.send(RouterInput::ChangeRoute(url, ()));
                    }
                    true
                }
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("PROPS CHANGED {:#?}", props);
        self.context = props.context;
        true
    }
}

impl Renderable<PollForm> for PollForm {
    fn view(&self) -> Html<Self> {
        let submit_text = if self.submitting {
            "Creating..."
        } else {
            "Create Poll"
        };
        html! {
            <form class="poll_form app_container", >
                { self.view_title() }
                { self.view_options() }
                { self.view_num_choices() }
                <section class="poll_form_times", >
                    <header><h2>{ "Times" }</h2></header>
                    <main>
                        { self.view_open_time() }
                        { self.view_close_time() }
                    </main>
                    <aside>
                        {"Open for 2 days"}
                    </aside>
                </section>
                {
                    if self.use_whitelist {
                        self.view_whitelist()
                    } else {
                        self.view_blacklist()
                    }
                }
                <button
                    class="poll_form_submit",
                    type="submit",
                    disabled=self.submitting,
                    onclick=|e| {
                        e.prevent_default();
                        Msg::Submit
                    },
                >
                    { submit_text }
                </button>
            </form>
        }
    }
}

impl PollForm {
    fn creator(&self) -> Option<String> {
        let result = match &self.scatter_identity {
            Some(result) => result,
            None => return None,
        };

        let identity = match result {
            Ok(identity) => identity,
            Err(_error) => return None,
        };

        match identity.accounts.first() {
            Some(ref account) => Some(account.name.clone()),
            None => None,
        }
    }

    fn view_title(&self) -> Html<PollForm> {
        html! {
            <section class="poll_form_title", >
                <header><h2>{ "Title" }</h2></header>
                <main>
                    <input
                        disabled=self.submitting,
                        placeholder="What is your question?",
                        class="poll_form_input",
                        value=&self.title,
                        oninput=|e| Msg::SetTitle(e.value),
                        required=true,
                        maxlength=self.global_config.max_title_len,
                    />
                </main>
                <aside>
                    { "Required. Must be between 1-30 characters" }
                </aside>
            </section>
        }
    }

    fn view_options(&self) -> Html<PollForm> {
        html! {
            <section class="poll_form_options", >
                <header><h2>{ "Options" }</h2></header>
                <main>
                    { for self.options.iter().enumerate().map(|(i, o)| self.view_option(i, o)) }
                </main>
                <aside>{ "Info" }</aside>
            </section>
        }
    }

    fn view_option(&self, index: usize, option: &str) -> Html<PollForm> {
        let options_len = self.options.len();
        let is_last = index == options_len - 1;
        let is_not_full = options_len < self.global_config.max_options_len;
        html! {
            <div class="poll_form_option", >
                <input
                    disabled=self.submitting,
                    placeholder=format!("Option {}", index + 1),
                    value=option,
                    onfocus=|_| {
                        if is_last && is_not_full {
                            Msg::AddOption
                        } else {
                            Msg::NoOp
                        }
                    },
                    oninput=|e| Msg::SetOption(index, e.value),
                    maxlength=self.global_config.max_option_len,
                />
                <button
                    class="poll_form_option_delete",
                    tabindex=-1,
                    disabled=options_len <= 2 || self.submitting,
                    onclick=|e| {
                        e.prevent_default();
                        Msg::DelOption(index)
                    },
                >
                    { "Delete" }
                </button>
            </div>
        }
    }

    fn view_whitelist(&self) -> Html<PollForm> {
        html! {
            <section class="poll_form_voter_requirements", >
                <header><h2>{ "Voter requirements" }</h2></header>
                <main>
                    <div class="field", >
                        <strong class="poll_form_label", >
                            { "Whitelist" }
                        </strong>
                        <div class="prefill_links", >
                            <a href="#", onclick=|e| {
                                e.prevent_default();
                                Msg::SetOnlyBPs
                            }, >
                                { "Only BPs" }
                            </a>
                            <a href="#", onclick=|e| {
                                e.prevent_default();
                                Msg::SetOnlyActiveBPs
                            }, >
                                { "Only Active BPs" }
                            </a>
                            <a href="#", onclick=|e| {
                                e.prevent_default();
                                Msg::SetOnlyStandbyBPs
                            }, >
                                { "Only Standby BPs" }
                            </a>
                        </div>
                        <input
                            disabled=self.submitting,
                            class="poll_form_input",
                            oninput=|e| Msg::SetWhitelist(e.value),
                            value=self.whitelist.join(" "),
                        />
                    </div>
                </main>
                <aside>{ "May select up to 30 accounts. Accounts must exist." }</aside>
            </section>
        }
    }

    fn view_blacklist(&self) -> Html<PollForm> {
        html! {
            <section class="poll_form_blacklist", >
                <strong class="poll_form_label", >
                    { "Blacklist" }
                </strong>
                <input
                    disabled=self.submitting,
                    class="poll_form_input",
                    oninput=|e| Msg::SetBlacklist(e.value),
                    value=self.blacklist.join(" "),
                />
            </section>
        }
    }

    fn view_num_choices(&self) -> Html<PollForm> {
        html! {
            <section class="poll_form_num_choices", >
                <header>
                    <h2>{ "Number of choices" }</h2>
                </header>
                <main>
                    { self.view_min_num_choices() }
                    { self.view_num_choices_ranges() }
                    { self.view_max_num_choices() }
                </main>
                <aside>
                    { self.view_num_choices_info() }
                </aside>
            </section>
        }
    }

    fn view_num_choices_info(&self) -> Html<PollForm> {
        let options_len = self.options.len();
        let min_num_choices = self.min_num_choices;
        let max_num_choices = self.max_num_choices;
        let text = if min_num_choices == max_num_choices {
            if min_num_choices == options_len {
                "Voters must rank all options".to_string()
            } else if min_num_choices == 1 {
                "Voters must select one option".to_string()
            } else {
                format!("Voters must select {} options", min_num_choices)
            }
        } else {
            format!(
                "Voters can select {} to {} options",
                min_num_choices, max_num_choices
            )
        };
        html! {
            <span class="info", >
                { text }
            </span>
        }
    }

    fn view_num_choices_ranges(&self) -> Html<PollForm> {
        let options_len = self.options.len();
        html! {
            <>
                <input
                    class="poll_form_min_num_choices_range",
                    disabled=self.submitting,
                    type="range",
                    min=1,
                    max=options_len,
                    value=self.min_num_choices,
                    oninput=|e| Msg::SetMinChoices(e.value),
                />
                <input
                    class="poll_form_max_num_choices_range",
                    disabled=self.submitting,
                    type="range",
                    min=1,
                    max=options_len,
                    value=self.max_num_choices,
                    oninput=|e| Msg::SetMaxChoices(e.value),
                />
            </>
        }
    }

    fn view_min_num_choices(&self) -> Html<PollForm> {
        html! {
            <label class="min_num_choices", >
                <strong>{ "Min" }</strong>
                <input
                    disabled=self.submitting,
                    type="number",
                    value=self.min_num_choices,
                    oninput=|e| Msg::SetMinChoices(e.value),
                    min=1,
                    max=self.options.len(),
                />
            </label>
        }
    }

    fn view_max_num_choices(&self) -> Html<PollForm> {
        html! {
            <label class="max_num_choices", >
                <strong>{ "Max" }</strong>
                <input
                    disabled=self.submitting,
                    type="number",
                    value=self.max_num_choices,
                    oninput=|e| Msg::SetMaxChoices(e.value),
                    min=1,
                    max=self.options.len(),
                />
            </label>
        }
    }

    fn view_open_time(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form_open_time", >
                <strong class="poll_form_label", >
                    { "Open" }
                </strong>
                <input
                    disabled=self.submitting,
                    type="datetime-local",
                    class="poll_form_input",
                    oninput=|e| Msg::SetOpenTime(e.value),
                />
            </div>
        }
    }

    fn view_close_time(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form_close_time", >
                <strong class="poll_form_label", >
                    { "Close" }
                </strong>
                <input
                    disabled=self.submitting,
                    type="datetime-local",
                    class="poll_form_input",
                    oninput=|e| Msg::SetCloseTime(e.value),
                />
            </div>
        }
    }
}
