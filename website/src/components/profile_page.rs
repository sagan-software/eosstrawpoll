use components::PollList;
use prelude::*;
use stdweb::web::document;

#[derive(PartialEq, Clone, Default)]
pub struct ProfilePage {
    pub context: Context,
    pub chain: Chain,
    pub account: String,
}

impl Component for ProfilePage {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        self.chain = props.chain;
        self.account = props.account;
        true
    }
}

impl Page for ProfilePage {
    fn title(&self) -> String {
        self.account.clone()
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
                    context=&self.context,
                    scope=self.account.clone(),
                    limit=Some(50),
                    chain=&self.chain,
                />
            </>
        }
    }
}

page_view! { ProfilePage }
