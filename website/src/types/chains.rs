pub struct Endpoint {
    pub protocol: &'static str,
    pub host: &'static str,
    pub port: u16,
}

pub struct Chain {
    pub chain_id: &'static str,
    pub code_account: &'static str,
    pub token_account: &'static str,
    pub core_symbol: &'static str,
    pub endpoint: Endpoint,
}

const DEVNET: Chain = {
    chain_id: "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f",
    code_account: "eosstrawpoll",
    token_account: "eosio.token",
    core_symbol: "SYS",
    endpoint: Endpoint {
        protocol: "https",
        host: "localhost",
        port: 8889,
    },
};