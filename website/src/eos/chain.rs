use eos::types::*;

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
