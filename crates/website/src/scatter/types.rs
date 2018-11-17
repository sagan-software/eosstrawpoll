use crate::eos::types::*;
use eosio::{AccountName, Action};
use serde::Serialize;
use serde_json;
use stdweb::Value;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScatterRequiredFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<ScatterNetwork>>,
}

impl PartialEq for ScatterRequiredFields {
    fn eq(&self, other: &ScatterRequiredFields) -> bool {
        self.accounts == other.accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScatterNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

impl PartialEq for ScatterNetwork {
    fn eq(&self, other: &ScatterNetwork) -> bool {
        self.chain_id == other.chain_id
            && self.protocol == other.protocol
            && self.blockchain == other.blockchain
            && self.host == other.host
            && self.port == other.port
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ScatterAccount {
    pub name: AccountName,
    pub authority: String,
    pub blockchain: String,
}

impl PartialEq for ScatterAccount {
    fn eq(&self, other: &ScatterAccount) -> bool {
        self.name == other.name
            && self.authority == other.authority
            && self.blockchain == other.blockchain
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct ScatterIdentity {
    pub hash: String,
    pub kyc: bool,
    pub name: String,
    pub public_key: PublicKey,
    pub accounts: Vec<ScatterAccount>,
}

impl ScatterIdentity {
    pub fn account_name(&self) -> Option<AccountName> {
        match self.accounts.first() {
            Some(account) => Some(account.name.clone()),
            None => None,
        }
    }
}

impl PartialEq for ScatterIdentity {
    fn eq(&self, other: &ScatterIdentity) -> bool {
        self.hash == other.hash
            && self.kyc == other.kyc
            && self.name == other.name
            && self.public_key == other.public_key
            && self.accounts == other.accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ScatterError {
    NotConnected,
    Locked,
    Rejected,
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterTransaction {
    pub actions: Vec<serde_json::Value>,
}

impl<Data> From<Action<Data>> for ScatterTransaction
where
    Data: Serialize,
{
    fn from(action: Action<Data>) -> ScatterTransaction {
        let serialized_action = serde_json::to_value(&action).unwrap();
        ScatterTransaction {
            actions: vec![serialized_action],
        }
    }
}

impl<Data> From<Vec<Action<Data>>> for ScatterTransaction
where
    Data: Serialize,
{
    fn from(actions: Vec<Action<Data>>) -> ScatterTransaction {
        let mut serialized_actions = Vec::new();
        for action in &actions {
            serialized_actions.push(serde_json::to_value(&action).unwrap());
        }
        ScatterTransaction {
            actions: serialized_actions,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushedTransaction {
    pub transaction_id: String,
}
