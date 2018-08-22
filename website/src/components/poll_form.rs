use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use components::{RelativeTime, Svg, Symbol};
use context::Context;
use failure::Error;
use route::Route;
use serde_json;
use services::eos::{self, EosService, TableRows};
use std::cmp::{max, min};
use std::collections::HashSet;
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;
use stdweb::web::Date;
use types::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct PollForm {
    slug: String,
    title: String,
    options: Vec<String>,
    use_whitelist: bool,
    whitelist: Vec<String>,
    blacklist: Vec<String>,
    min_num_choices: usize,
    max_num_choices: usize,
    open_time: u32,
    close_time: u32,
    submitting: bool,
    context: Context,
    link: ComponentLink<PollForm>,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pushed_poll: Option<Result<scatter::PushedTransaction, ScatterError>>,
    router: Box<Bridge<RouterAgent<()>>>,
    eos: EosService,
    global_config_task: Option<FetchTask>,
    global_config: GlobalConfig,
}

pub enum Msg {
    NoOp,
    Submit,
    SetTitle(String),
    AddOption,
    SetOption(usize, String),
    DelOption(usize),
    AddListAccount(String),
    DelListAccount(usize),
    SetMinChoices(String),
    SetMaxChoices(String),
    SetOpenTime(String),
    SetCloseTime(String),
    Scatter(ScatterOutput),
    Router(RouterOutput<()>),
    GotGlobalConfig(Result<TableRows<GlobalConfig>, Error>),
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Props {
    pub context: Context,
}

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

        let mut poll_form = PollForm {
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
            link,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed_poll: None,
            router,
            eos: EosService::new(),
            global_config_task: None,
            global_config: GlobalConfig::default(),
        };

        poll_form.fetch_global_config();
        poll_form
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => false,
            Msg::NoOp => false,
            Msg::SetTitle(value) => {
                self.title = value;
                true
            }
            Msg::AddOption => {
                info!("add option");
                if self.options.len() < self.global_config.max_options_len {
                    self.options.push("".to_string());
                    true
                } else {
                    false
                }
            }
            Msg::SetOption(i, value) => {
                info!("setting option {} to {}", i, value);
                if i < self.options.len() {
                    self.options[i] = value;
                }
                true
            }
            Msg::DelOption(i) => {
                let options_len = self.options.len();
                if i < options_len && options_len > 2 {
                    self.options.remove(i);
                    debug!("deleted option {}, leaving options {:#?}", i, self.options);
                    let options_len = self.options.len();
                    self.max_num_choices = min(self.max_num_choices, options_len);
                    self.min_num_choices = min(self.max_num_choices, self.min_num_choices);
                    true
                } else {
                    false
                }
            }
            Msg::AddListAccount(value) => true,
            Msg::DelListAccount(index) => true,
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
                self.submitting = true;

                let creator = match self.creator() {
                    Some(creator) => creator,
                    None => {
                        let required_fields = self.context.required_fields();
                        let scatter_input = ScatterInput::GetIdentity(required_fields);
                        self.scatter_agent.send(scatter_input);
                        return true;
                    }
                };

                let network = self.context.network();
                let config = self.context.eos_config();

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

