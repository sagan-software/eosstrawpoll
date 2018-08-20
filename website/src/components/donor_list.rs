use agents::router::{RouterAgent, RouterInput, RouterOutput};
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Donor;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct DonorList {
    props: Props,
    link: ComponentLink<DonorList>,
    router: Box<Bridge<RouterAgent<()>>>,
    eos: EosService,
    task: Option<FetchTask>,
    donors: Option<Result<eos::TableRows<Donor>, Error>>,
}

pub enum Msg {
    Router(RouterOutput<()>),
    NavigateTo(Route),
    Donors(Result<eos::TableRows<Donor>, Error>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<u32>,
}

impl Component for DonorList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);
        let mut donor_list = DonorList {
            props,
            link,
            router,
            eos: EosService::new(),
            task: None,
            donors: None,
        };
        donor_list.fetch_donors();
        donor_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
            }
            Msg::Donors(result) => {
                self.donors = Some(result);
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

impl Renderable<DonorList> for DonorList {
    fn view(&self) -> Html<Self> {
        match &self.donors {
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

impl DonorList {
    fn fetch_donors(&mut self) {
        let mut params = eos::TableRowsParams {
            json: true,
            scope: "eosstrawpoll".to_string(),
            code: "eosstrawpoll".to_string(),
            table: "donors".to_string(),
            lower_bound: self.props.lower_bound.clone(),
            upper_bound: self.props.upper_bound.clone(),
            limit: self.props.limit,
            key_type: None,
            index_position: None,
        };
        let callback = self.link.send_back(Msg::Donors);
        let endpoint = &self.props.context.endpoint;
        let task = self.eos.get_table_rows(endpoint, params, callback);
        self.task = Some(task);
    }

    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donor_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div class="donor_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_items(&self, donors: &[Donor]) -> Html<Self> {
        html! {
            <ul class="donor_list_items", >
                { for donors.iter().map(|donor| self.view_item(donor)) }
            </ul>
        }
    }

    fn view_item(&self, donor: &Donor) -> Html<Self> {
        let donor_route = Route::Profile(donor.account.clone());
        html! {
            <li class="donor_list_item", >
                <a class="donor_creator",
                    href=donor_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(donor_route.clone())
                    },
                >
                    { &donor.account }
                </a>
                <div class="donor_donated", >
                    { &donor.donated } { " EOS" }
                </div>
            </li>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="donor_list_empty", >
                { "Empty" }
            </div>
        }
    }
}
