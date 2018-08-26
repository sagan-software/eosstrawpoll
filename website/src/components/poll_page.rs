use agents::router::{RouterAgent, RouterInput};
use agents::scatter::{
    self, ScatterAction, ScatterAgent, ScatterError, ScatterInput, ScatterOutput,
};
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use std::collections::HashMap;
use std::time::Duration;
use stdweb::traits::IEvent;
use traits::Page;
use types::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::{IntervalService, Task};

pub struct PollPage {
    eos: EosService,
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
    task: Option<FetchTask>,
    poll: Option<Result<Poll, Error>>,
    creator: String,
    slug: String,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pushed: Option<Result<scatter::PushedTransaction, ScatterError>>,
    choices: Vec<Choice>,
    writein_input: String,
    show_results: bool,
    submitting: bool,
    link: ComponentLink<PollPage>,
    interval_service: IntervalService,
    interval_task: Option<Box<Task>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub creator: String,
    pub slug: String,
    pub show_results: bool,
}

pub enum Msg {
    Ignore,
    NavigateTo(Route),
    Polls(Result<eos::TableRows<Poll>, Error>),
    Scatter(ScatterOutput),
    ToggleChoice(Choice),
    SetWriteinInput(String),
    AddWritein,
    Vote,
    FetchPolls,
}

