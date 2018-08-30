use eos::types::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct ChainIdPrefix(String);

impl From<ChainId> for ChainIdPrefix {
    fn from(chain_id: ChainId) -> ChainIdPrefix {
        let mut chain_id = chain_id.clone();
        chain_id.truncate(12);
        ChainIdPrefix(chain_id)
    }
}

impl ToString for ChainIdPrefix {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Endpoint {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Chain {
    pub chain_id: ChainId,
    pub code_account: AccountName,
    pub eosio_token_account: AccountName,
    pub core_symbol: Symbol,
    pub endpoint: Endpoint,
}

impl From<Chain> for ChainIdPrefix {
    fn from(chain: Chain) -> ChainIdPrefix {
        chain.chain_id.clone().into()
    }
}

impl Chain {
    pub fn chain_id_prefix(&self) -> ChainIdPrefix {
        self.chain_id.clone().into()
    }
}

impl ChainIdPrefix {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub fn eos_devnet() -> Chain {
    Chain {
        chain_id: "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f".to_string(),
        code_account: "eosstrawpoll".to_string(),
        eosio_token_account: "eosio.token".to_string(),
        core_symbol: "SYS".to_string(),
        endpoint: Endpoint {
            protocol: "https".to_string(),
            host: "localhost".to_string(),
            port: 8889,
        },
    }
}

// pub static EOS_TESTNET_JUNGLE: Chain = Chain {
//     chain_id: "038f4b0fc8ff18a4f0842a8f0564611f6e96e8535901dd45e43ac8691a1c4dca".to_string(),
//     code_account: "eosstrawpoll".to_string(),
//     eosio_token_account: "eosio.token".to_string(),
//     core_symbol: "EOS".to_string(),
//     endpoint: Endpoint {
//         protocol: "http".to_string(),
//         host: "jungle.cryptolions.io".to_string(),
//         port: 18888,
//     },
// };

// pub static EOS_MAINNET: Chain = Chain {
//     chain_id: "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906".to_string(),
//     code_account: "eosstrawpoll".to_string(),
//     eosio_token_account: "eosio.token".to_string(),
//     core_symbol: "EOS".to_string(),
//     endpoint: Endpoint {
//         protocol: "https".to_string(),
//         host: "api.eosnewyork.io".to_string(),
//         port: 443,
//     },
// };

// pub static TELOS_TESTNET: Chain = Chain {
//     chain_id: "6c8aacc339bf1567743eb9c8ab4d933173aa6dca4ae6b6180a849c422f5bb207".to_string(),
//     code_account: "eosstrawpoll".to_string(),
//     eosio_token_account: "eosio.token".to_string(),
//     core_symbol: "TLOS".to_string(),
//     endpoint: Endpoint {
//         protocol: "http".to_string(),
//         host: "64.38.144.179".to_string(),
//         port: 8888,
//     },
// };

// pub static CHAINS: [&Chain; 4] = [
//     &EOS_DEVNET,
//     &EOS_TESTNET_JUNGLE,
//     &EOS_MAINNET,
//     &TELOS_TESTNET,
// ];

// pub fn find_chain(prefix: ChainIdPrefix) -> Option<&'static Chain> {
//     CHAINS
//         .iter()
//         .filter(|c| c.chain_id.starts_with(prefix.as_str()))
//         .cloned()
//         .next()
// }
