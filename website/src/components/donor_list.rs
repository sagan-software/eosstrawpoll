use agents::tables::*;
use components::Link;
use context::Context;
use route::Route;
use types::Donor;
use yew::prelude::*;

pub struct DonorList {
    props: Props,
    donors: Option<Result<Vec<Donor>, String>>,
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

impl Component for DonorList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tables_config = props.context.tables_config();
        let mut tables = TablesAgent::new(tables_config, link.send_back(Msg::Tables));
        tables.send(TablesInput::GetDonors);
        DonorList {
            props,
            donors: None,
            tables,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tables(output) => match output {
                TablesOutput::Donors(donors) => {
                    self.donors = Some(donors);
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

impl Renderable<DonorList> for DonorList {
    fn view(&self) -> Html<Self> {
        match &self.donors {
            Some(result) => match result {
                Ok(table) => {
                    if table.is_empty() {
                        self.view_empty()
                    } else {
                        self.view_loaded(&table)
                    }
                }
                Err(error) => self.view_error(error),
            },
            None => self.view_loading(),
        }
    }
}

impl DonorList {
    fn view_loading(&self) -> Html<Self> {
        html! {
            <div class="donor_list -loading", >
                { "Loading..." }
            </div>
        }
    }

    fn view_error(&self, error: &str) -> Html<Self> {
        html! {
            <div class="donor_list -error", >
                { "Error: " }{ error }
            </div>
        }
    }

    fn view_loaded(&self, donors: &[Donor]) -> Html<Self> {
        html! {
            <ul class="donor_list -loaded", >
                { for donors.iter().map(|donor| self.view_item(donor)) }
            </ul>
        }
    }

    fn view_empty(&self) -> Html<Self> {
        html! {
            <div class="donor_list -empty", >
                { "Empty" }
            </div>
        }
    }

    fn view_item(&self, donor: &Donor) -> Html<Self> {
        let donor_route = Route::Profile(donor.account.clone());
        let donated = donor.donated as f64;
        html! {
            <li class="donor_list_item", >
                <Link: class="donor_creator",
                    route=donor_route,
                    text=donor.account.clone(),
                />
                <div class="donor_donated", >
                    { format!("{:.*}", 4, donated / 10000.) } { " EOS" }
                </div>
            </li>
        }
    }
}
