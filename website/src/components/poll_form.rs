use agents::api::*;
use agents::router::{RouterAgent, RouterInput, RouterOutput};
use agents::scatter::{
    self, ScatterAction, ScatterAgent, ScatterError, ScatterInput, ScatterOutput,
};
use components::{RelativeTime, Svg, Symbol};
use context::Context;
use route::Route;
use std::cmp::{max, min};
use std::collections::HashSet;
use stdweb::traits::IEvent;
use stdweb::web::Date;
use types::*;
use yew::prelude::*;

pub struct PollForm {
    action: CreatePoll,
    submitting: bool,
    context: Context,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pushed_poll: Option<Result<scatter::PushedTransaction, ScatterError>>,
    router: Box<Bridge<RouterAgent<()>>>,
    global_config: GlobalConfig,
    _api: Box<Bridge<ApiAgent>>,
    use_advanced: bool,

    // basic options
    allow_multiple_choices: bool,
    allow_writeins: bool,
    close_in_an_hour: bool,
}

pub enum Msg {
    NoOp,
    Scatter(ScatterOutput),
    Router(RouterOutput<()>),
    Api(ApiOutput),
    Submit,
    SetTitle(String),
    AddOption,
    SetOption(usize, String),
    DelOption(usize),

    // Basic options
    ToggleAllowMultipleChoices,
    ToggleAllowWriteins,
    ToggleCloseAfterAnHour,

