use stdweb::Value;
use yew::callback::Callback;

#[derive(Debug, Clone)]
pub struct EosService(Value);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EosConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,

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

js_serializable!(EosConfig);

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

js_deserializable!(Info);

impl EosService {
    pub fn new(config: EosConfig) -> Self {
        let lib = js! {
            var config = @{config};
            var Eos = require("eosjs");
            return Eos(config);
        };
        EosService(lib)
    }

    pub fn from_value(value: Value) -> Self {
        EosService(value)
    }

    pub fn to_value(&self) -> &Value {
        self.0.as_ref()
    }

    pub fn get_info(&self, callback: Callback<Result<Info, String>>) {
        let lib = self.0.as_ref();
        let callback = move |error: String, info: Info| {
            let result = if error.is_empty() {
                Ok(info)
            } else {
                Err(error)
            };
            callback.emit(result);
        };
        js! {
            var lib = @{lib};
            var callback = @{callback};
            try {
                lib.getInfo({})
                    .then(function (info) {
                        console.log("!!! 111", info);
                        callback("", info);
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.log("!!! 222", error);
                        callback(error, "");
                        callback.drop();
                    });
            } catch (error) {
                        console.log("!!! 333", error);
                callback(error, "");
                callback.drop();
            }
        }
    }
}
