use agents::router::{RouterAgent, RouterInput, RouterOutput};
use context::Context;
use failure::Error;
use route::Route;
use services::eos::{self, EosService};
use stdweb::traits::IEvent;
use types::Donation;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct DonationList {
    props: Props,
    link: ComponentLink<DonationList>,
    router: Box<Bridge<RouterAgent<()>>>,
    eos: EosService,
    task: Option<FetchTask>,
    donations: Option<Result<eos::TableRows<Donation>, Error>>,
}

pub enum Msg {
    Router(RouterOutput<()>),
    NavigateTo(Route),
    Donations(Result<eos::TableRows<Donation>, Error>),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<u32>,
}

impl Component for DonationList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Router);
        let router = RouterAgent::bridge(callback);
        let mut donation_list = DonationList {
            props,
            link,
            router,
            eos: EosService::new(),
            task: None,
            donations: None,
        };
        donation_list.fetch_donations();
        donation_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_output) => true,
            Msg::NavigateTo(route) => {
                let url = route.to_string();
                self.router.send(RouterInput::ChangeRoute(url, ()));
                false
            }
            Msg::Donations(result) => {
                self.donations = Some(result);
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

impl Renderable<DonationList> for DonationList {
    fn view(&self) -> Html<Self> {
        match &self.donations {
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

impl DonationList {
    fn fetch_donations(&mut self) {
        let mut params = eos::TableRowsParams {
            json: true,
            scope: "eosstrawpoll".to_string(),
            code: "eosstrawpoll".to_string(),
            table: "newdonations".to_string(),
            lower_bound: self.props.lower_bound.clone(),
            upper_bound: self.props.upper_bound.clone(),
            limit: self.props.limit,
            key_type: None,
            index_position: None,
        };
        let callback = self.link.send_back(Msg::Donations);
        let endpoint = &self.props.context.endpoint;
        let task = self.eos.get_table_rows(endpoint, params, callback);
        self.task = Some(task);
    }

    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &Error) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_items(&self, donations: &[Donation]) -> Html<Self> {
        html! {
            <ul class="donation_list_items", >
                { for donations.iter().map(|donation| self.view_item(donation)) }
            </ul>
        }
    }

    fn view_item(&self, donation: &Donation) -> Html<Self> {
        let donor_route = Route::Profile(donation.account.clone());
        html! {
            <li class="donation_list_item", >
                <a class="donation_creator",
                    href=donor_route.to_string(),
                    onclick=|e| {
                        e.prevent_default();
                        Msg::NavigateTo(donor_route.clone())
                    },
                >
                    { &donation.account }
                </a>
                <div class="donation_donated", >
                    { &donation.donated / 1000. } { " EOS" }
                </div>
            </li>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="donation_list_empty", >
                { "Empty" }
            </div>
        }
    }
}
