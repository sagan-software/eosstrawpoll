use components::{DonationList, DonorList};
use context::Context;
use traits::Page;
use yew::prelude::*;

pub struct DonationsPage {
    context: Context,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

impl Component for DonationsPage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        DonationsPage {
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

impl Page for DonationsPage {
    fn title(&self) -> String {
        "Donations".to_string()
    }
    fn class(&self) -> String {
        "donations_page".to_string()
    }
    fn content(&self) -> Html<Self> {
        html! {
            <>
                <DonorList: context=&self.context, />
                <DonationList: context=&self.context, />
            </>
        }
    }
}

page_view! { DonationsPage }
