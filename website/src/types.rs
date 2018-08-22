use std::collections::HashMap;
use std::time::Duration;
use stdweb::web::Date;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Numeric(u64),
    Name(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poll {
    pub id: Id,
    pub creator: String,
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub min_num_choices: usize,
    pub max_num_choices: usize,
    pub votes: Vec<Vote>,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub create_time: u32,
    pub open_time: u32,
    pub close_time: u32,
    pub metadata: String,
    pub popularity: String,
}

impl Poll {
    pub fn results(&self) -> HashMap<usize, Vec<(String, usize)>> {
        let mut results: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

        for vote in &self.votes {
            for (rank, choice) in vote.choices.iter().enumerate() {
                let vote = (vote.voter.clone(), rank);
                if let Some(votes) = results.get_mut(choice) {
                    votes.push(vote);
                    continue;
                }

                results.insert(*choice, vec![vote]);
            }
        }

        results
    }

    pub fn num_votes(&self) -> usize {
        let mut total: usize = 0;
        for vote in &self.votes {
            total += vote.choices.len();
        }
        total
    }

    pub fn is_open(&self) -> bool {
        let now = (Date::now() / 1000.) as u32;
        self.open_time < now
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vote {
    pub voter: String,
    pub created: u32,
    pub staked: u64,
    pub choices: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreatePollAction {
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateVoteAction {
    pub creator: String,
    pub slug: String,
    pub voter: String,
    pub choices: Vec<usize>,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Donation {
    pub id: u64,
    pub account: String,
    pub donated: u64,
    pub memo: String,
    pub created: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Donor {
    pub account: String,
    pub donated: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAction {
    pub from: String,
    pub to: String,
    pub quantity: String,
    pub memo: String,
}
