use scatter::{Identity, ScatterError, ScatterService};

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

#[derive(Debug, Clone)]
pub struct Context {
    pub lang: Lang,
    pub endpoint: String,
    pub identity: Option<Result<Identity, ScatterError>>,
    pub chain_id: Option<String>,
    pub scatter: Option<Box<ScatterService>>,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            lang: Lang::default(),
            endpoint: "http://api.eosnewyork.io".to_string(),
            identity: None,
            chain_id: None,
            scatter: None,
        }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self.lang == other.lang
            && self.endpoint == other.endpoint
            && self.identity == other.identity
            && self.chain_id == other.chain_id
    }
}

impl Context {
    pub fn is_logged_in(&self) -> bool {
        match &self.identity {
            Some(ref result) => result.is_ok(),
            None => false,
        }
    }
}
