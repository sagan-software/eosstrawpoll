use eos::types::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use stdweb::web::Date;
use types::json::*;

pub use contract::{Answer, Poll, PollName, Vote};

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
    pub max_answers_len: usize,
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
            max_answers_len: 100,
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
            && self.max_answers_len == other.max_answers_len
            && self.popularity_gravity == other.popularity_gravity
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Poll {
//     pub id: PollName,
//     pub account: AccountName,
//     pub title: String,
//     pub prefilled_options: Vec<String>,
//     pub min_answers: usize,
//     pub max_answers: usize,
//     pub max_writein_answers: usize,
//     #[serde(deserialize_with = "bool_from_u8")]
//     pub use_allow_list: bool,
//     pub account_list: Vec<AccountName>,
//     #[serde(deserialize_with = "u64_from_string")]
//     pub create_time: u64,
//     pub open_time: u64,
//     pub close_time: u64,
// }

pub fn raw_results(poll: &Poll, votes: &[Vote]) -> HashMap<String, Vec<(AccountName, usize)>> {
    let mut results: HashMap<String, Vec<(AccountName, usize)>> = HashMap::new();

    for option in poll.prefilled_options.iter() {
        results.insert(option.to_string(), Vec::new());
    }

    for vote in votes {
        for (rank, answer) in vote.answers.iter().enumerate() {
            let vote = (vote.account.clone(), rank);

            let name = if answer.prefilled_option_index >= 0 {
                poll.prefilled_options[answer.prefilled_option_index as usize].clone()
            } else {
                answer.writein.trim().to_lowercase()
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

pub fn results_by_percent(
    poll: &Poll,
    votes: &[Vote],
) -> Vec<(String, f32, Vec<(AccountName, usize)>)> {
    let mut total_num_answers: usize = 0;
    for vote in votes {
        total_num_answers += vote.answers.len();
    }
    let raw_results = raw_results(poll, votes);
    let mut results = Vec::new();
    for (option, votes) in &raw_results {
        let percent = if !votes.is_empty() && total_num_answers > 0 {
            (votes.len() as f32) / (total_num_answers as f32)
        } else {
            0.0
        };
        results.push((option.clone(), percent, votes.clone()));
    }
    debug!(
        "!!!!!!!! raw_results: {:#?}, results: {:#?}",
        raw_results, results
    );
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
// ranked answer voting results

pub fn is_open(poll: Poll) -> bool {
    let now = (Date::now() / 1000.) as u32;
    poll.open_time.seconds() < now
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PollTease {
    pub id: PollName,
    pub account: AccountName,
    pub title: String,
    pub create_time: u64,
    pub open_time: u64,
    pub close_time: u64,
    pub num_votes: u32,
    #[serde(deserialize_with = "f64_from_string")]
    pub popularity: f64,
}

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct Vote {
//     pub id: u64,
//     pub poll_id: PollName,
//     pub account: AccountName,
//     pub create_time: u64,
//     pub answers: Vec<Answer>,
// }

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct Answer {
//     pub prefilled_option_index: i16,
//     pub writein: String,
// }

// impl PartialEq for Answer {
//     fn eq(&self, other: &Answer) -> bool {
//         self.prefilled_option_index == other.prefilled_option_index && self.writein == other.writein
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Donation {
    pub id: u64,
    pub account: AccountName,
    pub donated: u64,
    pub memo: String,
    pub create_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Donor {
    pub account: AccountName,
    pub donated: u64,
    pub first_donation: Donation,
    pub last_donation: Donation,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AccountListPreset {
    pub description: String,
    pub account_list: Vec<AccountName>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Profile {
    pub account: AccountName,
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
    pub account_list_presets: Vec<AccountListPreset>,
}