                self.scatter_agent
                    .send(ScatterInput::PushActions(network, config, actions));
                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    let is_ok = result.is_ok();
                    self.scatter_identity = Some(result);
                    if !is_ok && self.submitting {
                        self.submitting = false;
                    }
                    if is_ok && self.submitting {
                        self.update(Msg::Submit)
                    } else {
                        true
                    }
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
                    if self.submitting {
                        self.pushed_poll = Some(result.clone());
                        self.submitting = false;
                        if let (Ok(_), Some(creator)) = (result, self.creator()) {
                            let route = Route::Poll(creator, self.slug.clone());
                            let url = route.to_string();
                            self.router.send(RouterInput::ChangeRoute(url, ()));
                        }
                        true
                    } else {
                        false
                    }
                }
            },
            Msg::GotGlobalConfig(result) => {
                self.global_config_task = None;
                match result {
                    Ok(table) => {
                        if let Some(global_config) = table.rows.first() {
                            self.global_config = global_config.clone();
                            true
                        } else {
                            warn!("global config table is empty");
                            false
                        }
                    }
                    Err(error) => {
                        error!("couldn't fetch global config: {:#?}", error);
                        false
                    }
                }
            }
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
        html! {
            <form class="poll_form", >
                { self.view_title() }
                { self.view_options() }
                { self.view_num_choices() }
                { self.view_open_time() }
                { self.view_close_time() }
                {
                    if self.use_whitelist {
                        self.view_whitelist()
                    } else {
                        self.view_blacklist()
                    }
                }
                { self.view_submit_area() }
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

    fn fetch_global_config(&mut self) {
        let params = eos::TableRowsParams {
            json: true,
            scope: "eosstrawpoll".to_string(),
            code: "eosstrawpoll".to_string(),
            table: "config".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };
        let callback = self.link.send_back(Msg::GotGlobalConfig);
        let endpoint = &self.context.endpoint;
        let task = self.eos.get_table_rows(&endpoint, params, callback);
        self.global_config_task = Some(task);
    }

    fn validate_options(&self) -> Result<(), String> {
        let mut options = HashSet::new();
        let max_option_len = self.global_config.max_option_len;

        for option in &self.options {
            let trimmed = option.trim();

            if trimmed.is_empty() {
                continue;
            }

            if options.contains(trimmed) {
                return Err("Duplicate options are not allowed".to_string());
            }

            if trimmed.len() > max_option_len {
                return Err(format!(
                    "Options cannot be longer than {} characters",
                    max_option_len
                ));
            }

            options.insert(trimmed);
        }

        let num_options = options.len();
        if num_options < 2 {
            return Err("Must have at least 2 options".to_string());
        }

        let max_options_len = self.global_config.max_options_len;
        if num_options > max_options_len {
            return Err(format!("Cannot have more than {} options", max_options_len));
        }

        Ok(())
    }

    fn view_field(
        &self,
        label: &str,
        class: &str,
        input: Html<Self>,
        help: Html<Self>,
    ) -> Html<Self> {
        html! {
            <div class=format!("field -{}", class), >
                <label class="label", >{ label }</label>
                <div class="help", >{ help }</div>
                <div class="input", >{ input }</div>
            </div>
        }
    }

    fn view_title(&self) -> Html<Self> {
        let input: Html<Self> = html! {
            <input
                disabled=self.submitting,
                placeholder="What is your question?",
                value=&self.title,
                oninput=|e| Msg::SetTitle(e.value),
                required=true,
                maxlength=self.global_config.max_title_len,
            />
        };
        let max_title_len = self.global_config.max_title_len;
        let help: Html<Self> = html! {
            <p></p>
        };
        self.view_field("Title", "title", input, help)
    }

    fn view_options(&self) -> Html<PollForm> {
        let input: Html<Self> = html! {
            { for self.options.iter().enumerate().map(|(i, o)| self.view_option(i, o)) }
        };
        let max_options_len = self.global_config.max_options_len;
        let error = self.validate_options().err();
        let help: Html<Self> = match error {
            Some(error) => html! {
                <p>{ error }</p>
            },
            None => html! {
                <p></p>
            },
        };
        self.view_field("Options", "options", input, help)
    }

    fn view_option(&self, index: usize, option: &str) -> Html<PollForm> {
        let options_len = self.options.len();
        let is_last = index == options_len - 1;
        let is_not_full = options_len < self.global_config.max_options_len;
        html! {
            <div class="option", key=format!("{}_{}", index, option), >
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
                <button class="button -delete",
                    tabindex=-1,
                    disabled=options_len <= 2 || self.submitting,
                    onclick=|e| {
                        e.prevent_default();
                        Msg::DelOption(index)
                    },
                >
                    <Svg: symbol=Symbol::Trash, />
                </button>
            </div>
        }
    }

    fn view_whitelist(&self) -> Html<PollForm> {
        let input = html! {
            <input
                disabled=self.submitting,
                class="poll_form_input",
                oninput=|e| Msg::AddListAccount(e.value),
            />
        };
        let help = html! {
            <p>{ format!("Up to {} accounts. Accounts must exist.", self.global_config.max_whitelist_len) }</p>
        };
        self.view_field("Whitelist", "whitelist", input, help)
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
                />
            </section>
        }
    }

    fn view_num_choices(&self) -> Html<PollForm> {
        let options_len = self.options.len();
        let min_num_choices = self.min_num_choices;
        let max_num_choices = self.max_num_choices;
        let input = html! {
            <>
                <div class="min_num_choices_input", >
                    <input
                        disabled=self.submitting,
                        value=min_num_choices,
                        oninput=|e| Msg::SetMinChoices(e.value),
                        min=1,
                        max=options_len,
                    />
                </div>
                <input class="min_num_choices_range",
                    disabled=self.submitting,
                    type="range",
                    min=1,
                    max=options_len,
                    value=min_num_choices,
                    oninput=|e| Msg::SetMinChoices(e.value),
                />
                <input class="max_num_choices_range",
                    disabled=self.submitting,
                    type="range",
                    min=1,
                    max=options_len,
                    value=max_num_choices,
                    oninput=|e| Msg::SetMaxChoices(e.value),
                />
                <div class="max_num_choices_input", >
                    <input
                        disabled=self.submitting,
                        value=max_num_choices,
                        oninput=|e| Msg::SetMaxChoices(e.value),
                        min=1,
                        max=options_len,
                    />
                </div>
            </>
        };
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
        let help = html! { <p>{ text }</p> };
        self.view_field("Number of choices", "num_choices", input, help)
    }

    fn view_open_time(&self) -> Html<PollForm> {
        let input = html!{
            <input
                disabled=self.submitting,
                type="datetime-local",
                oninput=|e| Msg::SetOpenTime(e.value),
            />
        };
        let now = (Date::now() / 1000.) as u32;
        let help = if self.open_time <= now {
            html! { { "Opens immediately" } }
        } else {
            html! {
                <>
                    { "Opens in " }
                    <RelativeTime: timestamp=self.open_time, simple=true, />
                </>
            }
        };
        self.view_field("Open Time", "open_time", input, help)
    }

    fn view_close_time(&self) -> Html<PollForm> {
        let input = html!{
            <input
                disabled=self.submitting,
                type="datetime-local",
                oninput=|e| Msg::SetCloseTime(e.value),
            />
        };
        let help = if self.close_time == 0 {
            html! { { "Never closes" } }
        } else {
            let now = (Date::now() / 1000.) as u32;
            let start = max(self.open_time, now);
            html! {
                <>
                    { "Closes after " }
                    <RelativeTime:
                        base_timestamp=Some(start),
                        timestamp=self.close_time,
                        simple=true,
                    />
                </>
            }
        };
        self.view_field("Close Time", "close_time", input, help)
    }

    fn view_submit_area(&self) -> Html<PollForm> {
        let is_logged_in = self.creator().is_some();
        let submit_text = match (is_logged_in, self.submitting) {
            (false, false) => "Login & Create Poll",
            (false, true) => "Logging in...",
            (true, false) => "Create Poll",
            (true, true) => "Creating...",
        };
        html! {
            <div class="submit_area", >
                <div class="submit_bg", >
                    <button type="submit",
                        disabled=self.submitting,
                        onclick=|e| {
                            e.prevent_default();
                            Msg::Submit
                        },
                    >
                        { submit_text }
                    </button>
                </div>
            </div>
        }
    }
}
