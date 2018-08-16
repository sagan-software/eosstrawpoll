use components::{PollList, PollsOrder, PollsTable};
use context::Context;
use yew::prelude::*;

pub struct NewPollsPage {
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for NewPollsPage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NewPollsPage {
            context: props.context,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<NewPollsPage> for NewPollsPage {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "New Polls" }</h1>
                <PollList:
                    endpoint=&self.context.endpoint.to_string(),
                    limit=Some(50),
                    table=Some(PollsTable::NewPolls),
                    order=Some(PollsOrder::Created),
                />
            </div>
        }
    }
}
