use serde_json;
use stdweb::Value;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct ScatterService(Value);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequiredFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<RequiredAccount>>,
}

js_serializable!(RequiredFields);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequiredAccount {
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

js_serializable!(RequiredAccount);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Account {
    pub name: String,
    pub authority: String,
    pub blockchain: String,
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        self.name == other.name
            && self.authority == other.authority
            && self.blockchain == other.blockchain
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub hash: String,
    pub kyc: bool,
    pub name: String,
    pub public_key: String,
    pub accounts: Vec<Account>,
}

impl PartialEq for Identity {
    fn eq(&self, other: &Identity) -> bool {
        self.hash == other.hash
            && self.kyc == other.kyc
            && self.name == other.name
            && self.public_key == other.public_key
            && self.accounts == other.accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ScatterError {
    NotConnected,
    Locked,
    Rejected,
    Unknown,
}

impl ScatterService {
    pub fn connect(_appname: &str, callback: Callback<Option<ScatterService>>) {
        let callback = move |connected: bool, lib: Value| {
            let scatter = if connected {
                Some(ScatterService(lib))
            } else {
                None
            };
            callback.emit(scatter);
        };
        js! { @(no_return)
            var callback = @{callback};

            if (window.scatter) {
                var scatter = window.scatter;
                window.scatter = null;
                callback(true, scatter);
                callback.drop();
            } else {
                var timeout = setTimeout(function () {
                    callback(false, null);
                    callback.drop();
                }, 10000);
                document.addEventListener("scatterLoaded", function () {
                    clearTimeout(timeout);
                    var scatter = window.scatter;
                    window.scatter = null;
                    callback(true, scatter);
                    callback.drop();
                });
            }
        };
    }

    pub fn get_identity(
        &self,
        required_fields: Option<RequiredFields>,
        callback: Callback<Result<Identity, ScatterError>>,
    ) {
        let lib = self.0.as_ref();
        let callback = move |data: Option<String>, error: String| {
            let result = match (data, error.as_str()) {
                (_, "locked") => Err(ScatterError::Locked),
                (_, "identity_rejected") => Err(ScatterError::Rejected),
                (Some(data), "") => {
                    let identity = serde_json::from_str::<Identity>(&data).unwrap();
                    Ok(identity)
                }
                _ => {
                    if error.contains("Connect and Authenticate first") {
                        Err(ScatterError::NotConnected)
                    } else {
                        Err(ScatterError::Unknown)
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

    pub fn get_identity_for_chain(
        &self,
        chain_id: String,
        callback: Callback<Result<Identity, ScatterError>>,
    ) {
        let required_fields = RequiredFields {
            accounts: Some(vec![RequiredAccount {
                chain_id: Some(chain_id),
                protocol: None,
                blockchain: None,
                host: None,
                port: None,
            }]),
        };
        self.get_identity(Some(required_fields), callback);
    }

    pub fn forget_identity(&self, callback: Callback<bool>) {
        let lib = self.0.as_ref();
        let callback = move |logged_out: bool| {
            callback.emit(logged_out);
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

    pub fn identity(&self) -> Option<Identity> {
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
            Value::String(json) => serde_json::from_str::<Identity>(&json).ok(),
            _ => None,
        }
    }

    // pub fn forget_identity

    // authenticate

    // suggest_network

    // eos

    // require_version
}
