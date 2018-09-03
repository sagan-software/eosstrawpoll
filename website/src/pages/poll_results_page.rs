use components::Link;
use eos::service::*;
use prelude::*;
use scatter::*;
use std::time::Duration;
use stdweb::web::document;
use yew::services::fetch::FetchTask;
use yew::services::{IntervalService, Task};

pub struct PollResultsPage {
    eos: EosService,
    context: Context,
    task: Option<FetchTask>,
    poll: Option<Result<Poll, EosError>>,
    creator: String,
    slug: String,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<ScatterIdentity, ScatterError>>,
    link: ComponentLink<PollResultsPage>,
    interval_service: IntervalService,
    interval_task: Option<Box<Task>>,
    chain: Chain,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub creator: String,
    pub slug: String,
    pub chain: Chain,
}

pub enum Msg {
    Polls(Result<TableRows<Poll>, EosError>),
    Scatter(ScatterOutput),
    FetchPolls,
}

impl Component for PollResultsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let creator = props.creator;

        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        let mut poll_page = PollResultsPage {
            eos: EosService::new(),
            context: props.context,
            task: None,
            poll: None,
            slug: props.slug.clone(),
            creator: creator.clone(),
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            link,
            interval_service: IntervalService::new(),
            interval_task: None,
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
                        None => Some(Err(EosError::Message("poll not found".to_string()))),
                    },
                    Err(error) => Some(Err(error)),
                };
                self.task = None;

                if self.interval_task.is_none() {
                    let cb = self.link.send_back(|_| Msg::FetchPolls);
                    let task = self.interval_service.spawn(Duration::from_secs(1), cb);
                    self.interval_task = Some(Box::new(task));
                }
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    self.scatter_identity = Some(result);
                }
                ScatterOutput::ForgotIdentity(_result) => {
                    self.scatter_identity = None;
                }
                ScatterOutput::Connected(result) => {
                    if result.is_ok() {
                        self.scatter_agent.send(ScatterInput::CurrentIdentity);
                    }
                    self.scatter_connected = Some(result);
                }
                ScatterOutput::PushedTransaction(_) => (),
            },
            Msg::FetchPolls => {
                self.fetch_poll();
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Page for PollResultsPage {
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
        "poll_page poll_page_results".into()
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

page_view! { PollResultsPage }

impl PollResultsPage {
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

    fn voter(&self) -> Option<AccountName> {
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

    fn get_vote(&mut self) -> Option<Vote> {
        let voter = match self.voter() {
            Some(voter) => voter,
            None => return None,
        };

        let votes = match &self.poll {
            Some(Ok(poll)) => &poll.votes,
            _ => return None,
        };

        let filtered_votes = votes
            .iter()
            .filter(|vote| vote.voter == voter)
            .cloned()
            .collect::<Vec<Vote>>();

        filtered_votes.first().map(|v| v.clone())
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
                <h1>{ "Error" }</h1>
            </div>
        }
    }

    fn view_ok(&self, poll: &Poll) -> Html<Self> {
        let vote = Route::Poll(
            self.chain.to_chain_id_prefix(),
            poll.creator.clone(),
            poll.slug.clone(),
        );
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
                    { for results.iter().map(|(option, percent, votes)| self.view_option_result(option, *percent, &votes)) }
                </div>
                <div class="poll_actions", >
                    <Link: route=vote, text="Vote", />
                </div>
            </>
        }
    }

    fn view_option_result(
        &self,
        option: &str,
        percent: f32,
        _votes: &[(String, usize)],
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
