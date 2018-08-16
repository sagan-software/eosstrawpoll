use components::PollList;
use context::Context;
use yew::prelude::*;

pub struct ProfilePage {
    account: String,
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub account: String,
}

impl Component for ProfilePage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let account = props.account;
        let context = props.context;
        ProfilePage { account, context }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<ProfilePage> for ProfilePage {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ &self.account }</h1>
                <PollList:
                    endpoint=&self.context.endpoint,
                    scope=self.account.clone(),
                    limit=Some(50),
                />
            </div>
        }
    }
}