impl Component for PollPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::Ignore);
        let router = RouterAgent::bridge(callback);
        let creator = props.creator;

        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        let context = props.context;
        let mut poll_page = PollPage {
            eos: EosService::new(),
            router,
            context,
            task: None,
            poll: None,
            slug: props.slug.clone(),
            creator: creator.clone(),
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            choices: Vec::new(),
            writein_input: "".to_string(),
            show_results: props.show_results,
            submitting: false,
            link,
            interval_service: IntervalService::new(),
            interval_task: None,
        };

        poll_page.fetch_poll();
        poll_page
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                match route {
                    Route::Poll(creator, slug) => {
                        if creator == self.creator && slug == self.slug {
                            if self.show_results {
                                self.router
                                    .send(RouterInput::ChangeRouteNoBroadcast(url, ()));
                                self.show_results = false;
                                if let Some(mut task) = self.interval_task.take() {
                                    task.cancel();
                                    self.interval_task = None;
                                }
                                true
                            } else {
                                false
                            }
                        } else {
                            self.router.send(RouterInput::ChangeRoute(url, ()));
                            false
                        }
                    }
                    Route::PollResults(creator, slug) => {
                        if creator == self.creator && slug == self.slug {
                            if self.show_results {
                                false
                            } else {
                                self.fetch_poll();
                                self.router
                                    .send(RouterInput::ChangeRouteNoBroadcast(url, ()));
                                self.show_results = true;
                                true
                            }
                        } else {
                            self.router.send(RouterInput::ChangeRoute(url, ()));
                            false
                        }
                    }
                    _ => {
                        self.router.send(RouterInput::ChangeRoute(url, ()));
                        false
                    }
                }
            }
            Msg::Polls(result) => {
                self.poll = match result {
                    Ok(table) => match table.rows.first() {
                        Some(poll) => Some(Ok(poll.clone())),
                        None => Some(Err(format_err!("poll not found"))),
                    },
                    Err(error) => Some(Err(error)),
                };
                self.task = None;
                self.load_choices();

                if self.show_results && self.interval_task.is_none() {
                    let cb = self.link.send_back(|_| Msg::FetchPolls);
                    let task = self.interval_service.spawn(Duration::from_secs(1), cb);
                    self.interval_task = Some(Box::new(task));
                }

                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    let is_ok = result.is_ok();
                    self.scatter_identity = Some(result);
                    match (is_ok, self.submitting) {
                        (false, true) => {
                            self.submitting = false;
                            true
                        }
                        (true, true) => self.update(Msg::Vote),
                        (true, false) => {
                            self.load_choices();
                            true
                        }
                        (false, false) => true,
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
                        self.pushed = Some(result.clone());
                        self.submitting = false;
                        if result.is_ok() {
                            let route = Route::PollResults(self.creator.clone(), self.slug.clone());
                            self.update(Msg::NavigateTo(route))
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
            },
            Msg::ToggleChoice(choice) => {
                info!("CHOICES: {:#?}, CHOICE: {:#?}", self.choices, choice);
                if self.choices.contains(&choice) {
                    self.choices.retain(|c| choice != *c);
                } else {
                    self.choices.push(choice);
                }
                if let Some(Ok(poll)) = &self.poll {
                    if self.choices.len() > poll.max_choices {
                        self.choices.remove(0);
                    }
                }
                true
            }
            Msg::Vote => {
                info!("submitting form");
                self.submitting = true;

                let voter = match self.voter() {
                    Some(voter) => voter,
                    None => {
                        let required_fields = self.context.required_fields();
                        let scatter_input = ScatterInput::GetIdentity(required_fields);
                        self.scatter_agent.send(scatter_input);
                        return true;
                    }
                };

                let network = self.context.network();
                let config = self.context.eos_config();

                let action: ScatterAction = CreateVote {
                    creator: self.creator.to_string(),
                    slug: self.slug.clone(),
                    voter: voter.clone(),
                    choices: self.choices.clone(),
                }.into();
                let actions = vec![action];

                self.scatter_agent
                    .send(ScatterInput::PushActions(network, config, actions));
                true
            }
            Msg::FetchPolls => {
                self.fetch_poll();
                true
            }
            Msg::SetWriteinInput(input) => {
                self.writein_input = input;
                true
            }
            Msg::AddWritein => {
                self.choices
                    .push(Choice::from_writein(self.writein_input.clone()));
                self.writein_input = "".to_string();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Page for PollPage {
    fn title(&self) -> String {
        match &self.poll {
            Some(result) => match result {
                Ok(poll) => poll.title.clone(),
                Err(_error) => "Error".to_string(),
            },
            None => "Loading...".to_string(),
        }
    }
    fn class(&self) -> String {
        let state_modifier = match &self.poll {
            Some(result) => match result {
                Ok(_) => "loaded",
                Err(_) => "error",
            },
            None => "loading",
        };

        let results_modifier = if self.show_results { "results" } else { "vote" };

        format!(
            "poll_page poll_page_{} poll_page_{}",
            state_modifier, results_modifier
        )
    }
    fn content(&self) -> Html<Self> {
        match &self.poll {
            Some(result) => match result {
                Ok(poll) => self.view_ok(poll),
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

page_view! { PollPage }

impl PollPage {
    fn fetch_poll(&mut self) {
        let params = eos::TableRowsParams {
            scope: self.creator.clone(),
            code: "eosstrawpoll".to_string(),
            table: "polls".to_string(),
            json: true,
            lower_bound: Some(self.slug.clone()),
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };

        let callback = self.link.send_back(Msg::Polls);
        let task = self
            .eos
            .get_table_rows(self.context.endpoint.as_str(), params, callback);
        self.task = Some(task);
    }

    fn voter(&self) -> Option<String> {
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

    fn has_voted(&self) -> bool {
        let voter = match self.voter() {
            Some(voter) => voter,
            None => return false,
        };

        let votes = match &self.poll {
            Some(Ok(poll)) => &poll.votes,
            _ => return false,
        };

        votes.into_iter().any(|vote| vote.voter == voter)
    }

    fn load_choices(&mut self) {
        let voter = match self.voter() {
            Some(voter) => voter,
            None => return,
        };

        let votes = match &self.poll {
            Some(Ok(poll)) => &poll.votes,
            _ => return,
        };

        let filtered_votes = votes
            .iter()
            .filter(|vote| vote.voter == voter)
            .cloned()
            .collect::<Vec<Vote>>();

        if let Some(vote) = filtered_votes.first() {
            self.choices = vote.choices.to_owned();
        }
    }

    fn view_loading(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Loading" }</h1>
            </div>
        }
    }

    fn view_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Error: " }{ error }</h1>
            </div>
        }
    }

    fn view_ok(&self, poll: &Poll) -> Html<Self> {
        if self.show_results {
            self.view_ok_results(poll)
        } else {
            self.view_ok_vote(poll)
        }
    }

    fn vote_text(&self) -> &str {
        let is_logged_in = self.voter().is_some();
        match (is_logged_in, self.submitting, self.has_voted()) {
            (false, false, _) => "Login & Vote",
            (false, true, _) => "Logging in...",
            (true, false, false) => "Vote",
            (true, true, false) => "Voting...",
            (true, false, true) => "Change Vote",
            (true, true, true) => "Changing Vote...",
        }
    }

    fn view_ok_vote(&self, poll: &Poll) -> Html<Self> {
        let results = Route::PollResults(poll.creator.clone(), poll.slug.clone());
        let num_options = poll.options.len();
        let num_choices_text = match (
            poll.min_choices,
            poll.max_choices,
            poll.min_choices == poll.max_choices,
            poll.max_choices == num_options,
        ) {
            (1, 1, _, _) => "choose one option".to_string(),
            (_, _, true, true) => "rank all options".to_string(),
            (_, _, true, false) => format!("choose and rank {} options", poll.min_choices),
            (_, _, false, _) => format!(
                "choose and rank {} to {} options",
                poll.min_choices, poll.max_choices
            ),
        };
        let choose_one = poll.min_choices == 1 && poll.max_choices == 1;
        let select_text = if self.choices.len() < poll.min_choices && !self.choices.is_empty() {
            let diff = poll.min_choices - self.choices.len();
            if diff == 1 {
                "Select one more option".to_string()
            } else {
                format!("Select {} more options", diff)
            }
        } else {
            "".to_string()
        };
        html! {
            <>
                <p class="poll_num_choices", >
                    { format!("Please {}:", num_choices_text) }
                </p>
                <div class="poll_options", >
                    { for poll.options.iter().enumerate().map(|(i, option)| self.view_option(i, option, choose_one)) }
                </div>
                { if poll.max_writeins > 0 { self.view_writein_input() } else { html! {} } }
                <div class="poll_actions", >
                    <button
                        disabled=self.choices.len() < poll.min_choices,
                        type="submit",
                        onclick=|e| {
                            e.prevent_default();
                            Msg::Vote
                        },
                    >
                        { self.vote_text() }
                    </button>
                    <p>{ select_text }</p>
                    <a
                        href=results.to_string(),
                        onclick=|e| {
                            e.prevent_default();
                            Msg::NavigateTo(results.clone())
                        },
                    >
                        { "View results" }
                    </a>
                </div>
            </>
        }
    }

    fn view_writein_input(&self) -> Html<Self> {
        html! {
            <>
                <input class="poll_writein_input",
                    oninput=|e| Msg::SetWriteinInput(e.value),
                    value=&self.writein_input,
                    placeholder="Write in your own answer",
                />
                <button class="poll_writein_button",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::AddWritein
                    },
                >
                    { "Add" }
                </button>
            </>
        }
    }

    fn view_option(&self, index: usize, option: &str, choose_one: bool) -> Html<Self> {
        let choice = Choice {
            option_index: index as i16,
            writein: "".to_string(),
        };
        let is_selected = self.choices.contains(&choice);
        let input = if choose_one {
            html! {
                <input class="poll_option_checkbox",
                    type="radio",
                    name="choices",
                    onchange=|_| Msg::ToggleChoice(choice.clone()),
                    checked=is_selected,
                />
            }
        } else {
            html! {
                <input class="poll_option_checkbox",
                    type="checkbox",
                    onchange=|_| Msg::ToggleChoice(choice.clone()),
                    checked=is_selected,
                />
            }
        };
        html! {
            <label class="poll_option", >
                { input }
                <span class="poll_option_text", >
                    { option }
                </span>
            </label>
        }
    }

    fn view_ok_results(&self, poll: &Poll) -> Html<Self> {
        let vote = Route::Poll(poll.creator.clone(), poll.slug.clone());
        let results_text = if poll.votes.len() == 1 {
            "Results from one voter:".to_string()
        } else {
            format!("Results from {} voters:", poll.votes.len())
        };
        let results = poll.results_by_percent();
        info!("RESULTS! {:#?}", results);
        html! {
            <>
                <p class="poll_num_choices", >
                    { results_text }
                </p>
                <div class="poll_options", >
                    { for results.iter().map(|(option, percent, votes)| self.view_option_result(option, percent, &votes)) }
                </div>
                <div class="poll_actions", >
                    <a
                        href=vote.to_string(),
                        onclick=|e| {
                            e.prevent_default();
                            Msg::NavigateTo(vote.clone())
                        },
                    >
                        { self.vote_text() }
                    </a>
                </div>
            </>
        }
    }

    fn view_option_result(
        &self,
        option: &str,
        percent: &f32,
        votes: &[(String, usize)],
    ) -> Html<Self> {
        html! {
            <div class="poll_option", >
                <span class="poll_option_text", >{ option }</span>
                <span class="poll_option_percent", >
                    { (percent * 100.) as u32 }{ "%" }
                </span>
                <span class="poll_option_bar", style=format!("transform:scaleX({})", percent), ></span>
            </div>
        }
    }
}
