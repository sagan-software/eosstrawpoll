use std::cmp::{max, min};
use stdweb::traits::IEvent;
use stdweb::web::Date;
use yew::prelude::*;

#[derive(Debug)]
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
}

pub enum Msg {
    NoOp,
    Submit,
    SetTitle(String),
    AddOption,
    SetOption(usize, String),
    DelOption(usize),
    SetWhitelist(String),
    SetBlacklist(String),
    SetMinChoices(String),
    SetMaxChoices(String),
    SetOpenTime(String),
    SetCloseTime(String),
}

impl Component for PollForm {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
                info!("setting open time: {}", value);
                let date = Date::parse(&value);
                info!("setting open time 2: {}", date);
                let open_time = (date / 1000.) as u32;
                info!("setting open time 3: {}", open_time);
                false
            }
            Msg::SetCloseTime(value) => {
                info!("setting close time: {}", value);
                false
            }
            Msg::Submit => {
                info!("submitting form {:#?}", self);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PollForm> for PollForm {
    fn view(&self) -> Html<Self> {
        html! {
            <form
                class="poll_form",
            >
                { self.view_title() }
                { self.view_options() }
                { self.view_whitelist() }
                { self.view_blacklist() }
                { self.view_num_choices() }
                { self.view_open_time() }
                { self.view_close_time() }
                <button
                    class="poll_form__submit",
                    type="submit",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::Submit
                    },
                >
                    { "Create Poll" }
                </button>
            </form>
        }
    }
}

impl PollForm {
    fn view_title(&self) -> Html<PollForm> {
        html! {
            <label class="poll_form__title", >
                <strong class="poll_form__label", >
                    { "Title" }
                </strong>
                <input
                    placeholder="Poll title",
                    class="poll_form__input",
                    value=&self.title,
                    oninput=|e| Msg::SetTitle(e.value),
                />
            </label>
        }
    }

    fn view_options(&self) -> Html<PollForm> {
        html! {
            <fieldset class="poll_form__options", >
                <legend class="poll_form__label", >
                    { "Options" }
                </legend>
                { for self.options.iter().enumerate().map(|(i, o)| self.view_option(i, o)) }
                <button
                    class="poll_form__options__add",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::AddOption
                    },
                >
                    { "Add Option" }
                </button>
            </fieldset>
        }
    }

    fn view_option(&self, index: usize, option: &str) -> Html<PollForm> {
        let options_len = self.options.len();
        html! {
            <div class="poll_form__option", >
                <input
                    placeholder="Poll option...",
                    value=option,
                    onfocus=|_| {
                        if index == options_len - 1 {
                            Msg::AddOption
                        } else {
                            Msg::NoOp
                        }
                    },
                    oninput=|e| Msg::SetOption(index, e.value),
                />
                <button
                    class="poll_form__option__delete",
                    disabled=options_len <= 2,
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
            <label class="poll_form__whitelist", >
                <strong class="poll_form__label", >
                    { "Whitelist" }
                </strong>
                <textarea
                    class="poll_form__input",
                    oninput=|e| Msg::SetWhitelist(e.value),
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
                <textarea
                    class="poll_form__input",
                    oninput=|e| Msg::SetBlacklist(e.value),
                />
            </label>
        }
    }

    fn view_num_choices(&self) -> Html<PollForm> {
        html! {
            <div class="poll_form__num_choices", >
                { self.view_min_num_choices() }
                { self.view_max_num_choices() }
            </div>
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
