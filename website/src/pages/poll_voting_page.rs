use components::*;
use eos::*;
use prelude::*;
use router::RouterAgent;
use scatter::*;
use stdweb::traits::IEvent;
use stdweb::web::document;
use views::svg;
use yew::services::fetch::FetchTask;

pub struct PollVotingPage {
    props: Props,
    eos: EosService,
    poll_task: Option<FetchTask>,
    poll: Option<Result<Poll, EosError>>,
    votes_task: Option<FetchTask>,
    votes: Vec<Vote>,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<ScatterIdentity, ScatterError>>,
    pushed: Option<Result<PushedTransaction, ScatterError>>,
    answers: Vec<Answer>,
    available_answers: Vec<(String, Answer)>,
    writein_input: String,
    submitting: bool,
    link: ComponentLink<PollVotingPage>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub chain: Chain,
    pub poll_id: PollId,
}

pub enum Msg {
    Polls(Result<TableRows<Poll>, EosError>),
    Votes(Result<TableRows<Vote>, EosError>),
    Scatter(ScatterOutput),
    ToggleAnswer(Answer),
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
            props,
            eos: EosService::new(),
            poll_task: None,
            poll: None,
            votes_task: None,
            votes: Vec::new(),
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            answers: Vec::new(),
            available_answers: Vec::new(),
            writein_input: "".to_string(),
            submitting: false,
            link,
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
                        None => Some(Err(EosError::Message("poll not found".to_string()))),
                    },
                    Err(error) => Some(Err(error)),
                };
                self.poll_task = None;
                self.load_answers();
                self.fetch_votes();
                true
            }
            Msg::Votes(result) => match result {
                Ok(table) => {
                    self.votes = table.rows;
                    self.load_answers();
                    true
                }
                Err(error) => {
                    error!("Error getting votes: {:#?}", error);
                    false
                }
            },
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
                            self.load_answers();
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
                                self.props.chain.to_chain_id_prefix(),
                                self.props.poll_id.clone(),
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
            Msg::ToggleAnswer(answer) => {
                info!("CHOICES: {:#?}, CHOICE: {:#?}", self.answers, answer);
                self.toggle_answer(answer);
                true
            }
            Msg::Vote => {
                info!("submitting form");
                self.submitting = true;

                let voter = match self.voter() {
                    Some(voter) => voter,
                    None => {
                        let required_fields = self.props.chain.to_scatter_required_fields();
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
                    self.answers = vec![Answer {
                        prefilled_option_index: -1,
                        writein: self.writein_input.clone(),
                    }];
                }

                let network = self.props.chain.to_scatter_network();
                let config = self.props.chain.to_eos_config();

                let action = CreateVote {
                    poll_id: self.props.poll_id.clone(),
                    account: voter.clone(),
                    answers: self.answers.clone(),
                }.to_action(&self.props.chain);

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
                if self.writein_input.is_empty() {
                    return false;
                }

                let answer = Answer::from_writein(self.writein_input.clone());
                self.toggle_answer(answer.clone());

                let available_answer = (self.writein_input.clone(), answer.clone());
                if !self.available_answers.contains(&available_answer) {
                    self.available_answers.push(available_answer);
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
                    } else if poll.max_writein_answers == 0 {
                        "-loaded -no-writein"
                    } else {
                        "-loaded -allow-writein"
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
    fn toggle_answer(&mut self, answer: Answer) {
        if self.answers.contains(&answer) {
            self.answers.retain(|c| answer != *c);
        } else {
            self.answers.push(answer);
        }

        if let Some(Ok(poll)) = &self.poll {
            if self.answers.len() > poll.max_answers {
                self.answers.remove(0);
            }
        }
    }

    fn fetch_poll(&mut self) {
        let params = TableRowsParams {
            scope: self.props.chain.code_account.clone(),
            code: self.props.chain.code_account.clone(),
            table: "polls".to_string(),
            json: true,
            lower_bound: Some(self.props.poll_id.clone()),
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
            encode_type: None,
        };

        let callback = self.link.send_back(Msg::Polls);
        let endpoint = self.props.chain.endpoint.to_string();
        let task = self.eos.get_table_rows(endpoint.as_str(), params, callback);
        self.poll_task = Some(task);
    }

    fn fetch_votes(&mut self) {
        let lower_bound = name_to_u64(self.props.poll_id.clone());
        let upper_bound = lower_bound + 1;
        let params = TableRowsParams {
            scope: self.props.chain.code_account.clone(),
            code: self.props.chain.code_account.clone(),
            table: "votes".to_string(),
            json: true,
            lower_bound: Some(lower_bound.to_string()),
            upper_bound: Some(upper_bound.to_string()),
            limit: Some(10000),
            key_type: Some("i64".into()),
            index_position: Some("2".into()),
            encode_type: None,
        };

        let callback = self.link.send_back(Msg::Votes);
        let endpoint = self.props.chain.endpoint.to_string();
        let task = self.eos.get_table_rows(endpoint.as_str(), params, callback);
        self.votes_task = Some(task);
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
        // let voter = match self.voter() {
        //     Some(voter) => voter,
        //     None => return false,
        // };

        // let votes = match &self.poll {
        //     Some(Ok(poll)) => &poll.votes,
        //     _ => return false,
        // };

        // votes.into_iter().any(|vote| vote.voter == voter)
        false
    }

    fn load_answers(&mut self) {
        let poll = match &self.poll {
            Some(Ok(poll)) => poll,
            _ => return,
        };

        self.available_answers = poll
            .prefilled_options
            .iter()
            .enumerate()
            .map(|(i, label)| {
                (
                    label.clone(),
                    Answer {
                        prefilled_option_index: i as i16,
                        writein: "".into(),
                    },
                )
            }).collect();

        let voter = match self.voter() {
            Some(voter) => voter,
            None => return,
        };

        let filtered_votes = self
            .votes
            .iter()
            .filter(|vote| vote.account == voter)
            .cloned()
            .collect::<Vec<Vote>>();

        if let Some(vote) = filtered_votes.first() {
            self.answers = vote.answers.to_owned();

            if is_only_one_writein(poll) {
                self.writein_input = self
                    .answers
                    .first()
                    .map(|c| c.writein.clone())
                    .unwrap_or_else(|| "".to_string());
            } else {
                for answer in &self.answers {
                    if !answer.writein.is_empty() {
                        self.available_answers
                            .push((answer.writein.clone(), answer.clone()));
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

    fn view_error(&self, error: &EosError) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Error " }</h1>
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
        //  - Only 1 pre-filled answer
        //  - Only 1 writein answer
        //  - Multiple pre-filled answers and 1 writein option
        //  - Multiple pre-filled answers and multiple writein options
        //  - 1 pre-filled answer and 1 writein option
        //  - 1 pre-filled answer and multiple writein options

        let results = Route::PollResults(self.props.chain.to_chain_id_prefix(), poll.id.clone());
        let select_text = if self.answers.len() < poll.min_answers && !self.answers.is_empty() {
            let diff = poll.min_answers - self.answers.len();
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
        let can_submit = (self.answers.len() >= poll.min_answers)
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
        let choose_one = poll.min_answers == 1 && poll.max_answers == 1;
        let writein_input = if poll.max_writein_answers > 0 {
            self.view_writein_input(poll)
        } else {
            html! { <></> }
        };
        html! {
            <>
                <p class="poll_num_answers", >
                    { format!("Please {}:", vote_help_text(poll)) }
                </p>
                <div class="poll_options", >
                    { for self.available_answers.iter().map(|(label, answer)| self.view_option(label, (*answer).clone(), choose_one)) }
                    { writein_input }
                </div>
            </>
        }
    }

    fn view_writein_input(&self, poll: &Poll) -> Html<Self> {
        if poll.max_writein_answers == 0 {
            return html! { <></> };
        }
        html! {
            <div class="poll_writein", >
                <input class="poll_writein_input",
                    oninput=|e| Msg::SetWriteinInput(e.value),
                    value=&self.writein_input,
                    placeholder="Write in your answer",
                />
                <button class="poll_writein_button",
                    disabled=self.writein_input.is_empty(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::AddWritein
                    },
                >
                    { svg::plus() }
                </button>
            </div>
        }
    }

    fn view_option(&self, label: &str, answer: Answer, choose_one: bool) -> Html<Self> {
        let is_selected = self.answers.contains(&answer);
        let input = if choose_one {
            html! {
                <input class="poll_option_checkbox",
                    type="radio",
                    name="answers",
                    onchange=|_| Msg::ToggleAnswer(answer.clone()),
                    checked=is_selected,
                />
            }
        } else {
            html! {
                <input class="poll_option_checkbox",
                    type="checkbox",
                    onchange=|_| Msg::ToggleAnswer(answer.clone()),
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
    poll.prefilled_options.is_empty() && poll.max_writein_answers == 1
}

fn vote_help_text(poll: &Poll) -> String {
    if is_only_one_writein(poll) {
        return "write in your answer".to_string();
    }

    let num_options = poll.prefilled_options.len();
    match (
        poll.min_answers,
        poll.max_answers,
        poll.min_answers == poll.max_answers,
        poll.max_answers == num_options,
    ) {
        (1, 1, _, _) => "choose one option".to_string(),
        (_, _, true, true) => "rank all options".to_string(),
        (_, _, true, false) => format!("choose and rank {} options", poll.min_answers),
        (_, _, false, _) => format!(
            "choose and rank {} to {} options",
            poll.min_answers, poll.max_answers
        ),
    }
}
