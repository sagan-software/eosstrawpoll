use std::cmp::Ordering;
use std::collections::HashMap;
use stdweb::web::Date;
use types::json::bool_from_u8;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub max_new_polls: usize,
    pub max_popular_polls: usize,
    pub max_new_donations: usize,
    pub max_title_len: usize,
    pub max_options_len: usize,
    pub max_option_len: usize,
    pub max_account_list_len: usize,
    pub max_writein_len: usize,
    pub max_choices_len: usize,
    pub popularity_gravity: f32,
    pub profile_unlock_threshold: u64,
}

impl Default for GlobalConfig {
    fn default() -> GlobalConfig {
        GlobalConfig {
            max_new_polls: 100,
            max_popular_polls: 100,
            max_new_donations: 100,
            max_title_len: 100,
            max_options_len: 50,
            max_option_len: 80,
            max_account_list_len: 300,
            max_writein_len: 80,
            max_choices_len: 100,
            popularity_gravity: 1.8,
            profile_unlock_threshold: 10000,
        }
    }
}

impl PartialEq for GlobalConfig {
    fn eq(&self, other: &GlobalConfig) -> bool {
        self.max_new_polls == other.max_new_polls
            && self.max_popular_polls == other.max_popular_polls
            && self.max_new_donations == other.max_new_donations
            && self.max_title_len == other.max_title_len
            && self.max_options_len == other.max_options_len
            && self.max_option_len == other.max_option_len
            && self.max_account_list_len == other.max_account_list_len
            && self.max_writein_len == other.max_writein_len
            && self.max_choices_len == other.max_choices_len
            && self.popularity_gravity == other.popularity_gravity
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
    pub min_choices: usize,
    pub max_choices: usize,
    pub max_writeins: usize,
    #[serde(deserialize_with = "bool_from_u8")]
    pub use_allow_list: bool,
    pub account_list: Vec<String>,
    pub min_staked: u64,
    pub min_value: u64,
    pub open_time: u32,
    pub close_time: u32,
    pub create_time: u32,
    pub votes: Vec<Vote>,
    pub popularity: String,
}

impl Poll {
    pub fn raw_results(&self) -> HashMap<String, Vec<(String, usize)>> {
        let mut results: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for vote in &self.votes {
            for (rank, choice) in vote.choices.iter().enumerate() {
                let vote = (vote.voter.clone(), rank);

                let name = if choice.option_index >= 0 {
                    self.options[choice.option_index as usize].clone()
                } else {
                    choice.writein.trim().to_lowercase()
                };

                if let Some(votes) = results.get_mut(&name) {
                    votes.push(vote);
                    continue;
                }

                results.insert(name, vec![vote]);
            }
        }

        results
    }

    pub fn results_by_percent(&self) -> Vec<(String, f32, Vec<(String, usize)>)> {
        let num_votes = self.num_votes();
        let raw_results = self.raw_results();
        let mut results = Vec::new();
        for (option, votes) in raw_results {
            let percent = (votes.len() as f32) / (num_votes as f32);
            results.push((option.clone(), percent, votes.clone()));
        }
        results.sort_by(|(a_name, a_percent, _), (b_name, b_percent, _)| {
            let percent_ordering = b_percent.partial_cmp(&a_percent).unwrap();
            if percent_ordering == Ordering::Equal {
                a_name.cmp(b_name)
            } else {
                percent_ordering
            }
        });
        results
    }

    // results by number of votes
    // results by voter staked amounts
    // results by voter value amounts
    // ranked choice voting results

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
    pub choices: Vec<Choice>,
    pub staked: u64,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Choice {
    pub option_index: i16,
    pub writein: String,
}

impl PartialEq for Choice {
    fn eq(&self, other: &Choice) -> bool {
        self.option_index == other.option_index && self.writein == other.writein
    }
}

impl Choice {
    pub fn from_writein(writein: String) -> Choice {
        Choice {
            option_index: -1,
            writein,
        }
    }

    pub fn from_index(index: usize) -> Choice {
        Choice {
            option_index: index as i16,
            writein: "".to_string(),
        }
    }
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
    pub last_donation: Donation,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
    pub account: String,
    pub first_seen: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Preset {
    pub description: String,
    pub use_allow_list: bool,
    pub account_list: Vec<String>,
    pub min_staked: u64,
    pub min_value: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Profile {
    pub account: String,
    pub url: String,
    pub bio: String,
    pub avatar_hash: String,
    pub location: String,
    pub github_id: String,
    pub twitter_id: String,
    pub steem_id: String,
    pub medium_id: String,
    pub twitch_id: String,
    pub youtube_id: String,
    pub facebook_id: String,
    pub theme: String,
    pub presets: Vec<Preset>,
}
