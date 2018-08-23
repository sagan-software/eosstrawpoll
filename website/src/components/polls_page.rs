use components::{PollList, PollsOrder, PollsTable};
use context::Context;
use traits::Page;
use yew::prelude::*;

pub struct PollsPage {
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for PollsPage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PollsPage {
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

impl Page for PollsPage {
    fn title(&self) -> String {
        "Polls".to_string()
    }
    fn class(&self) -> String {
        "polls_page".to_string()
    }
    fn content(&self) -> Html<Self> {
        html! {
            <PollList:
                context=&self.context,
                limit=Some(50),
                table=Some(PollsTable::PopularPolls),
                order=Some(PollsOrder::Popularity),
            />
        }
    }
}

page_view! { PollsPage }
