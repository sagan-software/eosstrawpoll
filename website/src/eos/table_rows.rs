#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableRows<Row> {
    pub rows: Vec<Row>,
    pub more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableRowsParams {
    pub scope: String,
    pub code: String,
    pub table: String,
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
