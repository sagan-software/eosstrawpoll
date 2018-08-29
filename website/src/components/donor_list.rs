use agents::api::*;
use components::Link;
use context::Context;
use route::Route;
use std::cmp::min;
use types::Donor;
use yew::prelude::*;

pub struct DonorList {
    props: Props,
    donors: Option<Result<Vec<Donor>, String>>,
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

impl Component for DonorList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let api_config = props.context.api_config();
        let mut api = ApiAgent::new(api_config, link.send_back(Msg::Api));
        api.send(ApiInput::GetDonors);
        DonorList {
            props,
            donors: None,
            _api: api,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Api(output) => match output {
                ApiOutput::Donors(donors) => {
                    self.donors = Some(donors);
                    if let Some(Ok(ref mut donors)) = self.donors {
                        donors.sort_by(|a, b| b.donated.cmp(&a.donated));
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

impl Renderable<DonorList> for DonorList {
    fn view(&self) -> Html<Self> {
        match &self.donors {
            Some(result) => match result {
                Ok(table) => {
                    if table.is_empty() {
                        self.view_empty()
                    } else {
                        self.view_loaded(&table)
                    }
                }
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl DonorList {
    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donor_list -loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &str) -> Html<Self> {
        html! {
            <div class="donor_list -error", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_loaded(&self, donors: &[Donor]) -> Html<Self> {
        let limit = min(donors.len(), self.props.limit.unwrap_or_else(|| 20));
        html! {
            <ul class="donor_list -loaded", >
                { for donors[0..limit].iter().enumerate().map(|(i, donor)| self.view_item(i + 1, donor)) }
            </ul>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="donor_list -empty", >
                { "Empty" }
            </div>
        }
    }

    fn view_item(&self, rank: usize, donor: &Donor) -> Html<Self> {
        let donor_route = Route::Profile("cf057bbfb726".into(), donor.account.clone());
        let donated = donor.donated as f64;
        html! {
            <li class="donor_list_item", >
                <div class="donor_rank", >
                    { rank }
                </div>
                <Link: class="donor_account",
                    route=donor_route,
                    text=donor.account.clone(),
                />
                <div class="donor_donated", >
                    { format!("{:.*}", 4, donated / 10000.) } { " EOS" }
                </div>
            </li>
        }
    }
}
