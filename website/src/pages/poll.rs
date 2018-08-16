use agents::router::{RouterAgent, RouterInput};
use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
use failure::Error;
use route::Route;
use serde_json;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::{CreateVoteAction, Poll};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct PollPage {
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
    choices: Vec<usize>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub creator: String,
    pub slug: String,
}

pub enum Msg {
    Ignore,
    NavigateTo(Route),
    Polls(Result<eos::TableRows<Poll>, Error>),
    Scatter(ScatterOutput),
    ToggleChoice(usize),
    Vote,
}

impl Component for PollPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::Ignore);
        let router = RouterAgent::bridge(callback);
        let creator = props.creator;

        let mut eos = EosService::new();
        let params = eos::TableRowsParams {
            scope: creator.clone(),
            code: "eosstrawpoll".to_string(),
            table: "polls".to_string(),
            json: true,
            lower_bound: Some(props.slug.clone()),
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };

        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));

        let context = props.context;
        let callback = link.send_back(Msg::Polls);
        let task = eos.get_table_rows(context.endpoint.as_str(), params.clone(), callback);

        PollPage {
            router,
            context,
            task: Some(task),
            poll: None,
            slug: props.slug.clone(),
            creator: creator.clone(),
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            choices: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
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
                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    info!("got identity {:#?}", result);
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
                ScatterOutput::PushedActions(result) => {
                    self.pushed = Some(result.clone());
                    let results = Route::PollResults(self.creator.clone(), self.slug.clone());
                    let url = results.to_string();
                    self.router.send(RouterInput::ChangeRoute(url, ()));
                    true
                }
            },
            Msg::ToggleChoice(choice) => {
                info!("CHOICES: {:#?}, CHOICE: {:#?}", self.choices, choice);
                if self.choices.contains(&choice) {
                    self.choices.retain(|&c| c != choice);
                } else {
                    self.choices.push(choice);
                }
                true
            }
            Msg::Vote => {
                info!("submitting form");

                let voter = match self.voter() {
                    Some(voter) => voter,
                    None => return false,
                };

                let chain_id =
                    "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f".to_string();
                let network = scatter::Network {
                    blockchain: Some("eos".into()),
                    chain_id: Some(chain_id.clone()),
                    protocol: Some("http".into()),
                    host: Some("localhost".into()),
                    port: Some(8888),
                };

                let config = scatter::EosConfig {
                    chain_id: Some(chain_id.clone()),
                    key_provider: None,
                    http_endpoint: None,
                    expire_in_seconds: None,
                    broadcast: None,
                    verbose: None,
                    debug: None,
                    sign: None,
                };

                let action_data = CreateVoteAction {
                    creator: self.creator.to_string(),
                    slug: self.slug.clone(),
                    voter: voter.clone(),
                    choices: self.choices.clone(),
                    metadata: "".to_string(),
                };

                let data = serde_json::to_value(action_data).unwrap();

                let action = scatter::Action {
                    account: "eosstrawpoll".into(),
                    name: "createvote".into(),
                    authorization: vec![scatter::Authorization {
                        actor: voter.to_string(),
                        permission: "active".into(),
                    }],
                    data,
                };

                let actions = vec![action];

                self.scatter_agent
                    .send(ScatterInput::PushActions(network, config, actions));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PollPage> for PollPage {
    fn view(&self) -> Html<Self> {
        match &self.poll {
            Some(result) => match result {
                Ok(poll) => self.view_ok(poll),
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl PollPage {
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
        let results = Route::PollResults(poll.creator.clone(), poll.slug.clone());
        html! {
            <form>
                <h1>{ &poll.title }</h1>
                <ul>
                    { for poll.options.iter().enumerate().map(|(i, option)| self.view_option(i, option)) }
                </ul>
                <p>
                    <a
                        href=results.to_string(),
                        onclick=|e| {
                            e.prevent_default();
                            Msg::NavigateTo(results.clone())
                        },
                    >
                        { "View results" }
                    </a>
                </p>
                <button
                    type="submit",
                    onclick=|e| {
                        e.prevent_default();
                        Msg::Vote
                    },
                >
                    {"Vote"}
                </button>
            </form>
        }
    }

    fn view_option(&self, index: usize, option: &str) -> Html<Self> {
        let is_selected = self.choices.contains(&index);
        html! {
            <li>
                <label>
                    <input
                        type="checkbox",
                        onchange=|_| Msg::ToggleChoice(index),
                        checked=is_selected,
                    />
                    { option }
                </label>
            </li>
        }
    }
}
