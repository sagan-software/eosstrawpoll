use agents::api::*;
use agents::router::RouterAgent;
use agents::scatter::{
    self, ScatterAction, ScatterAgent, ScatterError, ScatterInput, ScatterOutput,
};
use components::*;
use context::Context;
use route::Route;
use std::cmp::{max, min};
use std::collections::HashSet;
use stdweb::traits::IEvent;
use types::*;
use yew::prelude::*;

pub struct PollForm {
    action: CreatePoll,
    submitting: bool,
    context: Context,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    global_config: GlobalConfig,
    _api: Box<Bridge<ApiAgent>>,
    use_advanced: bool,
    validation_result: Option<Result<(), String>>,

    // basic options
    allow_multiple_choices: bool,
    allow_writeins: bool,
}

pub enum Msg {
    NoOp,
    Scatter(ScatterOutput),
    Api(ApiOutput),
    Submit,
    SetTitle(String),
    AddOption,
    SetOption(usize, String),
    DelOption(usize),
    Validate,

    // Basic options
    ToggleAllowMultipleChoices,
    ToggleAllowWriteins,
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Props {
    pub context: Context,
}

impl Component for PollForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let scatter_agent =
            ScatterAgent::new("eosstrawpoll".into(), 10000, link.send_back(Msg::Scatter));
        let api_config = props.context.api_config();
        let mut api = ApiAgent::new(api_config, link.send_back(Msg::Api));
        api.send(ApiInput::GetGlobalConfig);

        PollForm {
            action: CreatePoll::default(),
            submitting: false,
            context: props.context,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            global_config: GlobalConfig::default(),
            _api: api,
            validation_result: None,
            use_advanced: false,
            allow_multiple_choices: false,
            allow_writeins: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NoOp => false,
            Msg::SetTitle(value) => {
                self.action.title = value;
                self.validation_result = Some(self.validate());
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
                    if self.validation_result.is_some() {
                        self.validation_result = Some(self.validate());
                    }
                }
                true
            }
            Msg::DelOption(i) => {
                let options_len = self.action.options.len();
                if i < options_len && options_len > 1 {
                    self.action.options.remove(i);
                    debug!(
                        "deleted option {}, leaving options {:#?}",
                        i, self.action.options
                    );
                    let options_len = self.action.options.len();
                    self.action.max_choices = min(self.action.max_choices, options_len);
                    self.action.min_choices = min(self.action.max_choices, self.action.min_choices);
                    if self.validation_result.is_some() {
                        self.validation_result = Some(self.validate());
                    }
                    true
                } else {
                    false
                }
            }
            Msg::Submit => {
                info!("submitting form");

                let validation_result = self.validate();
                if validation_result.is_err() {
                    self.validation_result = Some(validation_result);
                    return true;
                }

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

                // Remove empty options
                action
                    .options
                    .retain(|ref option| !option.trim().is_empty());

                if !self.use_advanced {
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
                        self.submitting = false;
                        match (result, self.creator()) {
                            (Ok(_), Some(creator)) => {
                                let route = Route::Poll(
                                    "cf057bbfb726".into(),
                                    creator,
                                    self.action.slug.clone(),
                                );
                                let url = route.to_string();
                                RouterAgent::redirect(url);
                                // self.router.send(RouterInput::ChangeRoute(url, ()));
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
            Msg::ToggleAllowMultipleChoices => {
                self.allow_multiple_choices = !self.allow_multiple_choices;
                if self.validation_result.is_some() {
                    self.validation_result = Some(self.validate());
                }
                true
            }
            Msg::ToggleAllowWriteins => {
                self.allow_writeins = !self.allow_writeins;
                if self.validation_result.is_some() {
                    self.validation_result = Some(self.validate());
                }
                true
            }
            Msg::Validate => {
                self.validation_result = Some(self.validate());
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
            <form
                class="poll_form",
                onsubmit=|e| {
                    e.prevent_default();
                    Msg::Submit
                },
            >
                { self.title_view() }
                { self.options_view() }
                { self.basic_view() }
                { self.submit_view() }
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

    fn validate(&self) -> Result<(), String> {
        if self.action.title.trim().is_empty() {
            return Err("Title must not be empty".to_string());
        }

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
        if num_options < 2 && !self.allow_writeins {
            return Err("Must have at least 2 options or allow write-in answers".to_string());
        }

        let max_options_len = self.global_config.max_options_len;
        if num_options > max_options_len {
            return Err(format!("Cannot have more than {} options", max_options_len));
        }

        Ok(())
    }

    fn title_view(&self) -> Html<Self> {
        let class_modifier =
            if self.validation_result.is_some() && self.action.title.trim().is_empty() {
                "-invalid"
            } else {
                ""
            };
        html! {
            <input class=format!("poll_form_title_input {}", class_modifier),
                disabled=self.submitting,
                placeholder="What is your question?",
                value=&self.action.title,
                oninput=|e| Msg::SetTitle(e.value),
                onblur=|_| Msg::Validate,
                required=true,
                maxlength=self.global_config.max_title_len,
                autofocus=true,
            />
        }
    }

    fn options_view(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form_options", >
                { for self.action.options.iter().enumerate().map(|(i, o)| self.option_view(i, o)) }
            </div>
        }
    }

    fn option_view(&self, index: usize, option: &str) -> Html<PollForm> {
        let options_len = self.action.options.len();
        let is_last = index == options_len - 1;
        let is_not_full = options_len < self.global_config.max_options_len;
        html! {
            <div class="poll_form_option", key=format!("{}_{}", index, option), >
                <input class="poll_form_option_input",
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
                <button class="poll_form_option_delete",
                    tabindex=-1,
                    disabled=options_len <= 1 || self.submitting,
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

    fn basic_view(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form_basic", >
                <label>
                    <input type="checkbox",
                        onchange=|_| Msg::ToggleAllowWriteins,
                        checked=self.allow_writeins,
                        disabled=self.submitting,
                    />
                    <span>{ "Allow write-in answers" }</span>
                </label>
                <label>
                    <input type="checkbox",
                        onchange=|_| Msg::ToggleAllowMultipleChoices,
                        checked=self.allow_multiple_choices,
                        disabled=self.submitting,
                    />
                    <span>{ "Allow multiple answers" }</span>
                </label>
            </div>
        }
    }

    fn status_view(&self) -> Html<Self> {
        match &self.validation_result {
            None => html! { <></> },
            Some(Ok(_)) => html! {
                <div class="poll_form_status -valid", >
                    <Svg: symbol=Symbol::CheckCircle, />
                </div>
            },
            Some(Err(error)) => html! {
                <div class="poll_form_status -invalid", >
                    <div class="message", >{ error }</div>
                    <Svg: symbol=Symbol::Warning, />
                </div>
            },
        }
    }

    fn submit_view(&self) -> Html<PollForm> {
        let submit_text = if self.submitting {
            "Creating Poll..."
        } else {
            "Create Poll"
        };
        let is_invalid = match &self.validation_result {
            Some(Err(_)) => true,
            _ => false,
        };
        let class_modifier = if is_invalid { "-invalid" } else { "" };
        html! {
            <div class=format!("poll_form_submit {}", class_modifier), >
                <Button:
                    class="poll_form_submit_button",
                    size=Size::Large,
                    style=if is_invalid { ButtonStyle::Danger } else { ButtonStyle::Primary },
                    type_="submit",
                    disabled=self.submitting || is_invalid,
                    text=submit_text.to_string(),
                />
                { self.status_view() }
            </div>
        }
    }
}
