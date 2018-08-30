use eos::types::*;
use serde::Serialize;
use serde_json;
use stdweb::Value;
use traits::ToAction;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct ScatterService(Value);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequiredFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Network>>,
}

impl PartialEq for RequiredFields {
    fn eq(&self, other: &RequiredFields) -> bool {
        self.accounts == other.accounts
    }
}

js_serializable!(RequiredFields);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Network {
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

impl PartialEq for Network {
    fn eq(&self, other: &Network) -> bool {
        self.chain_id == other.chain_id
            && self.protocol == other.protocol
            && self.blockchain == other.blockchain
            && self.host == other.host
            && self.port == other.port
    }
}

js_serializable!(Network);

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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EosConfig {
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

js_serializable!(EosConfig);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ScatterError {
    NotConnected,
    Locked,
    Rejected,
    Unknown(String),
}

js_serializable!(Authorization);
js_deserializable!(Authorization);

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

js_serializable!(ScatterTransaction);
js_deserializable!(ScatterTransaction);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushedTransaction {
    pub transaction_id: String,
}

js_serializable!(PushedTransaction);
js_deserializable!(PushedTransaction);

impl ScatterService {
    pub fn connect(
        appname: String,
        timeout: u32,
        callback: Callback<Result<ScatterService, ScatterError>>,
    ) {
        let callback = move |connected: bool, lib: Value| {
            let result = if connected {
                Ok(ScatterService(lib))
            } else {
                Err(ScatterError::NotConnected)
            };
            callback.emit(result);
        };
        js! { @(no_return)
            window.Eos = require("eosjs");
            var callback = @{callback};
            var appname = @{appname};
            var timeout = @{timeout};

            // try {
            //     // var ScatterJS = require("scatter-js/dist/scatter.cjs.js");
            //     var Scatter = require("scatter-core");
            //     var ScatterEos = require("scatter-plugin-eos");
            //     console.log("SCATTER", Scatter, ScatterEos);
            //     Scatter.plugins(new ScatterEos());
            //     console.time("Test!");
            //     window.Scatter = Scatter;
            //     Scatter.scatter
            //         .connect(appname)
            //         .then(function (connected) {
            //             console.log("balls 1");
            //             console.timeEnd("Test!");
            //             callback(true, Scatter.scatter);
            //             callback.drop();
            //         })
            //         .catch(function (error) {
            //             console.log("balls 2", error);
            //             callback(false, null);
            //             callback.drop();
            //         })
            // } catch (error) {
            //     console.log("balls 3", error, Scatter);
            //     callback(false, null);
            //     callback.drop();
            // }

            if (window.scatter) {
                var scatter = window.scatter;
                // window.scatter = null;
                callback(true, scatter);
                callback.drop();
            } else {
                var timeout = setTimeout(function () {
                    callback(false, null);
                    callback.drop();
                }, timeout);
                document.addEventListener("scatterLoaded", function () {
                    clearTimeout(timeout);
                    var scatter = window.scatter;
                    // window.scatter = null;
                    callback(true, scatter);
                    callback.drop();
                });
            }
        };
    }

    pub fn get_identity(
        &self,
        required_fields: RequiredFields,
        callback: Callback<Result<ScatterIdentity, ScatterError>>,
    ) {
        let lib = self.0.as_ref();
        let callback = move |data: Option<String>, error: String| {
            let result = match (data, error.as_str()) {
                (_, "locked") => Err(ScatterError::Locked),
                (_, "identity_rejected") => Err(ScatterError::Rejected),
                (Some(data), "") => {
                    let identity = serde_json::from_str::<ScatterIdentity>(&data).unwrap();
                    Ok(identity)
                }
                _ => {
                    if error.contains("Connect and Authenticate first") {
                        Err(ScatterError::NotConnected)
                    } else {
                        Err(ScatterError::Unknown(error))
                    }
                }
            };
            callback.emit(result);
        };
        js! { @(no_return)
            var scatter = @{lib};
            var callback = @{callback};
            var required_fields = @{required_fields};
            try {
                scatter
                    .getIdentity(required_fields)
                    .then(function (identity) {
                        callback(JSON.stringify(identity), "");
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.log("error from scatter");
                        console.dir(error);
                        callback(null, error.type || error.message);
                        callback.drop();
                    });
            } catch (error) {
                console.log("error from scatter");
                console.dir(error);
                callback(null, error.type || error.message);
                callback.drop();
            }
        };
    }

    pub fn forget_identity(&self, callback: Callback<Result<(), ScatterError>>) {
        let lib = self.0.as_ref();
        let callback = move |_logged_out: bool| {
            callback.emit(Ok(()));
        };
        js! { @(no_return)
            var scatter = @{lib};
            var callback = @{callback};
            try {
                scatter
                    .forgetIdentity()
                    .then(function (logged_out) {
                        callback(logged_out);
                        callback.drop();
                    })
                    .catch(function (_) {
                        callback(false);
                        callback.drop();
                    });
            } catch (_) {
                callback(false);
                callback.drop();
            }
        };
    }

    pub fn identity(&self) -> Option<ScatterIdentity> {
        let lib = self.0.as_ref();
        let value: Value = js!{
            var scatter = @{lib};
            try {
                return JSON.stringify(scatter.identity);
            } catch (_) {
                return null;
            }
        };
        match value {
            Value::String(json) => serde_json::from_str::<ScatterIdentity>(&json).ok(),
            _ => None,
        }
    }

    pub fn push_transaction(
        &self,
        network: Network,
        config: EosConfig,
        transaction: ScatterTransaction,
        callback: Callback<Result<PushedTransaction, ScatterError>>,
    ) {
        debug!("Pushing transaction: {:#?}", transaction);
        let scatter = self.0.as_ref();
        let callback = move |data: Option<String>, error: String| {
            let result = match (data, error.as_str()) {
                (_, "locked") => Err(ScatterError::Locked),
                (_, "identity_rejected") => Err(ScatterError::Rejected),
                (Some(json), "") => serde_json::from_str::<PushedTransaction>(&json).map_err(|e| {
                    ScatterError::Unknown(format!(
                        "Error deserializing json: {:#?}, JSON: {:#?}",
                        e, json
                    ))
                }),
                _ => {
                    if error.contains("Connect and Authenticate first") {
                        Err(ScatterError::NotConnected)
                    } else {
                        Err(ScatterError::Unknown(error))
                    }
                }
            };
            callback.emit(result);
        };
        js! { @(no_return)
            try {
                var scatter = @{scatter};
                var network = @{network};
                var config = @{config};
                var transaction = @{transaction};
                var callback = @{callback};
                var Eos = require("eosjs");
                var eos = scatter.eos(network, Eos, config);
                eos.transaction(transaction)
                    .then(function (pushed_transaction) {
                        console.warn("!!!!!! 0", pushed_transaction);
                        callback(JSON.stringify(pushed_transaction), "");
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.error("!!!!!! 1", error);
                        callback(null, JSON.stringify(error));
                        callback.drop();
                    });
            } catch (error) {
                console.error("!!!!!! 2", error);
                callback(null, JSON.stringify(error));
                callback.drop();
            }
        };
    }

    // suggest_network

    // require_version
}
