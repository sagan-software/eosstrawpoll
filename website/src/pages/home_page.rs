use components::*;
use prelude::*;
use stdweb::web::document;

pub struct HomePage {
    context: Context,
    chain: Chain,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub chain: Option<Chain>,
}

impl Component for HomePage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let context = props.context;
        let chain = props
            .chain
            .unwrap_or_else(|| context.selected_chain.clone());
        HomePage { context, chain }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        self.chain = props
            .chain
            .unwrap_or_else(|| self.context.selected_chain.clone());
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
                    <PollForm: context=&self.context, chain=&self.chain, />
                </div>
                <aside class="polls", >
                    <div class="popular_polls", >
                        <h2> { "Popular Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(10),
                            table=Some(PollsTable::PopularPolls),
                            order=Some(PollsOrder::Popularity),
                            chain=&self.chain,
                        />
                    </div>
                    <div class="new_polls", >
                        <h2> { "New Polls" } </h2>
                        <PollList:
                            context=&self.context,
                            limit=Some(5),
                            table=Some(PollsTable::NewPolls),
                            order=Some(PollsOrder::Created),
                            chain=&self.chain,
                        />
                    </div>
                </aside>
                <aside class="donations", >
                    <div class="top_donors", >
                        <h2> { "Top Donors" } </h2>
                        <DonorList: context=&self.context, chain=&self.chain, />
                    </div>
                    <div class="new_donations", >
                        <h2> { "New Donations" } </h2>
                        <DonationList: context=&self.context, chain=&self.chain, />
                    </div>
                    <DonationForm: context=&self.context, chain=&self.chain, />
                </aside>
            </>
        }
    }
}

page_view! { HomePage }
