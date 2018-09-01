use components::*;
use eos::*;
use failure::Error;
use prelude::*;
use router::RouterAgent;
use scatter::*;
use stdweb::traits::IEvent;
use stdweb::web::document;
use yew::services::fetch::FetchTask;

pub struct PollVotingPage {
    eos: EosService,
    context: Context,
    task: Option<FetchTask>,
    poll: Option<Result<Poll, Error>>,
    creator: String,
    slug: String,
    chain: Chain,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<ScatterIdentity, ScatterError>>,
    pushed: Option<Result<PushedTransaction, ScatterError>>,
    choices: Vec<Choice>,
    available_choices: Vec<(String, Choice)>,
    writein_input: String,
    submitting: bool,
    link: ComponentLink<PollVotingPage>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub creator: String,
    pub slug: String,
    pub chain: Chain,
}

pub enum Msg {
    Polls(Result<TableRows<Poll>, Error>),
    Scatter(ScatterOutput),
    ToggleChoice(Choice),
    SetWriteinInput(String),
    AddWritein,
    Vote,
}

impl Component for PollVotingPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Scatter);
        let scatter_agent = ScatterAgent::new("eosstrawpoll", 10000, callback);
        let mut poll_page = PollVotingPage {
            eos: EosService::new(),
            context: props.context,
            task: None,
            poll: None,
            slug: props.slug,
            creator: props.creator,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            choices: Vec::new(),
            available_choices: Vec::new(),
            writein_input: "".to_string(),
            submitting: false,
            link,
            chain: props.chain,
        };

        poll_page.fetch_poll();
        poll_page
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                ScatterOutput::PushedTransaction(result) => {
                    if self.submitting {
                        self.pushed = Some(result.clone());
                        self.submitting = false;
                        if result.is_ok() {
                            let route = Route::PollResults(
                                self.chain.to_chain_id_prefix(),
                                self.creator.clone(),
                                self.slug.clone(),
                            );
                            let url = route.to_string();
                            RouterAgent::redirect(url);
                            true
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
                self.toggle_choice(choice);
                true
            }
            Msg::Vote => {
                info!("submitting form");
                self.submitting = true;

                let voter = match self.voter() {
                    Some(voter) => voter,
                    None => {
                        let required_fields = self.chain.to_scatter_required_fields();
                        let scatter_input = ScatterInput::GetIdentity(required_fields);
                        self.scatter_agent.send(scatter_input);
                        return true;
                    }
                };

                let poll = match &self.poll {
                    Some(Ok(poll)) => poll,
                    _ => return true,
                };

                if is_only_one_writein(poll) {
                    self.choices = vec![Choice {
                        option_index: -1,
                        writein: self.writein_input.clone(),
                    }];
                }

                let network = self.chain.to_scatter_network();
                let config = self.chain.to_eos_config();

                let action = CreateVote {
                    creator: self.creator.to_string(),
                    slug: self.slug.clone(),
                    voter: voter.clone(),
                    choices: self.choices.clone(),
                }.to_action(&self.chain);

                let transaction: ScatterTransaction = action.into();

                self.scatter_agent.send(ScatterInput::PushTransaction(
                    network,
                    config,
                    transaction,
                ));
                true
            }
            Msg::SetWriteinInput(input) => {
                self.writein_input = input;
                true
            }
            Msg::AddWritein => {
                let choice = Choice::from_writein(self.writein_input.clone());
                self.toggle_choice(choice.clone());

                let available_choice = (self.writein_input.clone(), choice.clone());
                if !self.available_choices.contains(&available_choice) {
                    self.available_choices.push(available_choice);
                }

                self.writein_input = "".to_string();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Page for PollVotingPage {
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
                Ok(poll) => {
                    if is_only_one_writein(poll) {
                        "-loaded -only-one-writein"
                    } else {
                        ""
                    }
                }
                Err(_) => "",
            },
            None => "",
        };

        format!("poll_page poll_page_vote {}", state_modifier)
    }

    fn get_state(&self) -> PageState {
        match &self.poll {
            Some(result) => match result {
                Ok(_) => PageState::Loaded,
                Err(_) => PageState::Error,
            },
            None => PageState::Loading,
        }
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

page_view! { PollVotingPage }

impl PollVotingPage {
    fn toggle_choice(&mut self, choice: Choice) {
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
    }

    fn fetch_poll(&mut self) {
        let params = TableRowsParams {
            scope: self.creator.clone(),
            code: self.chain.code_account.clone(),
            table: "polls".to_string(),
            json: true,
            lower_bound: Some(self.slug.clone()),
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };

        let callback = self.link.send_back(Msg::Polls);
        let endpoint = self.chain.endpoint.to_string();
        let task = self.eos.get_table_rows(endpoint.as_str(), params, callback);
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
        let poll = match &self.poll {
            Some(Ok(poll)) => poll,
            _ => return,
        };

        self.available_choices = poll
            .options
            .iter()
            .enumerate()
            .map(|(i, label)| {
                (
                    label.clone(),
                    Choice {
                        option_index: i as i16,
                        writein: "".into(),
                    },
                )
            }).collect();

        let voter = match self.voter() {
            Some(voter) => voter,
            None => return,
        };

        let filtered_votes = poll
            .votes
            .iter()
            .filter(|vote| vote.voter == voter)
            .cloned()
            .collect::<Vec<Vote>>();

        if let Some(vote) = filtered_votes.first() {
            self.choices = vote.choices.to_owned();

            if is_only_one_writein(poll) {
                self.writein_input = self
                    .choices
                    .first()
                    .map(|c| c.writein.clone())
                    .unwrap_or_else(|| "".to_string());
            } else {
                for choice in &self.choices {
                    if !choice.writein.is_empty() {
                        self.available_choices
                            .push((choice.writein.clone(), choice.clone()));
                    }
                }
            }
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

    fn submit_text(&self) -> &str {
        match (self.submitting, self.has_voted()) {
            (false, false) => "Reply",
            (true, false) => "Replying...",
            (false, true) => "Change Reply",
            (true, true) => "Changing Reply...",
        }
    }

    fn view_ok(&self, poll: &Poll) -> Html<Self> {
        // Possible scenarios:
        //  - Only 1 pre-filled choice
        //  - Only 1 writein answer
        //  - Multiple pre-filled choices and 1 writein option
        //  - Multiple pre-filled choices and multiple writein options
        //  - 1 pre-filled choice and 1 writein option
        //  - 1 pre-filled choice and multiple writein options

        let results = Route::PollResults(
            self.chain.to_chain_id_prefix(),
            poll.creator.clone(),
            poll.slug.clone(),
        );
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
        let inner = if is_only_one_writein(poll) {
            self.view_only_one_writein()
        } else {
            self.view_options(poll)
        };
        let can_submit = (self.choices.len() >= poll.min_choices)
            || (is_only_one_writein(poll) && !self.writein_input.trim().is_empty());
        html! {
            <form class="poll_voting_form", onsubmit=|e| {
                e.prevent_default();
                Msg::Vote
            }, >
                // { self.view_options(poll) }
                // { self.view_writein_input(poll) }
                { inner }
                <div class="poll_actions", >
                    <Button:
                        size=Size::Large,
                        disabled=!can_submit || self.submitting,
                        type_="submit",
                        text=self.submit_text(),
                    />
                    <p>{ select_text }</p>
                    <Link: route=results, text="View results", />
                </div>
            </form>
        }
    }

    fn view_only_one_writein(&self) -> Html<Self> {
        html! {
            <>
                <input class="poll_writein_input",
                    autofocus=true,
                    oninput=|e| Msg::SetWriteinInput(e.value),
                    value=&self.writein_input,
                    placeholder="Write in your answer",
                />
            </>
        }
    }

    fn view_options(&self, poll: &Poll) -> Html<Self> {
        let choose_one = poll.min_choices == 1 && poll.max_choices == 1;

        html! {
            <>
                <p class="poll_num_choices", >
                    { format!("Please {}:", vote_help_text(poll)) }
                </p>
                <div class="poll_options", >
                    { for self.available_choices.iter().map(|(label, choice)| self.view_option(label, (*choice).clone(), choose_one)) }
                </div>
            </>
        }
    }

    fn view_writein_input(&self, poll: &Poll) -> Html<Self> {
        if poll.max_writeins == 0 {
            return html! { <></> };
        }
        html! {
            <>
                <input class="poll_writein_input",
                    oninput=|e| Msg::SetWriteinInput(e.value),
                    value=&self.writein_input,
                    placeholder="Write in your answer",
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

    fn view_option(&self, label: &str, choice: Choice, choose_one: bool) -> Html<Self> {
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
                    { label }
                </span>
            </label>
        }
    }
}

fn is_only_one_writein(poll: &Poll) -> bool {
    poll.options.is_empty() && poll.max_writeins == 1
}

fn vote_help_text(poll: &Poll) -> String {
    if is_only_one_writein(poll) {
        return "write in your answer".to_string();
    }

    let num_options = poll.options.len();
    match (
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
    }
}
