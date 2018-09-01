pub type ChainId = String;

pub type Name = String;

pub type AccountName = Name;

pub type TableName = Name;

pub type ActionName = Name;

pub type Symbol = Name;

pub type PermissionName = Name;

pub type ScopeName = Name;

pub type TypeName = Name;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

pub type PublicKey = String;

pub type PrivateKey = String;

pub type Signature = String;

pub type TransactionId = String;

pub type Asset = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    pub actor: AccountName,
    pub permission: PermissionName,
}

impl Authorization {
    pub fn active(actor: AccountName) -> Authorization {
        Authorization {
            actor,
            permission: "active".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action<Data> {
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<Authorization>,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub server_version: ServerVersion,
    pub chain_id: ChainId,
    pub head_block_num: BlockNum,
    pub head_block_id: BlockId,
    pub head_block_time: BlockTimestamp,
    pub head_block_producer: AccountName,
    pub last_irreversible_block_num: BlockNum,
    pub last_irreversible_block_id: BlockId,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableRows<Row> {
    pub rows: Vec<Row>,
    pub more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableRowsParams {
    pub scope: AccountName,
    pub code: AccountName,
    pub table: TableName,
    pub json: bool,
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
