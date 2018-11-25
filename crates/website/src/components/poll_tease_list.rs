use contract::PollTease;
use crate::chains::Chain;
use crate::components::{Link, RelativeTime};
use crate::context::Context;
use crate::eos::*;
use crate::prelude::*;
use crate::views::svg;
use std::cmp::min;

pub struct PollTeaseList {
    props: Props,
    polls: EosData<Vec<PollTease>>,
    eos_agent: Box<Bridge<EosAgent>>,
}

#[derive(PartialEq, Clone)]
pub enum PollsTable {
    PopularPolls,
    NewPolls,
}

impl Default for PollsTable {
    fn default() -> PollsTable {
        PollsTable::PopularPolls
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
    Chain(EosOutput),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub scope: String,
    pub table: PollsTable,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<usize>,
    pub order: Option<PollsOrder>,
    pub chain: Chain,
    pub detailed: bool,
}

impl Component for PollTeaseList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let eos_agent = EosAgent::new(props.chain.clone(), link.send_back(Msg::Chain));
        let mut poll_tease_list = PollTeaseList {
            props,
            polls: EosData::default(),
            eos_agent,
        };
        poll_tease_list.request_table();
        poll_tease_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(output) => match (&self.props.table, output) {
                (PollsTable::NewPolls, EosOutput::NewPolls(polls)) => {
                    debug!("got new polls: {:#?}", polls);
                    self.polls = polls;
                    true
                }
                (PollsTable::PopularPolls, EosOutput::PopularPolls(polls)) => {
                    debug!("got popular polls: {:#?}", polls);
                    self.polls = polls;
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            let chain = props.chain.clone();
            self.polls = EosData::Loading;
            self.props = props;
            self.eos_agent.send(EosInput::Configure(chain));
            self.request_table();
            true
        } else {
            false
        }
    }
}

impl Renderable<PollTeaseList> for PollTeaseList {
    fn view(&self) -> Html<Self> {
        match &self.polls {
            EosData::NotAsked => self.view_empty(),
            EosData::Loading => self.view_loading(),
            EosData::Success(data) => {
                if data.is_empty() {
                    self.view_empty()
                } else {
                    self.view_items(&data)
                }
            }
            EosData::Failure(error) => self.view_error(error),
        }
    }
}

impl PollTeaseList {
    fn request_table(&mut self) {
        match &self.props.table {
            PollsTable::NewPolls => {
                self.eos_agent.send(EosInput::GetNewPolls);
            }
            PollsTable::PopularPolls => {
                self.eos_agent.send(EosInput::GetPopularPolls);
            }
        };
    }

    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="poll_list -loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &EosError) -> Html<Self> {
        error!("POLL TEASE LIST ERROR: {}", error);
        html! {
            <div class="poll_list -error", >
                { svg::link_cross() }
            </div>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="poll_list -empty", >
                { svg::eos() }
            </div>
        }
    }

    fn view_items(&self, polls: &[PollTease]) -> Html<Self> {
        let limit = min(polls.len(), self.props.limit.unwrap_or_else(|| 20));
        html! {
            <ul class="poll_list -loaded", >
                { for polls[0..limit].iter().map(|poll| self.view_item(poll)) }
            </ul>
        }
    }

    fn view_item(&self, poll: &PollTease) -> Html<Self> {
        let poll_route = Route::PollVoting(self.props.chain.to_chain_id_prefix(), poll.id.clone());
        let account_route =
            Route::Profile(self.props.chain.to_chain_id_prefix(), poll.account.clone());
        html! {
            <li class="poll", >
                <Link: class="poll_title",
                    route=poll_route,
                    text=poll.title.clone(),
                />
                <div class="poll_details", >
                    <Link: class="poll_account",
                        route=account_route,
                        text=poll.account.to_string(),
                    />
                    <div class="poll_create_time", >
                        <RelativeTime: timestamp=poll.create_time.milliseconds(), />
                    </div>
                    <div class="poll_votes", >
                        { &poll.num_votes } { " vote" }{ if poll.num_votes == 1 { "" } else { "s" } }
                    </div>
                </div>
            </li>
        }
    }
}
