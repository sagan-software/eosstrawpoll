pub use eos::*;
use failure::Error;
use http::Response;
use serde;
use serde_json;
use yew::callback::Callback;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request};

pub struct EosService {
    fetch: FetchService,
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
        callback: Callback<Result<TableRows<Row>, Error>>,
    ) -> FetchTask
    where
        for<'de> Row: serde::Deserialize<'de> + 'static,
    {
        let handler = move |response: Response<Json<Result<TableRows<Row>, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            info!("META! {:#?}", meta);
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error getting profile https://gravatar.com/",
                    meta.status
                )))
            }
        };
        // let request = get_table_rows_request(endpoint, params);
        let params_value = serde_json::to_value(params).unwrap();
        let request = Request::builder()
            .method("POST")
            .uri(format!("{}/v1/chain/get_table_rows", endpoint))
            .body(Json(&params_value))
            .unwrap();
        self.fetch.fetch(request, handler.into())
    }
}