    // Advanced options
    ToggleAdvanced,
    AddListAccount(String),
    DelListAccount(usize),
    SetMinChoices(String),
    SetMaxChoices(String),
    SetOpenTime(String),
    SetCloseTime(String),
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Props {
    pub context: Context,
}

impl Component for PollForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let scatter_agent =
            ScatterAgent::new("eosstrawpoll".into(), 10000, link.send_back(Msg::Scatter));
        let api_config = props.context.api_config();
        let mut api = ApiAgent::new(api_config, link.send_back(Msg::Api));
        api.send(ApiInput::GetGlobalConfig);

        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);

        PollForm {
            action: CreatePoll::default(),
            submitting: false,
            context: props.context,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed_poll: None,
            router,
            global_config: GlobalConfig::default(),
            _api: api,
            use_advanced: false,
            allow_multiple_choices: false,
            allow_writeins: false,
            close_in_an_hour: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => false,
            Msg::NoOp => false,
            Msg::SetTitle(value) => {
                self.action.title = value;
                true
            }
            Msg::AddOption => {
                info!("add option");
                if self.action.options.len() < self.global_config.max_options_len {
                    self.action.options.push("".to_string());
                    true
                } else {
                    false
                }
            }
            Msg::SetOption(i, value) => {
                info!("setting option {} to {}", i, value);
                if i < self.action.options.len() {
                    self.action.options[i] = value;
                }
                true
            }
            Msg::DelOption(i) => {
                let options_len = self.action.options.len();
                if i < options_len && options_len > 2 {
                    self.action.options.remove(i);
                    debug!(
                        "deleted option {}, leaving options {:#?}",
                        i, self.action.options
                    );
                    let options_len = self.action.options.len();
                    self.action.max_choices = min(self.action.max_choices, options_len);
                    self.action.min_choices = min(self.action.max_choices, self.action.min_choices);
                    true
                } else {
                    false
                }
            }
            Msg::AddListAccount(_value) => true,
            Msg::DelListAccount(_index) => true,
            Msg::SetMinChoices(value) => {
                let num = value.parse::<usize>();
                match num {
                    Ok(num) => {
                        let options_len = self.action.options.len();
                        self.action.min_choices = min(max(1, num), options_len);
                        self.action.max_choices =
                            max(self.action.min_choices, self.action.max_choices);
                        true
                    }
                    Err(_) => false,
                }
            }
            Msg::SetMaxChoices(value) => {
                let num = value.parse::<usize>();
                match num {
                    Ok(num) => {
                        let options_len = self.action.options.len();
                        self.action.max_choices = min(max(1, num), options_len);
                        self.action.min_choices =
                            min(self.action.min_choices, self.action.max_choices);
                        true
                    }
                    Err(_) => false,
                }
            }
            Msg::SetOpenTime(value) => {
                let date = Date::parse(&value);
                self.action.open_time = (date / 1000.) as u32;
                // TODO change close time based on global config
                true
            }
            Msg::SetCloseTime(value) => {
                let date = Date::parse(&value);
                self.action.close_time = (date / 1000.) as u32;
                // TODO change open time based on global config
                true
            }
            Msg::Submit => {
                info!("submitting form");
                self.submitting = true;

                if let Some(creator) = self.creator() {
                    self.action.creator = creator;
                } else {
                    let required_fields = self.context.required_fields();
                    let scatter_input = ScatterInput::GetIdentity(required_fields);
                    self.scatter_agent.send(scatter_input);
                    return true;
                }

                self.action.random_slug();

                let network = self.context.network();
                let config = self.context.eos_config();
                let mut action = self.action.clone();

                if !self.use_advanced {
                    // Set basic options below

                    action.min_choices = 1;
                    let options_len = action.options.len();
                    match (self.allow_multiple_choices, self.allow_writeins) {
                        (true, true) => {
                            action.max_writeins = max(options_len, 2);
                            action.max_choices = action.max_writeins;
                        }
                        (true, false) => {
                            action.max_writeins = 0;
                            action.max_choices = options_len;
                        }
                        (false, true) => {
                            action.max_writeins = 1;
                            action.max_choices = 1;
                        }
                        (false, false) => {
                            action.max_writeins = 0;
                            action.max_choices = 1;
                        }
                    }

                    if self.close_in_an_hour {
                        let now = (Date::now() / 1000.) as u32;
                        action.close_time = now + 60 * 60;
                    }
                }

                let actions: Vec<ScatterAction> = vec![action.into()];

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
                        match (result, self.creator()) {
                            (Ok(_), Some(creator)) => {
                                let route = Route::Poll(creator, self.action.slug.clone());
                                let url = route.to_string();
                                self.router.send(RouterInput::ChangeRoute(url, ()));
                            }
                            (Ok(_), None) => {
                                warn!("Something strange happened: a poll was successfully submitted but no creator was found.");
                            }
                            (Err(error), _) => {
                                error!("Error submitting poll: {:#?}", error);
                            }
                        };
                        true
                    } else {
                        false
                    }
                }
            },
            Msg::Api(output) => match output {
                ApiOutput::GlobalConfig(global_config) => {
                    if let Ok(global_config) = global_config {
                        self.global_config = global_config;
                    }
                    true
                }
                _ => false,
            },
            Msg::ToggleAdvanced => {
                self.use_advanced = !self.use_advanced;
                true
            }
            Msg::ToggleAllowMultipleChoices => {
                self.allow_multiple_choices = !self.allow_multiple_choices;
                true
            }
            Msg::ToggleAllowWriteins => {
                self.allow_writeins = !self.allow_writeins;
                true
            }
            Msg::ToggleCloseAfterAnHour => {
                self.close_in_an_hour = !self.close_in_an_hour;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
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
                { self.view_tabs() }
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

    fn validate_options(&self) -> Result<(), String> {
        let mut options = HashSet::new();
        let max_option_len = self.global_config.max_option_len;

        for option in &self.action.options {
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
                value=&self.action.title,
                oninput=|e| Msg::SetTitle(e.value),
                required=true,
                maxlength=self.global_config.max_title_len,
            />
        };
        let help: Html<Self> = html! {
            <p></p>
        };
        self.view_field("Title", "title", input, help)
    }

    fn view_options(&self) -> Html<PollForm> {
        let input: Html<Self> = html! {
            { for self.action.options.iter().enumerate().map(|(i, o)| self.view_option(i, o)) }
        };
        // let max_options_len = self.global_config.max_options_len;
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
        let options_len = self.action.options.len();
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

    fn view_tabs(&self) -> Html<PollForm> {
        let basic_tab = if self.use_advanced {
            html! {
                <button
                    class="poll_tab",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::ToggleAdvanced
                    },
                >
                    { "Basic" }
                </button>
            }
        } else {
            html! {
                <span class="poll_tab poll_tab_active", >
                    { "Basic" }
                </span>
            }
        };
        let advanced_tab = if !self.use_advanced {
            html! {
                <button
                    class="poll_tab",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::ToggleAdvanced
                    },
                >
                    { "Advanced" }
                </button>
            }
        } else {
            html! {
                <span class="poll_tab poll_tab_active", >
                    { "Advanced" }
                </span>
            }
        };
        html! {
            <div class="poll_form_tabs", >
                <nav class="poll_form_tabs_nav", >
                    { basic_tab }
                    { advanced_tab }
                </nav>
                    {
                        if self.use_advanced {
                            self.view_advanced_tab()
                        } else {
                            self.view_basic_tab()
                        }
                    }
            </div>
        }
    }

    fn view_advanced_tab(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form_tabs_content poll_form_advanced_tab", >
                { self.view_num_choices() }
                { self.view_open_time() }
                { self.view_close_time() }
                { self.view_whitelist() }
            </div>
        }
    }

    fn view_basic_tab(&self) -> Html<PollForm> {
        // Allow multiple choices
        // Allow writeins
        // Close after N hours
        //
        html! {
            <div class="poll_form_tabs_content poll_form_basic_tab", >
                <label>
                    <input type="checkbox",
                        onchange=|_| Msg::ToggleAllowMultipleChoices,
                        checked=self.allow_multiple_choices,
                    />
                    { "Allow multiple choices" }
                </label>
                <label>
                    <input type="checkbox",
                        onchange=|_| Msg::ToggleAllowWriteins,
                        checked=self.allow_writeins,
                    />
                    { "Allow write-in choices" }
                </label>
                <label>
                    <input type="checkbox",
                        onchange=|_| Msg::ToggleCloseAfterAnHour,
                        checked=self.close_in_an_hour,
                    />
                    { "Close in an hour" }
                </label>
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
            <p>{ format!("Up to {} accounts. Accounts must exist.", self.global_config.max_account_list_len) }</p>
        };
        self.view_field("Whitelist", "whitelist", input, help)
    }

    fn view_num_choices(&self) -> Html<PollForm> {
        let options_len = self.action.options.len();
        let min_choices = self.action.min_choices;
        let max_choices = self.action.max_choices;
        let indicators: Vec<usize> = (0..options_len).collect();
        let input = html! {
            <>
                <div class="min_choices_input", >
                    <input
                        disabled=self.submitting,
                        value=min_choices,
                        oninput=|e| Msg::SetMinChoices(e.value),
                        min=1,
                        max=options_len,
                    />
                </div>
                <div class="num_choices_range", >
                    <input class="min_choices_range",
                        disabled=self.submitting,
                        type="range",
                        min=1,
                        max=options_len,
                        value=min_choices,
                        oninput=|e| Msg::SetMinChoices(e.value),
                    />
                    <input class="max_choices_range",
                        disabled=self.submitting,
                        type="range",
                        min=1,
                        max=options_len,
                        value=max_choices,
                        oninput=|e| Msg::SetMaxChoices(e.value),
                    />
                    <div class="num_choices_range_highlighted", ></div>
                    <div class="num_choices_range_bg", ></div>
                    { for indicators.iter().map(|_i| html!{
                        <div class="num_choices_indicator", ></div>
                    }) }
                </div>
                <div class="max_choices_input", >
                    <input
                        disabled=self.submitting,
                        value=max_choices,
                        oninput=|e| Msg::SetMaxChoices(e.value),
                        min=1,
                        max=options_len,
                    />
                </div>
            </>
        };
        let text = if min_choices == max_choices {
            if min_choices == options_len {
                "Voters must rank all options".to_string()
            } else if min_choices == 1 {
                "Voters must select one option".to_string()
            } else {
                format!("Voters must select {} options", min_choices)
            }
        } else {
            format!(
                "Voters can select {} to {} options",
                min_choices, max_choices
            )
        };
        let help = html! { <p>{ text }</p> };
        self.view_field("Number of allowed choices", "num_choices", input, help)
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
        let help = if self.action.open_time <= now {
            html! { { "Opens immediately" } }
        } else {
            html! {
                <>
                    { "Opens in " }
                    <RelativeTime: timestamp=self.action.open_time, simple=true, />
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
        let help = if self.action.close_time == 0 {
            html! { { "Never closes" } }
        } else {
            let now = (Date::now() / 1000.) as u32;
            let start = max(self.action.open_time, now);
            html! {
                <>
                    { "Closes after " }
                    <RelativeTime:
                        base_timestamp=Some(start),
                        timestamp=self.action.close_time,
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
