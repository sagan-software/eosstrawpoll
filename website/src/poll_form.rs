use context::Context;
use global_config::GlobalConfig;
use services::scatter::{self, Identity, ScatterError, ScatterService};
use std::cmp::{max, min};
use stdweb::traits::IEvent;
use stdweb::web::Date;
use yew::prelude::*;

pub struct PollForm {
    pub title: String,
    pub options: Vec<String>,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub min_num_choices: usize,
    pub max_num_choices: usize,
    pub open_time: u32,
    pub close_time: u32,
    pub submitting: bool,
    pub context: Box<Context>,
    pub global_config: GlobalConfig,
    pub active_bps: Vec<String>,
    pub standby_bps: Vec<String>,
    pub link: ComponentLink<PollForm>,
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
    GotIdentity(Result<Identity, ScatterError>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Box<Context>,
}

const EOS_MAINNET: &'static str =
    "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906";

const TELOS_TESTNET: &'static str =
    "9e46127b78e0a7f6906f549bba3d23b264c70ee6ec781aed9d4f1b72732f34fc";

impl Component for PollForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PollForm {
            title: "".to_string(),
            options: vec!["".to_string(), "".to_string(), "".to_string()],
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
            Msg::GotIdentity(result) => {
                info!("got identity {:#?}", result);
                self.context.identity = Some(result);
                true
            }
            Msg::Submit => {
                info!("submitting form");
                if self.context.is_logged_in() {
                    info!("logged in, do some stuff");
                } else {
                    match self.context.scatter {
                        Some(ref scatter) => {
                            info!("attempting to login");
                            let callback = self.link.send_back(Msg::GotIdentity);
                            let chain_id = EOS_MAINNET.to_string();
                            scatter.get_identity_for_chain(chain_id, callback);
                        }
                        None => {
                            info!("no scatter");
                        }
                    }
                }
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
        let submit_text = if self.context.is_logged_in() {
            "Create Poll"
        } else {
            "Login & Create Poll"
        };
        html! {
            <form
                class="poll_form",
            >
                { self.view_title() }
                { self.view_options() }
                <div class="poll_form__advanced", >
                    { self.view_num_choices() }
                    <div class="poll_form__times", >
                        { self.view_open_time() }
                        <div class="poll_form__time_info", >
                            {"Open for 2 days"}
                        </div>
                        { self.view_close_time() }
                    </div>
                    { self.view_whitelist() }
                    { self.view_blacklist() }
                </div>
                <button
                    class="poll_form__submit",
                    type="submit",
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
    fn view_title(&self) -> Html<PollForm> {
        html! {
            <label class="poll_form__title", >
                <input
                    placeholder="Poll title",
                    class="poll_form__input",
                    value=&self.title,
                    oninput=|e| Msg::SetTitle(e.value),
                    required=true,
                    maxlength=self.global_config.max_title_len,
                />
            </label>
        }
    }

    fn view_options(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form__options", >
                { for self.options.iter().enumerate().map(|(i, o)| self.view_option(i, o)) }
            </div>
        }
    }

    fn view_option(&self, index: usize, option: &str) -> Html<PollForm> {
        let options_len = self.options.len();
        let is_last = index == options_len - 1;
        let is_not_full = options_len < self.global_config.max_options_len;
        html! {
            <div class="poll_form__option", >
                <input
                    placeholder="Poll option...",
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
                <a
                    class="poll_form__option__delete",
                    disabled=options_len <= 2,
                    onclick=|e| {
                        e.prevent_default();
                        Msg::DelOption(index)
                    },
                >
                    { "Delete" }
                </a>
            </div>
        }
    }

    fn view_whitelist(&self) -> Html<PollForm> {
        html! {
            <label class="poll_form__whitelist", >
                <strong class="poll_form__label", >
                    { "Whitelist" }
                </strong>
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
                <input
                    class="poll_form__input",
                    oninput=|e| Msg::SetWhitelist(e.value),
                    value=self.whitelist.join(" "),
                />
            </label>
        }
    }

    fn view_blacklist(&self) -> Html<PollForm> {
        html! {
            <label class="poll_form__blacklist", >
                <strong class="poll_form__label", >
                    { "Blacklist" }
                </strong>
                <input
                    class="poll_form__input",
                    oninput=|e| Msg::SetBlacklist(e.value),
                    value=self.blacklist.join(" "),
                />
            </label>
        }
    }

    fn view_num_choices(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form__num_choices", >
                { self.view_min_num_choices() }
                <div>
                    { self.view_num_choices_info() }
                    { self.view_num_choices_range() }
                </div>
                { self.view_max_num_choices() }
            </div>
        }
    }

    fn view_num_choices_info(&self) -> Html<PollForm> {
        let options_len = self.options.len();
        html! {
            <span class="multi-range", >
                { "Voters must select 5 options" }
            </span>
        }
    }

    fn view_num_choices_range(&self) -> Html<PollForm> {
        let options_len = self.options.len();
        html! {
            <span class="multi-range", >
                <input
                    type="range",
                    min=1,
                    max=options_len,
                    value=self.min_num_choices,
                    oninput=|e| Msg::SetMinChoices(e.value),
                />
                <input
                    type="range",
                    min=1,
                    max=options_len,
                    value=self.max_num_choices,
                    oninput=|e| Msg::SetMaxChoices(e.value),
                />
            </span>
        }
    }

    fn view_min_num_choices(&self) -> Html<PollForm> {
        html! {
            <label class="poll_form__min_num_choices", >
                <strong class="poll_form__label", >
                    { "Min Choices" }
                </strong>
                <input
                    type="number",
                    class="poll_form__input",
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
            <div class="poll_form__max_num_choices", >
                <strong class="poll_form__label", >
                    { "Max Choices" }
                </strong>
                <input
                    type="number",
                    class="poll_form__input",
                    value=self.max_num_choices,
                    oninput=|e| Msg::SetMaxChoices(e.value),
                    min=1,
                    max=self.options.len(),
                />
            </div>
        }
    }

    fn view_open_time(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form__open_time", >
                <strong class="poll_form__label", >
                    { "Open Time" }
                </strong>
                <input
                    type="datetime-local",
                    class="poll_form__input",
                    oninput=|e| Msg::SetOpenTime(e.value),
                />
            </div>
        }
    }

    fn view_close_time(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form__close_time", >
                <strong class="poll_form__label", >
                    { "Close Time" }
                </strong>
                <input
                    type="datetime-local",
                    class="poll_form__input",
                    oninput=|e| Msg::SetCloseTime(e.value),
                />
            </div>
        }
    }
}
