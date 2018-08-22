use agents::tables::*;
use components::{Link, RelativeTime};
use context::Context;
use route::Route;
use types::Donation;
use yew::prelude::*;

pub struct DonationList {
    props: Props,
    donations: Option<Result<Vec<Donation>, String>>,
    tables: Box<Bridge<TablesAgent>>,
}

pub enum Msg {
    Tables(TablesOutput),
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub limit: Option<u32>,
}

impl Component for DonationList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tables_config = props.context.tables_config();
        let mut tables = TablesAgent::new(tables_config, link.send_back(Msg::Tables));
        tables.send(TablesInput::GetNewDonations);
        DonationList {
            props,
            donations: None,
            tables,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tables(output) => match output {
                TablesOutput::NewDonations(donations) => {
                    self.donations = Some(donations);
                    if let Some(Ok(ref mut donations)) = self.donations {
                        donations.sort_by(|a, b| b.created.cmp(&a.created));
                    }
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<DonationList> for DonationList {
    fn view(&self) -> Html<Self> {
        match &self.donations {
            Some(result) => match result {
                Ok(table) => {
                    if table.is_empty() {
                        self.view_empty()
                    } else {
                        self.view_items(&table)
                    }
                }
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl DonationList {
    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &str) -> Html<Self> {
        html! {
            <div class="donation_list_loading", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_items(&self, donations: &[Donation]) -> Html<Self> {
        html! {
            <ul class="donation_list_items", >
                { for donations.iter().map(|donation| self.view_item(donation)) }
            </ul>
        }
    }

    fn view_item(&self, donation: &Donation) -> Html<Self> {
        let donor_route = Route::Profile(donation.account.clone());
        let donated = donation.donated as f64;
        html! {
            <li class="donation_list_item", >
                <Link: class="donation_creator",
                    route=donor_route,
                    text=donation.account.clone(),
                />
                <div class="donation_donated", >
                    { format!("{:.*}", 4, donated / 10000.) } { " EOS" }
                </div>
                <RelativeTime: timestamp=donation.created, />
            </li>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="donation_list_empty", >
                { "Empty" }
            </div>
        }
    }
}
