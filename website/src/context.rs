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
    pub chain_id: Option<String>,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            lang: Lang::default(),
            endpoint: "https://api.eosnewyork.io".to_string(),
            chain_id: None,
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
