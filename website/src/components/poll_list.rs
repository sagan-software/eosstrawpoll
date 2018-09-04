use components::{Link, RelativeTime};
use eos::*;
use prelude::*;
use std::cmp::min;
use views::svg;

pub struct PollList {
    props: Props,
    polls: EosData<Vec<Poll>>,
    table: PollsTable,
    eos_agent: Box<Bridge<EosAgent>>,
}

#[derive(PartialEq, Clone)]
pub enum PollsTable {
    Polls,
    PopularPolls,
    NewPolls,
}

impl Default for PollsTable {
    fn default() -> PollsTable {
        PollsTable::Polls
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
    pub table: Option<PollsTable>,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<usize>,
    pub order: Option<PollsOrder>,
    pub chain: Chain,
    pub detailed: bool,
}

impl Component for PollList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut eos_agent = EosAgent::new(props.chain.clone(), link.send_back(Msg::Chain));

        let table = props.clone().table.unwrap_or_else(|| PollsTable::Polls);

        if table == PollsTable::NewPolls {
            eos_agent.send(EosInput::GetNewPolls);
        } else if table == PollsTable::PopularPolls {
            eos_agent.send(EosInput::GetPopularPolls);
        } else {
            eos_agent.send(EosInput::GetPolls(props.scope.clone()));
        }

        PollList {
            props,
            polls: EosData::default(),
            table,
            eos_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(output) => match (&self.table, output) {
                (PollsTable::NewPolls, EosOutput::NewPolls(polls)) => {
                    self.polls = polls;
                    true
                }
                (PollsTable::PopularPolls, EosOutput::PopularPolls(polls)) => {
                    self.polls = polls;
                    true
                }
                (PollsTable::Polls, EosOutput::Polls(scope, polls)) => {
                    if scope == self.props.scope {
                        self.polls = polls;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let chain = props.chain.clone();
        self.polls = EosData::Loading;
        self.props = props;
        self.eos_agent.send(EosInput::Configure(chain));

        let table = self
            .props
            .clone()
            .table
            .unwrap_or_else(|| PollsTable::Polls);
        if table == PollsTable::NewPolls {
            self.eos_agent.send(EosInput::GetNewPolls);
        } else if table == PollsTable::PopularPolls {
            self.eos_agent.send(EosInput::GetPopularPolls);
        } else {
            self.eos_agent
                .send(EosInput::GetPolls(self.props.scope.clone()));
        }

        true
    }
}

impl Renderable<PollList> for PollList {
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

impl PollList {
    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="poll_list -loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &EosError) -> Html<Self> {
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

    fn view_items(&self, polls: &[Poll]) -> Html<Self> {
        let limit = min(polls.len(), self.props.limit.unwrap_or_else(|| 20));
        html! {
            <ul class="poll_list -loaded", >
                { for polls[0..limit].iter().map(|poll| self.view_item(poll)) }
            </ul>
        }
    }

    fn view_item(&self, poll: &Poll) -> Html<Self> {
        let poll_route = Route::Poll(
            self.props.chain.to_chain_id_prefix(),
            poll.creator.clone(),
            poll.slug.clone(),
        );
        let creator_route =
            Route::Profile(self.props.chain.to_chain_id_prefix(), poll.creator.clone());
        html! {
            <li class="poll", >
                <Link: class="poll_title",
                    route=poll_route,
                    text=poll.title.clone(),
                />
                <div class="poll_details", >
                    <Link: class="poll_creator",
                        route=creator_route,
                        text=poll.creator.clone(),
                    />
                    <div class="poll_create_time", >
                        <RelativeTime: timestamp=poll.create_time, />
                    </div>
                    <div class="poll_votes", >
                        { &poll.votes.len() } { " vote" }{ if &poll.votes.len() == &1 { "" } else { "s" } }
                    </div>
                </div>
            </li>
        }
    }
}
