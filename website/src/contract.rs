use services::eos::EosService;
use services::scatter;
use stdweb::Value;
use yew::callback::Callback;

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

impl Contract {
    pub fn from_eos(
        eos: EosService,
        required_fields: scatter::RequiredFields,
        callback: Callback<Result<Contract, String>>,
    ) {
        let eos = eos.to_value();
        let callback = move |error: String, value: Value| {
            let result = if error.is_empty() {
                Ok(Contract(value))
            } else {
                Err(error)
            };
            callback.emit(result);
        };
        js! { @(no_return)
            var callback = @{callback};
            try {
                var eos = @{eos};
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
