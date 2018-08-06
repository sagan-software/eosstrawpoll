pub use http::{Error, Request};

#[derive(Deserialize, Debug)]
pub struct Info {
    pub server_version: String,
    pub chain_id: String,
    pub head_block_num: u32,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub last_irreversible_block_num: u32,
    pub last_irreversible_block_id: String,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
}

pub fn get_info_request(endpoint: &str) -> Result<Request<()>, Error> {
    let url = format!("{}/v1/chain/get_info", endpoint);
    Request::post(url)
        .header("Content-Type", "application/json")
        .body(())
}

// pub fn get_block(endpoint: &str, num: u64) -> impl Future<Item = String, Error = FetchError> {
//     let uri = format!("{}/v1/chain/get_block", endpoint).parse().unwrap();
//     let body_json = json!({ "block_num_or_id": num });
//     let body = Body::from(body_json.to_string());
//     let mut req = Request::new(body);
//     *req.method_mut() = Method::POST;
//     *req.uri_mut() = uri;
//     let client = Client::new();
//     client
//         .request(req)
//         .and_then(|res| res.into_body().concat2())
//         .map(|chunk| {
//             let v = chunk.to_vec();
//             String::from_utf8_lossy(&v).to_string()
//         }).from_err::<FetchError>()
// }
