use agents::chain::*;
use components::{Link, RelativeTime};
use context::Context;
use prelude::*;
use route::Route;
use std::cmp::min;

pub struct PollList {
    props: Props,
    polls: ChainData<Vec<Poll>>,
    table: PollsTable,
    _chain_agent: Box<Bridge<ChainAgent>>,
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
    Chain(ChainOutput),
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
}

impl Component for PollList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut chain_agent = ChainAgent::new(props.chain.clone(), link.send_back(Msg::Chain));

        let table = props.clone().table.unwrap_or_else(|| PollsTable::Polls);

        if table == PollsTable::NewPolls {
            chain_agent.send(ChainInput::GetNewPolls);
        } else if table == PollsTable::PopularPolls {
            chain_agent.send(ChainInput::GetPopularPolls);
        } else {
            chain_agent.send(ChainInput::GetPolls(props.scope.clone()));
        }

        PollList {
            props,
            polls: ChainData::default(),
            table,
            _chain_agent: chain_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(output) => match (&self.table, output) {
                (PollsTable::NewPolls, ChainOutput::NewPolls(polls)) => {
                    self.polls = polls;
                    true
                }
                (PollsTable::PopularPolls, ChainOutput::PopularPolls(polls)) => {
                    self.polls = polls;
                    true
                }
                (PollsTable::Polls, ChainOutput::Polls(scope, polls)) => {
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
        self.props = props;
        true
    }
}

impl Renderable<PollList> for PollList {
    fn view(&self) -> Html<Self> {
        match &self.polls {
            ChainData::NotAsked => self.view_empty(),
            ChainData::Loading => self.view_loading(),
            ChainData::Success(data) => {
                if data.is_empty() {
                    self.view_empty()
                } else {
                    self.view_items(&data)
                }
            }
            ChainData::Failure(error) => self.view_error(error),
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

    fn view_error(&self, error: &ChainError) -> Html<Self> {
        html! {
            <div class="poll_list -error", >
                { "Error: " }{ format!("{:#?}", error) }
            </div>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="poll_list -empty", >
                { "Empty" }
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
            "cf057bbfb726".into(),
            poll.creator.clone(),
            poll.slug.clone(),
        );
        let creator_route = Route::Profile("cf057bbfb726".into(), poll.creator.clone());
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
