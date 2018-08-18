use components::*;
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
            <div class="home_page app_container", >
                <h1>
                    { "Create real-time polls on EOS blockchains" }
                </h1>
                <div class="poll_form_wrapper", >
                    <PollForm: context=&self.context, />
                </div>
                <aside class="poll_lists", >
                    <div class="popular_polls", >
                        <h2> { "Popular Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(10),
                            table=Some(PollsTable::PopularPolls),
                            order=Some(PollsOrder::Popularity),
                        />
                    </div>
                    <div class="new_polls", >
                        <h2> { "New Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(10),
                            table=Some(PollsTable::NewPolls),
                            order=Some(PollsOrder::Created),
                        />
                    </div>
                </aside>
                <aside class="donations", >
                    <div class="top_donors", >
                        <h2> { "Top Donors" } </h2>
                        <DonorList: context=&self.context, />
                    </div>
                    <div class="new_donations", >
                        <h2> { "New Donations" } </h2>
                        <DonationList: context=&self.context, />
                    </div>
                    <DonationForm: context=&self.context, />
                </aside>
            </div>
        }
    }
}
