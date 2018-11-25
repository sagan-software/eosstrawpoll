use super::types::*;
use crate::eos::types::*;
use eosio::Authorization;
use serde::Serialize;
use serde_json;
use stdweb::Value;
use yew::prelude::*;

js_serializable!(ScatterRequiredFields);
js_serializable!(ScatterNetwork);
js_serializable!(EosJsConfig);
js_serializable!(ScatterTransaction);
js_deserializable!(ScatterTransaction);
js_serializable!(PushedTransaction);
js_deserializable!(PushedTransaction);

#[derive(Debug, Clone)]
pub struct ScatterService(Value);

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

            try {
                // var ScatterJS = require("scatter-js/dist/scatter.cjs.js");
                var Scatter = require("scatterjs-core").default;
                var ScatterEos = require("scatterjs-plugin-eosjs").default;
                console.log("SCATTER", Scatter, ScatterEos);
                Scatter.plugins(new ScatterEos());
                window.Scatter = Scatter;
                Scatter.scatter
                    .connect(appname)
                    .then(function (connected) {
                        console.log("!!!! CONNECTED?", connected);
                        callback(connected, Scatter.scatter);
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.log("!!!!!!! ERROR CONNECTING", error);
                        callback(false, null);
                        callback.drop();
                    })
            } catch (error) {
                console.log("ERROR SETTING UP SCATTER", error, Scatter);
                callback(false, null);
                callback.drop();
            }

            // if (window.scatter) {
            //     var scatter = window.scatter;
            //     // window.scatter = null;
            //     callback(true, scatter);
            //     callback.drop();
            // } else {
            //     var timeout = setTimeout(function () {
            //         callback(false, null);
            //         callback.drop();
            //     }, timeout);
            //     document.addEventListener("scatterLoaded", function () {
            //         clearTimeout(timeout);
            //         var scatter = window.scatter;
            //         // window.scatter = null;
            //         callback(true, scatter);
            //         callback.drop();
            //     });
            // }
        };
    }

    pub fn get_identity(
        &self,
        required_fields: ScatterRequiredFields,
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
                        console.log("promise error from scatter", error, required_fields);
                        console.dir(error);
                        callback(null, error.type || error.message || "Unknown error");
                        callback.drop();
                    });
            } catch (error) {
                console.log("try/catch error from scatter", error, required_fields);
                console.dir(error);
                callback(null, error.type || error.message || "Unknown error");
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
        network: ScatterNetwork,
        config: EosJsConfig,
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
