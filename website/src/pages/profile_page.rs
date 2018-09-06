use components::{Link, RelativeTime};
use eos::*;
use prelude::*;
use stdweb::web::document;
use views::svg;

pub struct ProfilePage {
    props: Props,
    polls: EosData<Vec<Poll>>,
    eos_agent: Box<Bridge<EosAgent>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub chain: Chain,
    pub account: AccountName,
}

pub enum Msg {
    Eos(EosOutput),
}

impl Component for ProfilePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let chain = props.chain.clone();
        let eos_callback = link.send_back(Msg::Eos);
        let mut eos_agent = EosAgent::new(chain, eos_callback);
        eos_agent.send(EosInput::GetPolls(props.account.clone()));
        ProfilePage {
            props,
            eos_agent,
            polls: EosData::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Eos(output) => match output {
                EosOutput::Polls(account, data) => {
                    if account == self.props.account {
                        self.polls = data;
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

impl ProfilePage {
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
        html! {
            <ul class="poll_list -loaded", >
                { for polls.iter().map(|poll| self.view_item(poll)) }
            </ul>
        }
    }

    fn view_item(&self, poll: &Poll) -> Html<Self> {
        let poll_route = Route::PollVoting(self.props.chain.to_chain_id_prefix(), poll.id.clone());
        html! {
            <li class="poll", >
                <Link: class="poll_title",
                    route=poll_route,
                    text=poll.title.clone(),
                />
                <div class="poll_details", >
                    <div class="poll_create_time", >
                        { "Submitted " }
                        <RelativeTime: timestamp=poll.create_time, />
                    </div>
                </div>
            </li>
        }
    }
}

impl Page for ProfilePage {
    fn title(&self) -> String {
        self.props.account.clone()
    }
    fn class(&self) -> String {
        "profile_page".to_string()
    }
    fn get_state(&self) -> PageState {
        PageState::Loaded
    }
    fn content(&self) -> Html<Self> {
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

page_view! { ProfilePage }
