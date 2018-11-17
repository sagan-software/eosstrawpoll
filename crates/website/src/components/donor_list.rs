use contract::Donor;
use crate::components::Link;
use crate::eos::*;
use crate::prelude::*;
use crate::views::svg;
use std::cmp::min;

pub struct DonorList {
    props: Props,
    donors: EosData<Vec<Donor>>,
    eos_agent: Box<Bridge<EosAgent>>,
}

pub enum Msg {
    Chain(EosOutput),
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
        let mut eos_agent = EosAgent::new(props.chain.clone(), link.send_back(Msg::Chain));
        eos_agent.send(EosInput::GetDonors);
        DonorList {
            props,
            donors: EosData::default(),
            eos_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(output) => match output {
                EosOutput::Donors(donors) => {
                    self.donors = donors;
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let chain = props.chain.clone();
        self.donors = EosData::Loading;
        self.props = props;
        self.eos_agent.send(EosInput::Configure(chain));
        self.eos_agent.send(EosInput::GetDonors);
        true
    }
}

impl Renderable<DonorList> for DonorList {
    fn view(&self) -> Html<Self> {
        match &self.donors {
            EosData::NotAsked => self.view_empty(),
            EosData::Loading => self.view_loading(),
            EosData::Success(data) => {
                if data.is_empty() {
                    self.view_empty()
                } else {
                    self.view_loaded(&data)
                }
            }
            EosData::Failure(error) => self.view_error(error),
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

    fn view_error(&self, _error: &EosError) -> Html<Self> {
        html! {
            <div class="donor_list -error", >
                { svg::link_cross() }
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
                { svg::eos() }
            </div>
        }
    }

    fn view_item(&self, rank: usize, donor: &Donor) -> Html<Self> {
        let chain_id_prefix = self.props.chain.to_chain_id_prefix();
        let donor_route = Route::Profile(chain_id_prefix, donor.account.clone());
        let donated = donor.donated as f64;
        html! {
            <li class="donor_list_item", >
                <div class="donor_rank", >
                    { rank }
                </div>
                <Link: class="donor_account",
                    route=donor_route,
                    text=donor.account.to_string(),
                />
                <div class="donor_donated", >
                    { format!("{:.*} {}", 4, donated / 10000., self.props.chain.core_symbol.name().to_string()) }
                </div>
            </li>
        }
    }
}
