use agents::router::{RouterAgent, RouterInput};
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Poll;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct NewPollsPage {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
    task: Option<FetchTask>,
    polls: Option<Result<eos::TableRows<Poll>, Error>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

pub enum Msg {
    Ignore,
    NavigateTo(Route),
    Polls(Result<eos::TableRows<Poll>, Error>),
}

impl Component for NewPollsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::Ignore);
        let router = RouterAgent::bridge(callback);

        let mut eos = EosService::new();
        let params = eos::TableRowsParams {
            scope: "eosstrawpoll".to_string(),
            code: "eosstrawpoll".to_string(),
            table: "newpolls".to_string(),
            json: true,
            lower_bound: None,
            upper_bound: None,
            limit: Some(100),
            key_type: None,
            index_position: None,
        };
        let context = props.context;
        let callback = link.send_back(Msg::Polls);
        let task = eos.get_table_rows(context.endpoint.as_str(), params.clone(), callback);

        NewPollsPage {
            router,
            context,
            task: Some(task),
            polls: None,
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
                self.polls = Some(result);
                self.task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<NewPollsPage> for NewPollsPage {
    fn view(&self) -> Html<Self> {
        let test_route = Route::Poll("alice".into(), "balls".into());
        html! {
            <div>
                <h1>{ "New Polls" }</h1>
                { self.view_poll_list() }
                <a
                    href=test_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(test_route.clone())
                    },
                >
                    { "Go to poll" }
                </a>
            </div>
        }
    }
}

impl NewPollsPage {
    fn view_poll_list(&self) -> Html<Self> {
        match &self.polls {
            Some(result) => match result {
                Ok(table) => {
                    if table.rows.is_empty() {
                        self.view_poll_list_empty()
                    } else {
                        self.view_poll_list_items(&table.rows)
                    }
                }
                Err(error) => self.view_poll_list_error(error),
            },
            None => self.view_poll_list_loading(),
        }
    }

    fn view_poll_list_loading(&self) -> Html<Self> {
        html! {
            <div class="poll_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_poll_list_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div class="poll_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_poll_list_items(&self, polls: &[Poll]) -> Html<Self> {
        html! {
            <ul class="poll_list_items", >
                { for polls.iter().map(|poll| self.view_poll_list_item(poll)) }
            </ul>
        }
    }

    fn view_poll_list_item(&self, poll: &Poll) -> Html<Self> {
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
                </a>
            </li>
        }
    }

    fn view_poll_list_empty(&self) -> Html<Self> {
        html! {
            <div class="poll_list_empty", >
                { "Empty" }
            </div>
        }
    }
}
