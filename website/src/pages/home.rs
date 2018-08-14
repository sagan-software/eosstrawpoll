use agents::router::{RouterAgent, RouterInput, RouterOutput};
use components::PollForm;
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Poll;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct HomePage {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Context,
    eos: EosService,
    popular_polls_task: Option<FetchTask>,
    popular_polls: Option<Result<eos::TableRows<Poll>, Error>>,
    new_polls_task: Option<FetchTask>,
    new_polls: Option<Result<eos::TableRows<Poll>, Error>>,
}

pub enum Msg {
    Router(RouterOutput<()>),
    NavigateTo(Route),
    PopularPolls(Result<eos::TableRows<Poll>, Error>),
    NewPolls(Result<eos::TableRows<Poll>, Error>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);

        let mut eos = EosService::new();
        let mut params = eos::TableRowsParams {
            scope: "eosstrawpoll".to_string(),
            code: "eosstrawpoll".to_string(),
            table: "newpolls".to_string(),
            json: true,
            lower_bound: None,
            upper_bound: None,
            limit: Some(10),
            key_type: None,
            index_position: None,
        };
        let context = props.context;
        let callback = link.send_back(Msg::NewPolls);
        let new_polls_task =
            eos.get_table_rows(context.endpoint.as_str(), params.clone(), callback);

        params.table = "popularpolls".to_string();
        let callback = link.send_back(Msg::PopularPolls);
        let popular_polls_task =
            eos.get_table_rows(context.endpoint.as_str(), params.clone(), callback);

        HomePage {
            router,
            context,
            eos: EosService::new(),
            popular_polls_task: Some(popular_polls_task),
            popular_polls: None,
            new_polls_task: Some(new_polls_task),
            new_polls: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
            }
            Msg::PopularPolls(result) => {
                self.popular_polls = Some(result);
                self.popular_polls_task = None;
                true
            }
            Msg::NewPolls(result) => {
                self.new_polls = Some(result);
                self.new_polls_task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        true
    }
}

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="home_page", >
                <header class="page__header app__container", >
                    <h1 class="page__title", >
                        { "Create real-time polls on EOS blockchains" }
                    </h1>
                </header>
                <div class="app__container", >
                    <PollForm: context=&self.context, />
                </div>
                <div class="app__container", >
                    { self.view_popular_polls() }
                    { self.view_new_polls() }
                </div>
            </div>
        }
    }
}

impl HomePage {
    fn view_popular_polls(&self) -> Html<Self> {
        self.view_poll_list("Popular", &self.popular_polls)
    }

    fn view_new_polls(&self) -> Html<Self> {
        self.view_poll_list("New", &self.new_polls)
    }

    fn view_poll_list(
        &self,
        title: &str,
        polls: &Option<Result<eos::TableRows<Poll>, Error>>,
    ) -> Html<Self> {
        let inner = match polls {
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
        };
        html! {
            <div class="poll_list", >
                <h2> { title } </h2>
                { inner }
            </div>
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
