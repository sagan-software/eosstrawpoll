use services::scatter;
use std::time::Duration;
use stdweb::Value;
use yew::callback::Callback;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub max_title_len: usize,
    pub max_options_len: usize,
    pub max_option_len: usize,
    pub max_whitelist_len: usize,
    pub max_blacklist_len: usize,
    pub min_duration: Duration,
    pub blacklist: Vec<String>,
    pub graylist: Vec<String>,
}

impl Default for GlobalConfig {
    fn default() -> GlobalConfig {
        GlobalConfig {
            max_title_len: 144,
            max_options_len: 50,
            max_option_len: 144,
            max_whitelist_len: 500,
            max_blacklist_len: 500,
            min_duration: Duration::from_secs(60 * 5),
            blacklist: vec![],
            graylist: vec![],
        }
    }
}

impl PartialEq for GlobalConfig {
    fn eq(&self, other: &GlobalConfig) -> bool {
        self.max_title_len == other.max_title_len
            && self.max_options_len == other.max_options_len
            && self.max_option_len == other.max_option_len
            && self.max_whitelist_len == other.max_whitelist_len
            && self.max_blacklist_len == other.max_blacklist_len
            && self.min_duration == other.min_duration
            && self.blacklist == other.blacklist
            && self.graylist == other.graylist
    }
}

#[derive(Debug, Clone)]
pub struct Contract(Value);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreatePoll {
    pub creator: String,
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub min_num_choices: usize,
    pub max_num_choices: usize,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub open_time: u32,
    pub close_time: u32,
    pub metadata: String,
}

js_serializable!(CreatePoll);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Poll {
    pub id: String,
    pub creator: String,
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub min_num_choices: usize,
    pub max_num_choices: usize,
    pub votes: Vec<Vote>,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub open_time: u32,
    pub close_time: u32,
    pub metadata: String,
    pub popularity: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vote {
    pub voter: String,
    pub created: u32,
    pub staked: String,
    pub choices: Vec<String>,
}

impl Contract {
    pub fn from_scatter(
        scatter: scatter::ScatterService,
        network: scatter::Network,
        required_fields: scatter::RequiredFields,
        config: scatter::EosJsConfig,
        callback: Callback<Result<Contract, String>>,
    ) {
        let scatter = scatter.to_value();
        let callback = move |error: String, value: Value| {
            let result = if error.is_empty() {
                Ok(Contract(value))
            } else {
                Err(error)
            };
            callback.emit(result);
        };
        js! { @(no_return)
            var scatter = @{scatter};
            var callback = @{callback};
            try {
                var scatter = @{scatter};
                var network = @{network};
                var config = @{config};
                var Eos = require("eosjs");
                var eos = scatter.eos(network, Eos, config);
                var required_fields = @{required_fields};
                console.log("!!!!", eos, required_fields);
                eos
                    .contract("eosstrawpoll", required_fields)
                    .then(function (contract) {
                        callback("", contract);
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.log("!!!", error);
                        callback(JSON.stringify(error), "");
                        callback.drop();
                    });
            } catch (error) {
                console.log("!!!", error);
                callback(JSON.stringify(error), "");
                callback.drop();
            }
        }
    }

    pub fn createpoll(&self, params: CreatePoll, callback: Callback<Result<String, String>>) {
        let contract = self.0.as_ref();
        let callback = move |error: String, value: String| {
            info!("WTF");
            let result = if error.is_empty() {
                Ok(value)
            } else {
                Err(error)
            };
            callback.emit(result);
        };
        js! { @(no_return)
            var callback = @{callback};
            try {
                var params = @{params};
                var contract = @{contract};
                var options = {
                    authorization: params.creator + "@active",
                    broadcast: true,
                    sign: true
                };
                console.log("CREATE POLL", params, options);
                contract.createpoll(
                    params,
                    options
                )
                    .then(function (transaction) {
                        console.log(transaction);
                        callback("", JSON.stringify(transaction));
                        callback.drop();
                    })
                    .catch(function (error) {
                        console.log("!!!", error);
                        callback(JSON.stringify(error), "");
                        callback.drop();
                    });
            } catch (error) {
                console.log("!!!", error);
                callback(JSON.stringify(error), "");
                callback.drop();
            }
        }
    }
}
