use agents::router::{RouterAgent, RouterInput};
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Poll;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct PollResultsPage {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
    task: Option<FetchTask>,
    poll: Option<Result<Poll, Error>>,
    creator: String,
    slug: String,
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
}

impl Component for PollResultsPage {
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
        let context = props.context;
        let callback = link.send_back(Msg::Polls);
        let task = eos.get_table_rows(context.endpoint.as_str(), params.clone(), callback);

        PollResultsPage {
            router,
            context,
            task: Some(task),
            poll: None,
            slug: props.slug.clone(),
            creator: creator.clone(),
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PollResultsPage> for PollResultsPage {
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

impl PollResultsPage {
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
        html! {
            <div class="app_container", >
                <h1>{ &poll.title } { "(Results)" }</h1>
                <ul class="poll_options", >
                    { for poll.options.iter().map(|option| self.view_option(option)) }
                </ul>
            </div>
        }
    }

    fn view_option(&self, option: &str) -> Html<Self> {
        html! {
            <li class="poll_option", >{ option }</li>
        }
    }
}
