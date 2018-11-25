use crate::eos::types::*;
use eosio_rpc::TableRows;
use failure;
use http::Response;
use serde;
use serde_json;
use yew::callback::Callback;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, StatusCode};

pub struct EosService {
    fetch: FetchService,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EosError {
    BadStatus(u16, Option<String>),
    Message(String),
}

impl ::std::fmt::Display for EosError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            EosError::BadStatus(status, message) => match message {
                Some(message) => write!(f, "bad status {}: {}", status, message),
                None => write!(f, "bad status {}", status),
            },
            EosError::Message(message) => write!(f, "{}", message),
        }
    }
}

impl From<failure::Error> for EosError {
    fn from(error: failure::Error) -> EosError {
        EosError::Message(format!("{:#?}", error))
    }
}

impl From<StatusCode> for EosError {
    fn from(status_code: StatusCode) -> EosError {
        EosError::BadStatus(
            status_code.as_u16(),
            status_code.canonical_reason().map(|s| s.to_string()),
        )
    }
}

impl EosService {
    pub fn new() -> Self {
        EosService {
            fetch: FetchService::new(),
        }
    }

    pub fn get_table_rows<Row>(
        &mut self,
        endpoint: &str,
        params: TableRowsParams,
        callback: Callback<Result<TableRows<Row>, EosError>>,
    ) -> FetchTask
    where
        for<'de> Row: serde::Deserialize<'de> + 'static,
    {
        let handler = move |response: Response<Json<Result<TableRows<Row>, failure::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data.map_err(|e| e.into()))
            } else {
                callback.emit(Err(meta.status.into()))
            }
        };
        // let request = get_table_rows_request(endpoint, params);
        let params_value = serde_json::to_value(params).unwrap();
        info!("Fetching table rows with params: {}", params_value);
        let request = Request::builder()
            .method("POST")
            .uri(format!("{}/v1/chain/get_table_rows", endpoint))
            .body(Json(&params_value))
            .unwrap();
        self.fetch.fetch(request, handler.into())
    }
}
