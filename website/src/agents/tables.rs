use failure::Error;
use serde::Deserialize;
use services::eos::{EosService, TableRows, TableRowsParams};
use std::collections::HashSet;
use types::*;
use yew::prelude::worker::*;
use yew::prelude::Callback;
use yew::services::fetch::FetchTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TablesConfig {
    pub endpoint: String,
    pub code: String,
    pub cache_timeout: u32,
}

impl PartialEq for TablesConfig {
    fn eq(&self, other: &TablesConfig) -> bool {
        self.endpoint == other.endpoint
            && self.code == other.code
            && self.cache_timeout == other.cache_timeout
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TablesInput {
    Configure(TablesConfig),
    GetGlobalConfig,
    GetPopularPolls,
    GetNewPolls,
    GetDonors,
    GetNewDonations,
}

impl Transferable for TablesInput {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TablesOutput {
    GlobalConfig(Result<GlobalConfig, String>),
    PopularPolls(Result<Vec<Poll>, String>),
    NewPolls(Result<Vec<Poll>, String>),
    Donors(Result<Vec<Donor>, String>),
    NewDonations(Result<Vec<Donation>, String>),
}

impl Transferable for TablesOutput {}

pub enum TablesMsg {
    GlobalConfig(Result<TableRows<GlobalConfig>, Error>),
    PopularPolls(Result<TableRows<Poll>, Error>),
    NewPolls(Result<TableRows<Poll>, Error>),
    Donors(Result<TableRows<Donor>, Error>),
    NewDonations(Result<TableRows<Donation>, Error>),
}

pub struct TablesAgent {
    config: Option<TablesConfig>,
    link: AgentLink<TablesAgent>,
    eos: EosService,
    subscribers: HashSet<HandlerId>,
    global_config_task: Option<FetchTask>,
    global_config: GlobalConfig,
    popular_polls_task: Option<FetchTask>,
    popular_polls: Option<Result<Vec<Poll>, String>>,
    new_polls_task: Option<FetchTask>,
    new_polls: Option<Result<Vec<Poll>, String>>,
    donors_task: Option<FetchTask>,
    donors: Option<Result<Vec<Donor>, String>>,
    new_donations_task: Option<FetchTask>,
    new_donations: Option<Result<Vec<Donation>, String>>,
}

impl Agent for TablesAgent {
    type Reach = Context;
    type Message = TablesMsg;
    type Input = TablesInput;
    type Output = TablesOutput;

    fn create(link: AgentLink<Self>) -> Self {
        TablesAgent {
            config: None,
            link,
            eos: EosService::new(),
            subscribers: HashSet::new(),
            global_config_task: None,
            global_config: GlobalConfig::default(),
            popular_polls_task: None,
            popular_polls: None,
            new_polls_task: None,
            new_polls: None,
            donors_task: None,
            donors: None,
            new_donations_task: None,
            new_donations: None,
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            TablesInput::Configure(config) => {
                self.config = Some(config);
            }
            TablesInput::GetGlobalConfig => self.fetch_global_config(),
            TablesInput::GetPopularPolls => {
                if let Some(popular_polls) = &self.popular_polls {
                    let out = TablesOutput::PopularPolls(popular_polls.clone());
                    self.link.response(who, out);
                }
                self.fetch_popular_polls()
            }
            TablesInput::GetNewPolls => {
                if let Some(new_polls) = &self.new_polls {
                    let out = TablesOutput::NewPolls(new_polls.clone());
                    self.link.response(who, out);
                }
                self.fetch_new_polls()
            }
            TablesInput::GetDonors => {
                if let Some(donors) = &self.donors {
                    let out = TablesOutput::Donors(donors.clone());
                    self.link.response(who, out);
                }
                self.fetch_donors()
            }
            TablesInput::GetNewDonations => {
                if let Some(new_donations) = &self.new_donations {
                    let out = TablesOutput::NewDonations(new_donations.clone());
                    self.link.response(who, out);
                }
                self.fetch_new_donations()
            }
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            TablesMsg::GlobalConfig(result) => None,
            TablesMsg::PopularPolls(result) => {
                let rows = get_rows(result);
                self.popular_polls = Some(rows.clone());
                Some(TablesOutput::PopularPolls(rows))
            }
            TablesMsg::NewPolls(result) => {
                let rows = get_rows(result);
                self.new_polls = Some(rows.clone());
                Some(TablesOutput::NewPolls(rows))
            }
            TablesMsg::Donors(result) => {
                let rows = get_rows(result);
                self.donors = Some(rows.clone());
                Some(TablesOutput::Donors(rows))
            }
            TablesMsg::NewDonations(result) => {
                let rows = get_rows(result);
                self.new_donations = Some(rows.clone());
                Some(TablesOutput::NewDonations(rows))
            }
        };
        if let Some(output) = output {
            for sub in &self.subscribers {
                self.link.response(*sub, output.clone());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

fn get_rows<Row>(result: Result<TableRows<Row>, Error>) -> Result<Vec<Row>, String>
where
    for<'de> Row: Deserialize<'de> + 'static,
{
    result
        .map(|table| table.rows)
        .map_err(|error| format!("{:#?}", error))
}

impl TablesAgent {
    pub fn new(config: TablesConfig, callback: Callback<TablesOutput>) -> Box<Bridge<TablesAgent>> {
        let mut tables = TablesAgent::bridge(callback);
        tables.send(TablesInput::Configure(config));
        tables
    }

    fn fetch<Msg, Row>(&mut self, params: TableRowsParams, msg: Msg) -> Option<FetchTask>
    where
        Msg: Fn(Result<TableRows<Row>, Error>) -> TablesMsg + 'static,
        for<'de> Row: Deserialize<'de> + 'static,
    {
        let config = match &self.config {
            Some(config) => config,
            None => return None,
        };
        let callback = self.link.send_back(msg);
        let endpoint = config.endpoint.clone();
        let task = self.eos.get_table_rows(&endpoint, params, callback);
        Some(task)
    }

    fn get_code(&self) -> String {
        match &self.config {
            Some(config) => config.code.clone(),
            None => "".to_string(),
        }
    }

    fn fetch_global_config(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.get_code(),
            code: self.get_code(),
            table: "config".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };
        self.global_config_task = self.fetch(params, TablesMsg::GlobalConfig);
    }

    fn fetch_popular_polls(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.get_code(),
            code: self.get_code(),
            table: "popularpolls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        self.popular_polls_task = self.fetch(params, TablesMsg::PopularPolls);
    }

    fn fetch_new_polls(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.get_code(),
            code: self.get_code(),
            table: "newpolls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        self.new_polls_task = self.fetch(params, TablesMsg::NewPolls);
    }

    fn fetch_donors(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.get_code(),
            code: self.get_code(),
            table: "donors".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        self.donors_task = self.fetch(params, TablesMsg::Donors);
    }

    fn fetch_new_donations(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.get_code(),
            code: self.get_code(),
            table: "newdonations".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        self.new_donations_task = self.fetch(params, TablesMsg::NewDonations);
    }
}
