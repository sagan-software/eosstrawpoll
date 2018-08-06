use std::time::Duration;

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
