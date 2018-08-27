use agents::scatter::*;
use components::Link;
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use std::time::Duration;
use stdweb::traits::IEvent;
use stdweb::web::document;
use traits::Page;
use types::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::{IntervalService, Task};

pub struct PollResultsPage {
    eos: EosService,
    context: Context,
    task: Option<FetchTask>,
    poll: Option<Result<Poll, Error>>,
    creator: String,
    slug: String,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<Identity, ScatterError>>,
    link: ComponentLink<PollResultsPage>,
    interval_service: IntervalService,
    interval_task: Option<Box<Task>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub creator: String,
    pub slug: String,
    pub chain_id_prefix: String,
}

pub enum Msg {
    Polls(Result<eos::TableRows<Poll>, Error>),
    Scatter(ScatterOutput),
    FetchPolls,
}

impl Component for PollResultsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let creator = props.creator;

        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        let context = props.context;
        let mut poll_page = PollResultsPage {
            eos: EosService::new(),
            context,
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

                if self.interval_task.is_none() {
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
                ScatterOutput::PushedActions(_) => true,
            },
            Msg::FetchPolls => {
                self.fetch_poll();
                true
            }
        }
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
        let state_modifier = match &self.poll {
            Some(result) => match result {
                Ok(_) => "loaded",
                Err(_) => "error",
            },
            None => "loading",
        };
        format!("poll_page poll_page_results poll_page_{}", state_modifier)
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

    fn view_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Error: " }{ error }</h1>
            </div>
        }
    }

    fn view_ok(&self, poll: &Poll) -> Html<Self> {
        let vote = Route::Poll(
            "cf057bbfb726".into(),
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
                    { for results.iter().map(|(option, percent, votes)| self.view_option_result(option, percent, &votes)) }
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
