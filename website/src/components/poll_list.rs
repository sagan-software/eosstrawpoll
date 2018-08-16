use agents::router::{RouterAgent, RouterInput, RouterOutput};
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Poll;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct PollList {
    props: Props,
    link: ComponentLink<PollList>,
    router: Box<Bridge<RouterAgent<()>>>,
    eos: EosService,
    task: Option<FetchTask>,
    polls: Option<Result<eos::TableRows<Poll>, Error>>,
}

#[derive(PartialEq, Clone)]
pub enum PollsTable {
    Polls,
    PopularPolls,
    NewPolls,
}

impl Default for PollsTable {
    fn default() -> PollsTable {
        PollsTable::Polls
    }
}

impl ToString for PollsTable {
    fn to_string(&self) -> String {
        match self {
            PollsTable::Polls => "polls".to_string(),
            PollsTable::PopularPolls => "popularpolls".to_string(),
            PollsTable::NewPolls => "newpolls".to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum PollsOrder {
    Id,
    Created,
    Popularity,
}

impl Default for PollsOrder {
    fn default() -> PollsOrder {
        PollsOrder::Id
    }
}

pub enum Msg {
    Router(RouterOutput<()>),
    NavigateTo(Route),
    Polls(Result<eos::TableRows<Poll>, Error>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub endpoint: String,
    pub scope: String,
    pub table: Option<PollsTable>,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<u32>,
    pub order: Option<PollsOrder>,
}

impl Component for PollList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);
        let mut poll_list = PollList {
            props,
            link,
            router,
            eos: EosService::new(),
            task: None,
            polls: None,
        };
        poll_list.fetch_polls();
        poll_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
            }
            Msg::Polls(result) => {
                self.polls = Some(result);
                self.task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<PollList> for PollList {
    fn view(&self) -> Html<Self> {
        match &self.polls {
            Some(result) => match result {
                Ok(table) => {
                    if table.rows.is_empty() {
                        self.view_empty()
                    } else {
                        self.view_items(&table.rows)
                    }
                }
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl PollList {
    fn fetch_polls(&mut self) {
        let table = self
            .props
            .table
            .clone()
            .unwrap_or_else(|| PollsTable::Polls);
        let mut params = eos::TableRowsParams {
            json: true,
            scope: self.props.scope.clone(),
            code: "eosstrawpoll".to_string(),
            table: table.to_string(),
            lower_bound: self.props.lower_bound.clone(),
            upper_bound: self.props.upper_bound.clone(),
            limit: self.props.limit,
            key_type: None,
            index_position: None,
        };

        if table != PollsTable::Polls {
            params.scope = "eosstrawpoll".to_string();
        }

        let order = self.props.order.clone().unwrap_or_else(|| PollsOrder::Id);
        match order {
            PollsOrder::Id => {}
            PollsOrder::Popularity => {
                params.key_type = Some("popularity".to_string());
                // params.index_position = Some("1".to_string());
            }
            PollsOrder::Created => {
                params.key_type = Some("created".to_string());
                // params.index_position = Some("2".to_string());
            }
        }
        let callback = self.link.send_back(Msg::Polls);
        let endpoint = &self.props.endpoint;
        let task = self.eos.get_table_rows(endpoint, params, callback);
        self.task = Some(task);
    }

    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="poll_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div class="poll_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_items(&self, polls: &[Poll]) -> Html<Self> {
        html! {
            <ul class="poll_list_items", >
                { for polls.iter().map(|poll| self.view_item(poll)) }
            </ul>
        }
    }

    fn view_item(&self, poll: &Poll) -> Html<Self> {
        let route = Route::Poll(poll.creator.clone(), poll.slug.clone());
        html! {
            <li class="poll_list_item", >
                <a
                    href=route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(route.clone())
                    },
                >
                    { &poll.title }
                    { format!("({})", &poll.popularity)}
                </a>
            </li>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="poll_list_empty", >
                { "Empty" }
            </div>
        }
    }
}
