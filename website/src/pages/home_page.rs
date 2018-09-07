use components::*;
use prelude::*;

pub struct HomePage {
    context: Context,
    chain: Chain,
    props: Props,
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
        let context = props.context.clone();
        let chain = props
            .chain
            .clone()
            .unwrap_or_else(|| context.selected_chain.clone());
        HomePage {
            context,
            chain,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context.clone();
        self.chain = props
            .chain
            .clone()
            .unwrap_or_else(|| self.context.selected_chain.clone());
        self.props = props;
        true
    }
}

impl Page for HomePage {
    fn get_title(&self) -> String {
        let suffix = match &self.props.chain {
            Some(chain) => chain.long_name.clone(),
            None => "EOS blockchains".to_string(),
        };
        format!("Real-time polls on {}", suffix)
    }
    fn get_class(&self) -> String {
        "home_page".to_string()
    }
    fn get_state(&self) -> PageState {
        PageState::Loaded
    }
    fn get_route(&self) -> Route {
        match &self.props.chain {
            Some(chain) => Route::Home(Some(chain.to_chain_id_prefix())),
            None => Route::Home(None),
        }
    }
    fn get_description(&self) -> String {
        // TODO
        self.get_title()
    }
    fn get_content(&self) -> Html<Self> {
        html! {
            <>
                <div class="poll_form_wrapper", >
                    <PollForm: context=&self.context, chain=&self.chain, />
                </div>
                <aside class="polls", >
                    <div class="popular_polls", >
                        <h2> { "Popular Polls" } </h2>
                        <PollTeaseList:
                            context=&self.context,
                            limit=Some(10),
                            table=PollsTable::PopularPolls,
                            order=Some(PollsOrder::Popularity),
                            chain=&self.chain,
                        />
                    </div>
                    <div class="new_polls", >
                        <h2> { "New Polls" } </h2>
                        <PollTeaseList:
                            context=&self.context,
                            limit=Some(5),
                            table=PollsTable::NewPolls,
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
