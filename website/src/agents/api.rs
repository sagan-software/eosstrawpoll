use failure::Error;
use serde::Deserialize;
use services::eos::{EosService, TableRows, TableRowsParams};
use std::collections::{HashMap, HashSet};
use types::*;
use yew::prelude::worker::*;
use yew::prelude::Callback;
use yew::services::fetch::FetchTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfig {
    pub endpoint: String,
    pub code: String,
    pub cache_timeout: u32,
}

impl PartialEq for ApiConfig {
    fn eq(&self, other: &ApiConfig) -> bool {
        self.endpoint == other.endpoint
            && self.code == other.code
            && self.cache_timeout == other.cache_timeout
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiInput {
    Configure(ApiConfig),
    GetGlobalConfig,
    GetPopularPolls,
    GetNewPolls,
    GetDonors,
    GetNewDonations,
    GetPolls(String),
}

impl Transferable for ApiInput {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiOutput {
    GlobalConfig(Result<GlobalConfig, String>),
    PopularPolls(Result<Vec<Poll>, String>),
    NewPolls(Result<Vec<Poll>, String>),
    Donors(Result<Vec<Donor>, String>),
    NewDonations(Result<Vec<Donation>, String>),
    Polls(String, Result<Vec<Poll>, String>),
}

impl Transferable for ApiOutput {}

pub enum ApiMsg {
    GlobalConfig(Result<TableRows<GlobalConfig>, Error>),
    PopularPolls(Result<TableRows<Poll>, Error>),
    NewPolls(Result<TableRows<Poll>, Error>),
    Donors(Result<TableRows<Donor>, Error>),
    NewDonations(Result<TableRows<Donation>, Error>),
    Polls(String, Result<TableRows<Poll>, Error>),
}

pub struct ApiAgent {
    config: Option<ApiConfig>,
    link: AgentLink<ApiAgent>,
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
    polls_tasks: HashMap<String, FetchTask>,
    polls: HashMap<String, Result<Vec<Poll>, String>>,
}

impl Agent for ApiAgent {
    type Reach = Context;
    type Message = ApiMsg;
    type Input = ApiInput;
    type Output = ApiOutput;

    fn create(link: AgentLink<Self>) -> Self {
        ApiAgent {
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
            polls_tasks: HashMap::new(),
            polls: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            ApiInput::Configure(config) => {
                self.config = Some(config);
            }
            ApiInput::GetGlobalConfig => {
                let out = ApiOutput::GlobalConfig(Ok(self.global_config.clone()));
                self.link.response(who, out);
                self.fetch_global_config()
            }
            ApiInput::GetPopularPolls => {
                if let Some(popular_polls) = &self.popular_polls {
                    let out = ApiOutput::PopularPolls(popular_polls.clone());
                    self.link.response(who, out);
                }
                self.fetch_popular_polls()
            }
            ApiInput::GetNewPolls => {
                if let Some(new_polls) = &self.new_polls {
                    let out = ApiOutput::NewPolls(new_polls.clone());
                    self.link.response(who, out);
                }
                self.fetch_new_polls()
            }
            ApiInput::GetDonors => {
                if let Some(donors) = &self.donors {
                    let out = ApiOutput::Donors(donors.clone());
                    self.link.response(who, out);
                }
                self.fetch_donors()
            }
            ApiInput::GetNewDonations => {
                if let Some(new_donations) = &self.new_donations {
                    let out = ApiOutput::NewDonations(new_donations.clone());
                    self.link.response(who, out);
                }
                self.fetch_new_donations()
            }
            ApiInput::GetPolls(account) => {
                if let Some(polls) = self.polls.get(&account) {
                    let out = ApiOutput::Polls(account.clone(), polls.clone());
                    self.link.response(who, out);
                }
                self.fetch_polls(account.clone())
            }
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            ApiMsg::GlobalConfig(result) => match result {
                Ok(table) => {
                    if let Some(global_config) = table.rows.first() {
                        self.global_config = global_config.clone();
                        Some(ApiOutput::GlobalConfig(Ok(global_config.clone())))
                    } else {
                        warn!("global config table is empty");
                        None
                    }
                }
                Err(error) => Some(ApiOutput::GlobalConfig(Err(format!("{:#?}", error)))),
            },
            ApiMsg::PopularPolls(result) => {
                let rows = get_rows(result);
                self.popular_polls = Some(rows.clone());
                Some(ApiOutput::PopularPolls(rows))
            }
            ApiMsg::NewPolls(result) => {
                let rows = get_rows(result);
                self.new_polls = Some(rows.clone());
                Some(ApiOutput::NewPolls(rows))
            }
            ApiMsg::Donors(result) => {
                let rows = get_rows(result);
                self.donors = Some(rows.clone());
                Some(ApiOutput::Donors(rows))
            }
            ApiMsg::NewDonations(result) => {
                let rows = get_rows(result);
                self.new_donations = Some(rows.clone());
                Some(ApiOutput::NewDonations(rows))
            }
            ApiMsg::Polls(account, result) => {
                let rows = get_rows(result);
                self.polls.insert(account.clone(), rows.clone());
                Some(ApiOutput::Polls(account.clone(), rows))
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

impl ApiAgent {
    pub fn new(config: ApiConfig, callback: Callback<ApiOutput>) -> Box<Bridge<ApiAgent>> {
        let mut api = ApiAgent::bridge(callback);
        api.send(ApiInput::Configure(config));
        api
    }

    fn fetch<ToMsg, Row>(&mut self, params: TableRowsParams, to_msg: ToMsg) -> Option<FetchTask>
    where
        ToMsg: Fn(Result<TableRows<Row>, Error>) -> ApiMsg + 'static,
        for<'de> Row: Deserialize<'de> + 'static,
    {
        let config = match &self.config {
            Some(config) => config,
            None => return None,
        };
        let callback = self.link.send_back(to_msg);
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
        self.global_config_task = self.fetch(params, ApiMsg::GlobalConfig);
    }

    fn fetch_polls(&mut self, account: String) {
        let params = TableRowsParams {
            json: true,
            scope: account.clone(),
            code: self.get_code(),
            table: "polls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let acct = account.clone();
        let task = self.fetch(params, move |result| ApiMsg::Polls(acct.clone(), result));
        if let Some(task) = task {
            self.polls_tasks.insert(account, task);
        }
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
        self.popular_polls_task = self.fetch(params, ApiMsg::PopularPolls);
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
        self.new_polls_task = self.fetch(params, ApiMsg::NewPolls);
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
        self.donors_task = self.fetch(params, ApiMsg::Donors);
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
        self.new_donations_task = self.fetch(params, ApiMsg::NewDonations);
    }
}
