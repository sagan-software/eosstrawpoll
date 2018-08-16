use components::{PollForm, PollList, PollsOrder, PollsTable};
use context::Context;
use yew::prelude::*;

pub struct HomePage {
    context: Context,
}

pub enum Msg {}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomePage {
            context: props.context,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        true
    }
}

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="home_page", >
                <h1 class="page_title", >
                    { "Create real-time polls on EOS blockchains" }
                </h1>
                <div class="poll_form_wrapper", >
                    <PollForm: context=&self.context, />
                </div>
                <div class="app_container", >
                    { self.view_popular_polls() }
                    { self.view_new_polls() }
                </div>
            </div>
        }
    }
}

impl HomePage {
    fn view_popular_polls(&self) -> Html<Self> {
        html! {
            <div class="poll_list -popular", >
                <h2> { "Popular" } </h2>
                <PollList:
                    endpoint=&self.context.endpoint.to_string(),
                    limit=Some(10),
                    table=Some(PollsTable::PopularPolls),
                    order=Some(PollsOrder::Popularity),
                />
            </div>
        }
    }

    fn view_new_polls(&self) -> Html<Self> {
        html! {
            <div class="poll_list -new", >
                <h2> { "New" } </h2>
                <PollList:
                    endpoint=&self.context.endpoint.to_string(),
                    limit=Some(10),
                    table=Some(PollsTable::NewPolls),
                    order=Some(PollsOrder::Created),
                />
            </div>
        }
    }
}
