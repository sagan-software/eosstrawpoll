use eos::service::*;
use eos::types::*;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use stdweb::web::Date;
use types::*;
use yew::prelude::worker::{self, *};
use yew::prelude::Callback;
use yew::services::fetch::FetchTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EosData<Item> {
    NotAsked,
    Loading,
    Success(Item),
    Failure(EosError),
}

impl<Item> Default for EosData<Item> {
    fn default() -> Self {
        EosData::NotAsked
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EosInput {
    Configure(Chain),
    GetGlobalConfig,
    GetPopularPolls,
    GetNewPolls,
    GetDonors,
    GetNewDonations,
    GetPolls(AccountName),
    GetPoll(AccountName, PollName),
}

impl Transferable for EosInput {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EosOutput {
    Configured,
    GlobalConfig(EosData<Option<GlobalConfig>>),
    PopularPolls(EosData<Vec<PollTease>>),
    NewPolls(EosData<Vec<PollTease>>),
    Donors(EosData<Vec<Donor>>),
    NewDonations(EosData<Vec<Donation>>),
    Polls(AccountName, EosData<Vec<Poll>>),
    Poll(AccountName, PollName, EosData<Poll>),
}

impl Transferable for EosOutput {}

impl<Item> From<EosError> for EosData<Item> {
    fn from(chain_error: EosError) -> EosData<Item> {
        EosData::Failure(chain_error)
    }
}

pub enum EosMsg {
    GlobalConfig(Result<TableRows<GlobalConfig>, EosError>),
    PopularPolls(Result<TableRows<PollTease>, EosError>),
    NewPolls(Result<TableRows<PollTease>, EosError>),
    Donors(Result<TableRows<Donor>, EosError>),
    NewDonations(Result<TableRows<Donation>, EosError>),
    Polls(AccountName, Result<TableRows<Poll>, EosError>),
}

pub enum InternalEosData<Item> {
    NotAsked,
    Loading(f64, FetchTask),
    Success(f64, Item),
    Updating(f64, Item, FetchTask),
    Failure(f64, EosError),
    Retrying(f64, EosError, FetchTask),
}

impl<Item> Default for InternalEosData<Item> {
    fn default() -> Self {
        InternalEosData::NotAsked
    }
}

impl<Item> InternalEosData<Item>
where
    Item: Clone,
{
    fn to_eos_data(&self) -> EosData<Item> {
        match self {
            InternalEosData::NotAsked => EosData::NotAsked,
            InternalEosData::Loading(_, _) => EosData::Loading,
            InternalEosData::Success(_, item) => EosData::Success(item.clone()),
            InternalEosData::Updating(_, item, _) => EosData::Success(item.clone()),
            InternalEosData::Failure(_, error) => EosData::Failure(error.clone()),
            InternalEosData::Retrying(_, error, _) => EosData::Failure(error.clone()),
        }
    }

    fn with_task(&self, task: FetchTask) -> InternalEosData<Item> {
        let now = Date::now();
        match self {
            InternalEosData::NotAsked => InternalEosData::Loading(now, task),
            InternalEosData::Loading(_, _) => InternalEosData::Loading(now, task),
            InternalEosData::Success(_, item) => InternalEosData::Updating(now, item.clone(), task),
            InternalEosData::Updating(_, item, _) => {
                InternalEosData::Updating(now, item.clone(), task)
            }
            InternalEosData::Failure(_, error) => {
                InternalEosData::Retrying(now, error.clone(), task)
            }
            InternalEosData::Retrying(_, error, _) => {
                InternalEosData::Retrying(now, error.clone(), task)
            }
        }
    }

    fn from_task(task: FetchTask) -> InternalEosData<Item> {
        InternalEosData::Loading(Date::now(), task)
    }
}

impl<Item> InternalEosData<Vec<Item>> {
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Item, &Item) -> Ordering,
    {
        match self {
            InternalEosData::Success(_, items) => items.sort_by(compare),
            InternalEosData::Updating(_, items, _) => items.sort_by(compare),
            _ => (),
        };
    }
}

impl<Row> From<Result<TableRows<Row>, EosError>> for InternalEosData<Vec<Row>> {
    fn from(result: Result<TableRows<Row>, EosError>) -> InternalEosData<Vec<Row>> {
        let now = Date::now();
        match result {
            Ok(table) => InternalEosData::Success(now, table.rows),
            Err(error) => InternalEosData::Failure(now, error),
        }
    }
}

impl<Row> From<Result<TableRows<Row>, EosError>> for InternalEosData<Option<Row>>
where
    Row: Clone,
{
    fn from(result: Result<TableRows<Row>, EosError>) -> InternalEosData<Option<Row>> {
        let now = Date::now();
        match result {
            Ok(table) => InternalEosData::Success(now, table.rows.iter().cloned().next()),
            Err(error) => InternalEosData::Failure(now, error),
        }
    }
}

pub struct EosAgent {
    link: AgentLink<EosAgent>,
    eos: EosService,
    chain: Chain,
    subscribers: HashSet<HandlerId>,
    global_config: InternalEosData<Option<GlobalConfig>>,
    donors: InternalEosData<Vec<Donor>>,
    new_donations: InternalEosData<Vec<Donation>>,
    new_polls: InternalEosData<Vec<PollTease>>,
    popular_polls: InternalEosData<Vec<PollTease>>,
    polls: HashMap<AccountName, InternalEosData<Vec<Poll>>>,
}

impl Agent for EosAgent {
    type Reach = worker::Context;
    type Message = EosMsg;
    type Input = EosInput;
    type Output = EosOutput;

    fn create(link: AgentLink<Self>) -> Self {
        EosAgent {
            link,
            eos: EosService::new(),
            chain: Chain::default(),
            subscribers: HashSet::new(),
            global_config: InternalEosData::default(),
            donors: InternalEosData::default(),
            new_donations: InternalEosData::default(),
            new_polls: InternalEosData::default(),
            popular_polls: InternalEosData::default(),
            polls: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        let output = match msg {
            EosInput::Configure(chain) => {
                if chain != self.chain {
                    self.chain = chain;
                    self.global_config = InternalEosData::NotAsked;
                    self.donors = InternalEosData::NotAsked;
                    self.new_donations = InternalEosData::NotAsked;
                    self.new_polls = InternalEosData::NotAsked;
                    self.popular_polls = InternalEosData::NotAsked;
                    self.polls = HashMap::new();
                }
                EosOutput::Configured
            }
            EosInput::GetGlobalConfig => {
                self.fetch_global_config();
                let eos_data = self.global_config.to_eos_data();
                EosOutput::GlobalConfig(eos_data)
            }
            EosInput::GetPopularPolls => {
                self.fetch_popular_polls();
                let eos_data = self.popular_polls.to_eos_data();
                EosOutput::PopularPolls(eos_data)
            }
            EosInput::GetNewPolls => {
                self.fetch_new_polls();
                let eos_data = self.new_polls.to_eos_data();
                EosOutput::NewPolls(eos_data)
            }
            EosInput::GetDonors => {
                self.fetch_donors();
                let eos_data = self.donors.to_eos_data();
                EosOutput::Donors(eos_data)
            }
            EosInput::GetNewDonations => {
                self.fetch_new_donations();
                let eos_data = self.new_donations.to_eos_data();
                EosOutput::NewDonations(eos_data)
            }
            EosInput::GetPolls(account) => {
                self.fetch_polls(account.clone());
                let eos_data = match self.polls.get(&account) {
                    Some(polls) => polls.to_eos_data(),
                    None => EosData::default(),
                };
                EosOutput::Polls(account, eos_data)
            }
            // EosInput::GetPoll(chain, account, slug) => {}
            _ => EosOutput::Configured,
        };
        self.link.response(who, output);
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            EosMsg::GlobalConfig(result) => {
                self.global_config = result.into();
                EosOutput::GlobalConfig(self.global_config.to_eos_data())
            }
            EosMsg::PopularPolls(result) => {
                self.popular_polls = result.into();
                self.popular_polls
                    .sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());
                EosOutput::PopularPolls(self.popular_polls.to_eos_data())
            }
            EosMsg::NewPolls(result) => {
                self.new_polls = result.into();
                self.new_polls
                    .sort_by(|a, b| b.create_time.cmp(&a.create_time));
                EosOutput::NewPolls(self.new_polls.to_eos_data())
            }
            EosMsg::Donors(result) => {
                self.donors = result.into();
                self.donors.sort_by(|a, b| b.donated.cmp(&a.donated));
                EosOutput::Donors(self.donors.to_eos_data())
            }
            EosMsg::NewDonations(result) => {
                self.new_donations = result.into();
                self.new_donations
                    .sort_by(|a, b| b.create_time.cmp(&a.create_time));
                EosOutput::NewDonations(self.new_donations.to_eos_data())
            }
            EosMsg::Polls(account, result) => {
                self.polls.insert(account.clone(), result.into());
                let eos_data = match self.polls.get(&account) {
                    Some(polls) => polls.to_eos_data(),
                    None => EosData::default(),
                };
                EosOutput::Polls(account, eos_data)
            }
            _ => EosOutput::GlobalConfig(self.global_config.to_eos_data()),
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

impl EosAgent {
    pub fn new(chain: Chain, callback: Callback<EosOutput>) -> Box<Bridge<EosAgent>> {
        let mut agent = EosAgent::bridge(callback);
        agent.send(EosInput::Configure(chain));
        agent
    }

    fn fetch_table_rows<ToMsg, Row>(&mut self, params: TableRowsParams, to_msg: ToMsg) -> FetchTask
    where
        ToMsg: Fn(Result<TableRows<Row>, EosError>) -> EosMsg + 'static,
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
            encode_type: None,
        };
        let task = self.fetch_table_rows(params, EosMsg::GlobalConfig);
        self.global_config = self.global_config.with_task(task);
    }

    fn fetch_polls(&mut self, account: AccountName) {
        let lower_bound: u64 = account.into();
        let upper_bound = lower_bound + 1;
        let params = TableRowsParams {
            json: true,
            scope: self.chain.code_account.clone(),
            code: self.chain.code_account.clone(),
            table: "polls".to_string(),
            lower_bound: Some(lower_bound.to_string()),
            upper_bound: Some(upper_bound.to_string()),
            limit: Some(150),
            key_type: Some("i64".into()),
            index_position: Some("2".into()),
            encode_type: None,
        };
        let acct = account.clone();
        let callback = move |result| EosMsg::Polls(acct.clone(), result);
        let task = self.fetch_table_rows(params, callback);
        let polls = match self.polls.get(&account) {
            Some(polls) => polls.with_task(task),
            None => InternalEosData::from_task(task),
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
            encode_type: None,
        };
        let task = self.fetch_table_rows(params, EosMsg::PopularPolls);
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
            encode_type: None,
        };
        let task = self.fetch_table_rows(params, EosMsg::NewPolls);
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
            encode_type: None,
        };
        let task = self.fetch_table_rows(params, EosMsg::Donors);
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
            encode_type: None,
        };
        let task = self.fetch_table_rows(params, EosMsg::NewDonations);
        self.new_donations = self.new_donations.with_task(task);
    }
}
