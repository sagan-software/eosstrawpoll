use components::*;
use context::Context;
use prelude::*;
use stdweb::web::document;
use traits::{Page, PageState};

pub struct HomePage {
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for HomePage {
    type Message = ();
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

impl Page for HomePage {
    fn title(&self) -> String {
        "Real-time polls on EOS blockchains".to_string()
    }
    fn class(&self) -> String {
        "home_page".to_string()
    }
    fn get_state(&self) -> PageState {
        PageState::Loaded
    }
    fn content(&self) -> Html<Self> {
        html! {
            <>
                <div class="poll_form_wrapper", >
                    <PollForm: context=&self.context, chain=eos_devnet(), />
                </div>
                <aside class="polls", >
                    <div class="popular_polls", >
                        <h2> { "Popular Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(10),
                            table=Some(PollsTable::PopularPolls),
                            order=Some(PollsOrder::Popularity),
                            chain=eos_devnet(),
                        />
                    </div>
                    <div class="new_polls", >
                        <h2> { "New Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(5),
                            table=Some(PollsTable::NewPolls),
                            order=Some(PollsOrder::Created),
                            chain=eos_devnet(),
                        />
                    </div>
                </aside>
                <aside class="donations", >
                    <div class="top_donors", >
                        <h2> { "Top Donors" } </h2>
                        <DonorList: context=&self.context, chain=eos_devnet(), />
                    </div>
                    <div class="new_donations", >
                        <h2> { "New Donations" } </h2>
                        <DonationList: context=&self.context, chain=eos_devnet(), />
                    </div>
                    <DonationForm: context=&self.context, chain=eos_devnet(), />
                </aside>
            </>
        }
    }
}

page_view! { HomePage }
