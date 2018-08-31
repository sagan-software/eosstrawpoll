use eos::types::*;
use failure;
use serde::Deserialize;
use services::eos::{EosService, TableRows, TableRowsParams};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use stdweb::web::Date;
use types::*;
use yew::prelude::worker::{self, *};
use yew::prelude::Callback;
use yew::services::fetch::FetchTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChainData<Item> {
    NotAsked,
    Loading,
    Success(Item),
    Failure(ChainError),
}

impl<Item> Default for ChainData<Item> {
    fn default() -> Self {
        ChainData::NotAsked
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChainInput {
    Configure(Chain),
    GetGlobalConfig,
    GetPopularPolls,
    GetNewPolls,
    GetDonors,
    GetNewDonations,
    GetPolls(AccountName),
    GetPoll(AccountName, PollName),
}

impl Transferable for ChainInput {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChainOutput {
    Configured,
    GlobalConfig(ChainData<Option<GlobalConfig>>),
    PopularPolls(ChainData<Vec<Poll>>),
    NewPolls(ChainData<Vec<Poll>>),
    Donors(ChainData<Vec<Donor>>),
    NewDonations(ChainData<Vec<Donation>>),
    Polls(AccountName, ChainData<Vec<Poll>>),
    Poll(AccountName, PollName, ChainData<Poll>),
}

impl Transferable for ChainOutput {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChainError {
    Message(String),
}

impl<Item> From<ChainError> for ChainData<Item> {
    fn from(chain_error: ChainError) -> ChainData<Item> {
        ChainData::Failure(chain_error)
    }
}

impl From<failure::Error> for ChainError {
    fn from(error: failure::Error) -> ChainError {
        ChainError::Message(format!("{:#?}", error))
    }
}

pub enum ChainMsg {
    GlobalConfig(Result<TableRows<GlobalConfig>, failure::Error>),
    PopularPolls(Result<TableRows<Poll>, failure::Error>),
    NewPolls(Result<TableRows<Poll>, failure::Error>),
    Donors(Result<TableRows<Donor>, failure::Error>),
    NewDonations(Result<TableRows<Donation>, failure::Error>),
    Polls(AccountName, Result<TableRows<Poll>, failure::Error>),
}

pub enum InternalChainData<Item> {
    NotAsked,
    Loading(f64, FetchTask),
    Success(f64, Item),
    Updating(f64, Item, FetchTask),
    Failure(f64, ChainError),
    Retrying(f64, ChainError, FetchTask),
}

impl<Item> Default for InternalChainData<Item> {
    fn default() -> Self {
        InternalChainData::NotAsked
    }
}

impl<Item> InternalChainData<Item>
where
    Item: Clone,
{
    fn to_chain_data(&self) -> ChainData<Item> {
        match self {
            InternalChainData::NotAsked => ChainData::NotAsked,
            InternalChainData::Loading(_, _) => ChainData::Loading,
            InternalChainData::Success(_, item) => ChainData::Success(item.clone()),
            InternalChainData::Updating(_, item, _) => ChainData::Success(item.clone()),
            InternalChainData::Failure(_, error) => ChainData::Failure(error.clone()),
            InternalChainData::Retrying(_, error, _) => ChainData::Failure(error.clone()),
        }
    }

    fn with_task(&self, task: FetchTask) -> InternalChainData<Item> {
        let now = Date::now();
        match self {
            InternalChainData::NotAsked => InternalChainData::Loading(now, task),
            InternalChainData::Loading(_, _) => InternalChainData::Loading(now, task),
            InternalChainData::Success(_, item) => {
                InternalChainData::Updating(now, item.clone(), task)
            }
            InternalChainData::Updating(_, item, _) => {
                InternalChainData::Updating(now, item.clone(), task)
            }
            InternalChainData::Failure(_, error) => {
                InternalChainData::Retrying(now, error.clone(), task)
            }
            InternalChainData::Retrying(_, error, _) => {
                InternalChainData::Retrying(now, error.clone(), task)
            }
        }
    }

    fn from_task(task: FetchTask) -> InternalChainData<Item> {
        InternalChainData::Loading(Date::now(), task)
    }
}

impl<Item> InternalChainData<Vec<Item>> {
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Item, &Item) -> Ordering,
    {
        match self {
            InternalChainData::Success(_, items) => items.sort_by(compare),
            InternalChainData::Updating(_, items, _) => items.sort_by(compare),
            _ => (),
        };
    }
}

impl<Row> From<Result<TableRows<Row>, failure::Error>> for InternalChainData<Vec<Row>> {
    fn from(result: Result<TableRows<Row>, failure::Error>) -> InternalChainData<Vec<Row>> {
        let now = Date::now();
        match result {
            Ok(table) => InternalChainData::Success(now, table.rows),
            Err(error) => InternalChainData::Failure(now, error.into()),
        }
    }
}

impl<Row> From<Result<TableRows<Row>, failure::Error>> for InternalChainData<Option<Row>>
where
    Row: Clone,
{
    fn from(result: Result<TableRows<Row>, failure::Error>) -> InternalChainData<Option<Row>> {
        let now = Date::now();
        match result {
            Ok(table) => InternalChainData::Success(now, table.rows.iter().cloned().next()),
            Err(error) => InternalChainData::Failure(now, error.into()),
        }
    }
}

pub struct ChainAgent {
    link: AgentLink<ChainAgent>,
    eos: EosService,
    chain: Chain,
    subscribers: HashSet<HandlerId>,
    global_config: InternalChainData<Option<GlobalConfig>>,
    donors: InternalChainData<Vec<Donor>>,
    new_donations: InternalChainData<Vec<Donation>>,
    new_polls: InternalChainData<Vec<Poll>>,
    popular_polls: InternalChainData<Vec<Poll>>,
    polls: HashMap<AccountName, InternalChainData<Vec<Poll>>>,
}

impl Agent for ChainAgent {
    type Reach = worker::Context;
    type Message = ChainMsg;
    type Input = ChainInput;
    type Output = ChainOutput;

    fn create(link: AgentLink<Self>) -> Self {
        ChainAgent {
            link,
            eos: EosService::new(),
            chain: Chain::default(),
            subscribers: HashSet::new(),
            global_config: InternalChainData::default(),
            donors: InternalChainData::default(),
            new_donations: InternalChainData::default(),
            new_polls: InternalChainData::default(),
            popular_polls: InternalChainData::default(),
            polls: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        let output = match msg {
            ChainInput::Configure(chain) => {
                self.chain = chain;
                ChainOutput::Configured
            }
            ChainInput::GetGlobalConfig => {
                self.fetch_global_config();
                let chain_data = self.global_config.to_chain_data();
                ChainOutput::GlobalConfig(chain_data)
            }
            ChainInput::GetPopularPolls => {
                self.fetch_popular_polls();
                let chain_data = self.popular_polls.to_chain_data();
                ChainOutput::PopularPolls(chain_data)
            }
            ChainInput::GetNewPolls => {
                self.fetch_new_polls();
                let chain_data = self.new_polls.to_chain_data();
                ChainOutput::NewPolls(chain_data)
            }
            ChainInput::GetDonors => {
                self.fetch_donors();
                let chain_data = self.donors.to_chain_data();
                ChainOutput::Donors(chain_data)
            }
            ChainInput::GetNewDonations => {
                self.fetch_new_donations();
                let chain_data = self.new_donations.to_chain_data();
                ChainOutput::NewDonations(chain_data)
            }
            ChainInput::GetPolls(account) => {
                self.fetch_polls(account.clone());
                let chain_data = match self.polls.get(&account) {
                    Some(polls) => polls.to_chain_data(),
                    None => ChainData::default(),
                };
                ChainOutput::Polls(account, chain_data)
            }
            // ChainInput::GetPoll(chain, account, slug) => {}
            _ => ChainOutput::Configured,
        };
        self.link.response(who, output);
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            ChainMsg::GlobalConfig(result) => {
                self.global_config = result.into();
                ChainOutput::GlobalConfig(self.global_config.to_chain_data())
            }
            ChainMsg::PopularPolls(result) => {
                self.popular_polls = result.into();
                self.popular_polls.sort_by(|a, b| {
                    let a_pop: f64 = a.popularity.parse().unwrap();
                    let b_pop: f64 = b.popularity.parse().unwrap();
                    b_pop.partial_cmp(&a_pop).unwrap()
                });
                ChainOutput::PopularPolls(self.popular_polls.to_chain_data())
            }
            ChainMsg::NewPolls(result) => {
                self.new_polls = result.into();
                self.new_polls
                    .sort_by(|a, b| b.create_time.cmp(&a.create_time));
                ChainOutput::NewPolls(self.new_polls.to_chain_data())
            }
            ChainMsg::Donors(result) => {
                self.donors = result.into();
                self.donors.sort_by(|a, b| b.donated.cmp(&a.donated));
                ChainOutput::Donors(self.donors.to_chain_data())
            }
            ChainMsg::NewDonations(result) => {
                self.new_donations = result.into();
                self.new_donations.sort_by(|a, b| b.created.cmp(&a.created));
                ChainOutput::NewDonations(self.new_donations.to_chain_data())
            }
            ChainMsg::Polls(account, result) => {
                self.polls.insert(account.clone(), result.into());
                let chain_data = match self.polls.get(&account) {
                    Some(polls) => polls.to_chain_data(),
                    None => ChainData::default(),
                };
                ChainOutput::Polls(account, chain_data)
            }
            _ => ChainOutput::GlobalConfig(self.global_config.to_chain_data()),
        };
        for sub in &self.subscribers {
            self.link.response(*sub, output.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl ChainAgent {
    pub fn new(chain: Chain, callback: Callback<ChainOutput>) -> Box<Bridge<ChainAgent>> {
        let mut agent = ChainAgent::bridge(callback);
        agent.send(ChainInput::Configure(chain));
        agent
    }

    fn fetch_table_rows<ToMsg, Row>(&mut self, params: TableRowsParams, to_msg: ToMsg) -> FetchTask
    where
        ToMsg: Fn(Result<TableRows<Row>, failure::Error>) -> ChainMsg + 'static,
        for<'de> Row: Deserialize<'de> + 'static,
    {
        let callback = self.link.send_back(to_msg);
        let endpoint = self.chain.endpoint.to_string();
        self.eos.get_table_rows(&endpoint, params, callback)
    }

    fn fetch_global_config(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "config".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(1),
            key_type: None,
            index_position: None,
        };
        let task = self.fetch_table_rows(params, ChainMsg::GlobalConfig);
        self.global_config = self.global_config.with_task(task);
    }

    fn fetch_polls(&mut self, account: AccountName) {
        let params = TableRowsParams {
            json: true,
            scope: account.clone(),
            code: self.chain.code_account.clone(),
            table: "polls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let acct = account.clone();
        let callback = move |result| ChainMsg::Polls(acct.clone(), result);
        let task = self.fetch_table_rows(params, callback);
        let polls = match self.polls.get(&account) {
            Some(polls) => polls.with_task(task),
            None => InternalChainData::from_task(task),
        };
        self.polls.insert(account, polls);
    }

    fn fetch_popular_polls(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "popularpolls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let task = self.fetch_table_rows(params, ChainMsg::PopularPolls);
        self.popular_polls = self.popular_polls.with_task(task);
    }

    fn fetch_new_polls(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "newpolls".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let task = self.fetch_table_rows(params, ChainMsg::NewPolls);
        self.new_polls = self.new_polls.with_task(task);
    }

    fn fetch_donors(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "donors".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let task = self.fetch_table_rows(params, ChainMsg::Donors);
        self.donors = self.donors.with_task(task);
    }

    fn fetch_new_donations(&mut self) {
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "newdonations".to_string(),
            lower_bound: None,
            upper_bound: None,
            limit: Some(150),
            key_type: None,
            index_position: None,
        };
        let task = self.fetch_table_rows(params, ChainMsg::NewDonations);
        self.new_donations = self.new_donations.with_task(task);
    }
}
