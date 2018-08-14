use serde_json;
use serde_json::Value;
use yew::format::Json;
use yew::services::fetch::Request;

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
    pub lower_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub fn get_table_rows_request(endpoint: &str, params: TableRowsParams) -> Request<Json<Value>> {
    let params_value = serde_json::to_value(params).unwrap();
    Request::builder()
        .method("GET")
        .uri(format!("{}/v1/chain/get_table_rows", endpoint))
        .body(Json(params_value))
        .unwrap()
}
