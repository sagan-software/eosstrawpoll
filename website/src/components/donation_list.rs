use agents::api::*;
use components::Link;
use context::Context;
use route::Route;
use std::cmp::min;
use types::Donation;
use yew::prelude::*;

pub struct DonationList {
    props: Props,
    donations: Option<Result<Vec<Donation>, String>>,
    _api: Box<Bridge<ApiAgent>>,
}

pub enum Msg {
    Api(ApiOutput),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<usize>,
}

impl Component for DonationList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let api_config = props.context.api_config();
        let mut api = ApiAgent::new(api_config, link.send_back(Msg::Api));
        api.send(ApiInput::GetNewDonations);
        DonationList {
            props,
            donations: None,
            _api: api,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Api(output) => match output {
                ApiOutput::NewDonations(donations) => {
                    self.donations = Some(donations);
                    if let Some(Ok(ref mut donations)) = self.donations {
                        donations.sort_by(|a, b| b.created.cmp(&a.created));
                    }
                    true
                }
                _ => false,
            },
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
                    if table.is_empty() {
                        self.view_empty()
                    } else {
                        self.view_items(&table)
                    }
                }
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl DonationList {
    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &str) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_items(&self, donations: &[Donation]) -> Html<Self> {
        let limit = min(donations.len(), self.props.limit.unwrap_or_else(|| 10));
        html! {
            <ul class="donation_list_items", >
                { for donations[0..limit].iter().map(|donation| self.view_item(donation)) }
            </ul>
        }
    }

    fn view_item(&self, donation: &Donation) -> Html<Self> {
        let donor_route = Route::Profile("cf057bbfb726".into(), donation.account.clone());
        let donated = donation.donated as f64;
        html! {
            <li class="donation_list_item", >
                <Link: class="donation_account",
                    route=donor_route,
                    text=donation.account.clone(),
                />
                <div class="donation_donated", >
                    { format!("{:.*}", 4, donated / 10000.) } { " EOS" }
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
