use components::{PollList, PollsOrder, PollsTable};
use context::Context;
use yew::prelude::*;

pub struct PopularPollsPage {
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for PopularPollsPage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PopularPollsPage {
            context: props.context,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PopularPollsPage> for PopularPollsPage {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="page", >
                <h1 class="page_title", >{ "Polls" }</h1>
                <div class="page_main", >
                    <PollList:
                        context=&self.context,
                        limit=Some(50),
                        table=Some(PollsTable::PopularPolls),
                        order=Some(PollsOrder::Popularity),
                    />
                </div>
            </div>
        }
    }
}
