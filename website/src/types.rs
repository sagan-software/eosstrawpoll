use serde::{Deserialize, Deserializer, Serializer};
use std::collections::HashMap;
use stdweb::unstable::TryInto;
use stdweb::web::Date;
use traits::Action;

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

fn bool_from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u8 = Deserialize::deserialize(deserializer)?;
    Ok(s == 1)
}

fn bool_to_u8<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let num = if *x { 1 } else { 0 };
    s.serialize_u8(num)
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
    pub fn results(&self) -> HashMap<String, Vec<(String, usize)>> {
        let mut results: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for vote in &self.votes {
            for (rank, choice) in vote.choices.iter().enumerate() {
                let vote = (vote.voter.clone(), rank);

                let name = if choice.option_index >= 0 {
                    self.options[choice.option_index as usize].clone()
                } else {
                    choice.writein.clone()
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearProfileAction {
    pub account: String,
}

impl Action for ClearProfileAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "clearprofile".to_string()
    }

    fn actor(&self) -> String {
        self.account.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosePollAction {
    pub creator: String,
    pub slug: String,
}

impl Action for ClosePollAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "closepoll".to_string()
    }

    fn actor(&self) -> String {
        self.creator.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePollAction {
    pub creator: String,
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub min_choices: usize,
    pub max_choices: usize,
    pub max_writeins: usize,
    #[serde(serialize_with = "bool_to_u8")]
    pub use_allow_list: bool,
    pub account_list: Vec<String>,
    pub min_staked: u64,
    pub min_value: u64,
    pub open_time: u32,
    pub close_time: u32,
}

impl Default for CreatePollAction {
    fn default() -> CreatePollAction {
        CreatePollAction {
            creator: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            options: vec!["".to_string(), "".to_string()],
            min_choices: 1,
            max_choices: 1,
            max_writeins: 0,
            use_allow_list: true,
            account_list: Vec::new(),
            min_staked: 0,
            min_value: 0,
            open_time: 0,
            close_time: 0,
        }
    }
}

impl Action for CreatePollAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "createpoll".to_string()
    }

    fn actor(&self) -> String {
        self.creator.clone()
    }
}

impl CreatePollAction {
    pub fn random_slug(&mut self) {
        self.slug = js! {
            var text = "";
            var possible = "abcdefghijklmnopqrstuvwxyz12345";
            for (var i = 0; i < 12; i++) {
                text += possible.charAt(Math.floor(Math.random() * possible.length));
            }
            return text;
        }.try_into()
        .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateVoteAction {
    pub creator: String,
    pub slug: String,
    pub voter: String,
    pub choices: Vec<Choice>,
}

impl Action for CreateVoteAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "createvote".to_string()
    }

    fn actor(&self) -> String {
        self.voter.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyPollAction {
    pub creator: String,
    pub slug: String,
}

impl Action for DestroyPollAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "destroypoll".to_string()
    }

    fn actor(&self) -> String {
        self.creator.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyVoteAction {
    pub creator: String,
    pub slug: String,
    pub voter: String,
}

impl Action for DestroyVoteAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "destroyvote".to_string()
    }

    fn actor(&self) -> String {
        self.voter.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyVotesAction {
    pub creator: String,
    pub slug: String,
}

impl Action for DestroyVotesAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "destroyvotes".to_string()
    }

    fn actor(&self) -> String {
        self.creator.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OpenPollAction {
    pub creator: String,
    pub slug: String,
}

impl Action for OpenPollAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "openpoll".to_string()
    }

    fn actor(&self) -> String {
        self.creator.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigAction {
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

impl Action for SetConfigAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "setconfig".to_string()
    }

    fn actor(&self) -> String {
        "eosstrawpoll".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SetProfileAction {
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

impl Action for SetProfileAction {
    fn code(&self) -> String {
        "eosstrawpoll".to_string()
    }

    fn name(&self) -> String {
        "setprofile".to_string()
    }

    fn actor(&self) -> String {
        self.account.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAction {
    pub from: String,
    pub to: String,
    pub quantity: String,
    pub memo: String,
}

impl Action for TransferAction {
    fn code(&self) -> String {
        "eosio.token".to_string()
    }

    fn name(&self) -> String {
        "transfer".to_string()
    }

    fn actor(&self) -> String {
        self.from.clone()
    }
}
