use prelude::*;
use std::str::FromStr;
use stdweb::unstable::TryInto;

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
    pub selected_chain: Chain,
    pub available_chains: Vec<Chain>,
}

impl Default for Context {
    fn default() -> Context {
        let lang: String =
            js! { return window.navigator.userLanguage || window.navigator.language }
                .try_into()
                .unwrap();

        Context {
            lang: lang.parse().unwrap_or_else(|_| Lang::English),
            selected_chain: eos_testnet_jungle(),
            available_chains: vec![
                eos_testnet_jungle(),
                // eos_devnet(),
                // telos_devnet(),
                telos_testnet(),
            ],
        }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self.lang == other.lang
            && self.selected_chain == other.selected_chain
            && self.available_chains == other.available_chains
    }
}

impl Context {
    pub fn find_chain(&self, prefix: &ChainIdPrefix) -> Option<Chain> {
        self.available_chains
            .iter()
            .filter(|c| c.chain_id.starts_with(prefix.as_str()))
            .cloned()
            .next()
    }
}
