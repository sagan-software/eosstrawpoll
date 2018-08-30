use agents::chain::*;
use components::Link;
use context::Context;
use prelude::*;
use route::Route;
use std::cmp::min;

pub struct DonorList {
    props: Props,
    donors: ChainData<Vec<Donor>>,
    _chain_agent: Box<Bridge<ChainAgent>>,
}

pub enum Msg {
    Chain(ChainOutput),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<usize>,
    pub chain: Chain,
}

impl Component for DonorList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut chain_agent = ChainAgent::new(props.chain.clone(), link.send_back(Msg::Chain));
        chain_agent.send(ChainInput::GetDonors);
        DonorList {
            props,
            donors: ChainData::default(),
            _chain_agent: chain_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(output) => match output {
                ChainOutput::Donors(donors) => {
                    self.donors = donors;
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
            ChainData::NotAsked => self.view_empty(),
            ChainData::Loading => self.view_loading(),
            ChainData::Success(data) => {
                if data.is_empty() {
                    self.view_empty()
                } else {
                    self.view_loaded(&data)
                }
            }
            ChainData::Failure(error) => self.view_error(error),
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

    fn view_error(&self, error: &ChainError) -> Html<Self> {
        html! {
            <div class="donor_list -error", >
                { "Error: " }{ format!("{:#?}", error) }
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
                    { format!("{:.*} {}", 4, donated / 10000., self.props.chain.core_symbol) }
                </div>
            </li>
        }
    }
}
