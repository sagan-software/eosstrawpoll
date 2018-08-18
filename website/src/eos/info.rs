#[derive(Deserialize, Debug)]
pub struct Info {
    pub server_version: String,
    pub chain_id: String,
    pub head_block_num: u32,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub last_irreversible_block_num: u32,
    pub last_irreversible_block_id: String,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
}
