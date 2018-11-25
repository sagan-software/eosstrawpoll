use eosio::{n, AccountName, ActionName, Authorization, PermissionName, ScopeName, TableName};
use stdweb::unstable::TryInto;

pub type ChainId = String;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

pub type PublicKey = String;

pub type PrivateKey = String;

pub type Signature = String;

pub type TransactionId = String;

pub type Asset = String;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableRowsParams {
    pub json: bool,
    pub scope: ScopeName,
    pub code: AccountName,
    pub table: TableName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encode_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EosJsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<ChainId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_in_seconds: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<bool>,
}
