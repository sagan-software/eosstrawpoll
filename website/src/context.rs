use agents::api::ApiConfig;
use services::scatter::{EosConfig, Network, RequiredFields};
use std::str::FromStr;
use stdweb::unstable::TryInto;
use types::GlobalConfig;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Lang {
    English,
    Chinese,
}

impl Default for Lang {
    fn default() -> Lang {
        Lang::English
    }
}

impl FromStr for Lang {
    type Err = ();
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Lang::English)
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub lang: Lang,
    pub endpoint: String,
    pub chain_id: String,
    pub global_config: GlobalConfig,
}

impl Default for Context {
    fn default() -> Context {
        let lang: String =
            js! { return window.navigator.userLanguage || window.navigator.language }
                .try_into()
                .unwrap();

        let endpoint: String = js! { return process.env.DEFAULT_ENDPOINT }
            .try_into()
            .unwrap();

        let chain_id: String = js! { return process.env.DEFAULT_CHAIN_ID }
            .try_into()
            .unwrap();

        Context {
            lang: lang.parse().unwrap_or_else(|_| Lang::English),
            endpoint,
            chain_id,
            global_config: GlobalConfig::default(),
        }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self.lang == other.lang
            && self.endpoint == other.endpoint
            && self.chain_id == other.chain_id
    }
}

impl Context {
    pub fn network(&self) -> Network {
        Network {
            chain_id: Some(self.chain_id.clone()),
            protocol: Some("https".to_string()),
            blockchain: Some("eos".to_string()),
            host: Some("localhost".to_string()),
            port: Some(8889),
        }
    }

    pub fn eos_config(&self) -> EosConfig {
        EosConfig {
            chain_id: Some(self.chain_id.clone()),
            key_provider: None,
            http_endpoint: Some(self.endpoint.clone()),
            expire_in_seconds: None,
            broadcast: None,
            verbose: None,
            debug: None,
            sign: None,
        }
    }

    pub fn required_fields(&self) -> RequiredFields {
        RequiredFields {
            accounts: Some(vec![self.network()]),
        }
    }

    pub fn api_config(&self) -> ApiConfig {
        ApiConfig {
            endpoint: self.endpoint.clone(),
            code: "eosstrawpoll".to_string(),
            cache_timeout: 5,
        }
    }
}
