use eos::types::*;
use failure::Error;
use serde::Deserialize;
use services::eos::{EosService, TableRows, TableRowsParams};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use types::*;
use yew::prelude::worker::*;
use yew::services::fetch::FetchTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiData<Item> {
    NotAsked,
    Loading,
    Success(Item),
    Failure(ApiError),
}

impl<Item> Default for ApiData<Item> {
    fn default() -> Self {
        ApiData::NotAsked
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiInput {
    GetConfig(Chain),
    GetPopularPolls(Chain),
    GetNewPolls(Chain),
    GetDonors(Chain),
    GetNewDonations(Chain),
    GetPolls(Chain, AccountName),
    GetPoll(Chain, AccountName, PollName),
}

impl Transferable for ApiInput {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiOutput {
    Config(Chain, ApiData<GlobalConfig>),
    PopularPolls(Chain, ApiData<Vec<Poll>>),
    NewPolls(Chain, ApiData<Vec<Poll>>),
    Donors(Chain, ApiData<Vec<Donor>>),
    NewDonations(Chain, ApiData<Vec<Donation>>),
    Polls(Chain, AccountName, ApiData<Vec<Poll>>),
    Poll(Chain, AccountName, PollName, ApiData<Poll>),
}

impl Transferable for ApiOutput {}

pub type ApiError = String;

pub enum ApiMsg {
    Config(&'static Chain, Result<TableRows<GlobalConfig>, Error>),
    PopularPolls(&'static Chain, Result<TableRows<Poll>, Error>),
    NewPolls(&'static Chain, Result<TableRows<Poll>, Error>),
    Donors(&'static Chain, Result<TableRows<Donor>, Error>),
    NewDonations(&'static Chain, Result<TableRows<Donation>, Error>),
    Polls(&'static Chain, AccountName, Result<TableRows<Poll>, Error>),
}

pub enum InternalApiData<Item> {
    NotAsked,
    Loading(Instant, FetchTask),
    Success(Instant, Item),
    Updating(Instant, Item, FetchTask),
    Failure(Instant, ApiError),
    Retrying(Instant, ApiError, FetchTask),
}

impl<Item> Default for InternalApiData<Item> {
    fn default() -> Self {
        InternalApiData::NotAsked
    }
}

impl<Item> From<InternalApiData<Item>> for ApiData<Item> {
    fn from(internal_api_data: InternalApiData<Item>) -> ApiData<Item> {
        match internal_api_data {
            InternalApiData::NotAsked => ApiData::NotAsked,
            InternalApiData::Loading(_, _) => ApiData::Loading,
            InternalApiData::Success(_, item) => ApiData::Success(item),
            InternalApiData::Updating(_, item, _) => ApiData::Success(item),
            InternalApiData::Failure(_, error) => ApiData::Failure(error),
            InternalApiData::Retrying(_, error, _) => ApiData::Failure(error),
        }
    }
}

#[derive(Default)]
pub struct ChainState {
    config: InternalApiData<GlobalConfig>,
    donors: InternalApiData<Vec<Donor>>,
    new_donations: InternalApiData<Vec<Donation>>,
    new_polls: InternalApiData<Vec<Poll>>,
    popular_polls: InternalApiData<Vec<Poll>>,
    polls: InternalApiData<HashMap<AccountName, Vec<Poll>>>,
}

pub struct ApiAgent {
    link: AgentLink<ApiAgent>,
    eos: EosService,
    subscribers: HashSet<HandlerId>,
    chains: HashMap<ChainIdPrefix, ChainState>,
}

impl Agent for ApiAgent {
    type Reach = Context;
    type Message = ApiMsg;
    type Input = ApiInput;
    type Output = ApiOutput;

    fn create(link: AgentLink<Self>) -> Self {
        ApiAgent {
            link,
            eos: EosService::new(),
            subscribers: HashSet::new(),
            chains: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            ApiInput::GetConfig(chain) => {
                let out = if let Some(chain_state) = self.chains.get_mut(&chain.chain_id_prefix()) {
                    chain_state.fetch_config(&self.link, &self.eos, &chain);
                    ApiOutput::Config(chain, chain_state.config.into())
                } else {
                    let chain_state = ChainState::default();
                    chain_state.fetch_config(&self.link, &self.eos, &chain);
                    self.chains.insert(chain.chain_id_prefix(), chain_state);
                    ApiOutput::Config(chain, ApiData::Loading)
                };
                self.link.response(who, out);
            }
            // ApiInput::GetPopularPolls(chain) => {
            //     if let Some(popular_polls) = &self.popular_polls {
            //         let out = ApiOutput::PopularPolls(popular_polls.clone());
            //         self.link.response(who, out);
            //     }
            //     self.fetch_popular_polls()
            // }
            // ApiInput::GetNewPolls(chain) => {
            //     if let Some(new_polls) = &self.new_polls {
            //         let out = ApiOutput::NewPolls(new_polls.clone());
            //         self.link.response(who, out);
            //     }
            //     self.fetch_new_polls()
            // }
            // ApiInput::GetDonors(chain) => {
            //     if let Some(donors) = &self.donors {
            //         let out = ApiOutput::Donors(donors.clone());
            //         self.link.response(who, out);
            //     }
            //     self.fetch_donors()
            // }
            // ApiInput::GetNewDonations(chain) => {
            //     if let Some(new_donations) = &self.new_donations {
            //         let out = ApiOutput::NewDonations(new_donations.clone());
            //         self.link.response(who, out);
            //     }
            //     self.fetch_new_donations()
            // }
            // ApiInput::GetPolls(chain, account) => {
            //     if let Some(polls) = self.polls.get(&account) {
            //         let out = ApiOutput::Polls(account.clone(), polls.clone());
            //         self.link.response(who, out);
            //     }
            //     self.fetch_polls(account.clone())
            // }
            // ApiInput::GetPoll(chain, account, slug) => {}
            _ => (),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            ApiMsg::Config(chain, result) => match result {
                Ok(table) => {
                    if let Some(global_config) = table.rows.first() {
                        self.global_config = global_config.clone();
                        Some(ApiOutput::Config(Ok(global_config.clone())))
                    } else {
                        warn!("global config table is empty");
                        None
                    }
                }
                Err(error) => Some(ApiOutput::Config(Err(format!("{:#?}", error)))),
            },
            // ApiMsg::PopularPolls(result) => {
            //     let rows = get_rows(result);
            //     self.popular_polls = Some(rows.clone());
            //     Some(ApiOutput::PopularPolls(rows))
            // }
            // ApiMsg::NewPolls(result) => {
            //     let rows = get_rows(result);
            //     self.new_polls = Some(rows.clone());
            //     Some(ApiOutput::NewPolls(rows))
            // }
            // ApiMsg::Donors(result) => {
            //     let rows = get_rows(result);
            //     self.donors = Some(rows.clone());
            //     Some(ApiOutput::Donors(rows))
            // }
            // ApiMsg::NewDonations(result) => {
            //     let rows = get_rows(result);
            //     self.new_donations = Some(rows.clone());
            //     Some(ApiOutput::NewDonations(rows))
            // }
            // ApiMsg::Polls(account, result) => {
            //     let rows = get_rows(result);
            //     self.polls.insert(account.clone(), rows.clone());
            //     Some(ApiOutput::Polls(account.clone(), rows))
            // }
            _ => (),
        };
        // if let Some(output) = output {
        //     for sub in &self.subscribers {
        //         self.link.response(*sub, output.clone());
        //     }
        // }
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

// impl ApiAgent {
//     fn config(&self, chain) -> ApiData<Config> {

//     }
// }

impl ChainState {
    fn fetch_table_rows<ToMsg, Row, Item>(
        &mut self,
        link: &AgentLink<ApiAgent>,
        eos: &EosService,
        chain: &Chain,
        previous: InternalApiData<Item>,
        params: TableRowsParams,
        to_msg: ToMsg,
    ) -> InternalApiData<Item>
    where
        ToMsg: Fn(Result<TableRows<Row>, Error>) -> ApiMsg + 'static,
        for<'de> Row: Deserialize<'de> + 'static,
    {
        let callback = link.send_back(to_msg);
        let endpoint = chain.endpoint.to_string();
        let task = eos.get_table_rows(&endpoint, params, callback);
        let now = Instant::now();
        match previous {
            InternalApiData::NotAsked => InternalApiData::Loading(now, task),
            InternalApiData::Loading(_, _) => InternalApiData::Loading(now, task),
            InternalApiData::Success(_, item) => InternalApiData::Updating(now, item, task),
            InternalApiData::Updating(_, item, _) => InternalApiData::Updating(now, item, task),
            InternalApiData::Failure(_, error) => InternalApiData::Retrying(now, error, task),
            InternalApiData::Retrying(_, error, _) => InternalApiData::Retrying(now, error, task),
        }
    }

    fn fetch_config(
        &mut self,
        link: &AgentLink<ApiAgent>,
        eos: &EosService,
        chain: &'static Chain,
    ) {
        let params = TableRowsParams {
            json: true,
            scope: chain.code_account,
            code: chain.code_account,
            table: "config".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };
        self.config = self.fetch_table_rows(link, eos, chain, self.config, params, |result| {
            ApiMsg::Config(chain, result)
        });
    }

    // fn fetch_polls(
    //     &mut self,
    //     link: &AgentLink<ApiAgent>,
    //     eos: &EosService,
    //     chain: &Chain,
    //     account: AccountName,
    // ) {
    //     let params = TableRowsParams {
    //         json: true,
    //         scope: account.clone(),
    //         code: chain.code_account,
    //         table: "polls".to_string(),
    //         lower_bound: None,
    //         upper_bound: None,
    //         limit: Some(150),
    //         key_type: None,
    //         index_position: None,
    //     };
    //     let acct = account.clone();
    //     let task = self.fetch(params, move |result| ApiMsg::Polls(acct.clone(), result));
    //     if let Some(task) = task {
    //         self.polls_tasks.insert(account, task);
    //     }
    // }

    // fn fetch_popular_polls(&mut self, link: &AgentLink<ApiAgent>, eos: &EosService, chain: &Chain) {
    //     let params = TableRowsParams {
    //         json: true,
    //         scope: chain.code_account,
    //         code: chain.code_account,
    //         table: "popularpolls".to_string(),
    //         lower_bound: None,
    //         upper_bound: None,
    //         limit: Some(150),
    //         key_type: None,
    //         index_position: None,
    //     };
    //     self.popular_polls_task = self.fetch(params, ApiMsg::PopularPolls);
    // }

    // fn fetch_new_polls(&mut self, link: &AgentLink<ApiAgent>, eos: &EosService, chain: &Chain) {
    //     let params = TableRowsParams {
    //         json: true,
    //         scope: chain.code_account,
    //         code: chain.code_account,
    //         table: "newpolls".to_string(),
    //         lower_bound: None,
    //         upper_bound: None,
    //         limit: Some(150),
    //         key_type: None,
    //         index_position: None,
    //     };
    //     self.new_polls_task = self.fetch(params, ApiMsg::NewPolls);
    // }

    // fn fetch_donors(&mut self, link: &AgentLink<ApiAgent>, eos: &EosService, chain: &Chain) {
    //     let params = TableRowsParams {
    //         json: true,
    //         scope: chain.code_account,
    //         code: chain.code_account,
    //         table: "donors".to_string(),
    //         lower_bound: None,
    //         upper_bound: None,
    //         limit: Some(150),
    //         key_type: None,
    //         index_position: None,
    //     };
    //     self.donors_task = self.fetch(params, ApiMsg::Donors);
    // }

    // fn fetch_new_donations(&mut self, link: &AgentLink<ApiAgent>, eos: &EosService, chain: &Chain) {
    //     let params = TableRowsParams {
    //         json: true,
    //         scope: chain.code_account,
    //         code: chain.code_account,
    //         table: "newdonations".to_string(),
    //         lower_bound: None,
    //         upper_bound: None,
    //         limit: Some(150),
    //         key_type: None,
    //         index_position: None,
    //     };
    //     self.new_donations_task = self.fetch(params, ApiMsg::NewDonations);
    // }
}
