use components::PollList;
use eos::*;
use prelude::*;
use stdweb::web::document;

pub struct ProfilePage {
    props: Props,
    eos_agent: Box<Bridge<EosAgent>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub chain: Chain,
    pub account: String,
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
        let eos_agent = EosAgent::new(chain, eos_callback);
        ProfilePage { props, eos_agent }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Eos(output) => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
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
        html! {
            <>
                <PollList:
                    context=&self.props.context,
                    scope=&self.props.account,
                    limit=Some(50),
                    chain=&self.props.chain,
                />
            </>
        }
    }
}

page_view! { ProfilePage }
