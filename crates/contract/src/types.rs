use eosio::*;

#[eosio_table(globalconfig)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GlobalConfig {
    #[primary]
    pub id: u64,
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
            id: 0,
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

eosio_name!(PollId);

#[eosio_table(polls)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Poll {
    #[primary]
    pub id: PollId,
    pub account: AccountName,
    pub title: String,
    pub prefilled_options: Vec<String>,
    pub min_answers: u16,
    pub max_answers: u16,
    pub max_writein_answers: u16,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "::eosio::json::bool_from_u8")
    )]
    pub use_allow_list: bool,
    pub account_list: Vec<AccountName>,
    pub create_time: Time,
    pub open_time: Time,
    pub close_time: Time,
}

impl Poll {
    pub fn has_opened(&self) -> bool {
        self.open_time <= Time::now()
    }

    pub fn is_closed(&self) -> bool {
        !self.close_time.is_zero() && self.close_time <= Time::now()
    }

    pub fn is_open(&self) -> bool {
        self.has_opened() && !self.is_closed()
    }
}

#[eosio_table(votes)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Vote {
    #[primary]
    pub account: AccountName,
    pub create_time: Time,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Read, Write, NumBytes, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Answer {
    pub prefilled_option_index: i16,
    pub writein: String,
}

impl From<String> for Answer {
    fn from(writein: String) -> Answer {
        Answer {
            prefilled_option_index: -1,
            writein,
        }
    }
}

impl From<usize> for Answer {
    fn from(index: usize) -> Answer {
        Answer {
            prefilled_option_index: index as i16,
            writein: "".to_string(),
        }
    }
}

impl Answer {
    pub fn from_writein(writein: String) -> Answer {
        Answer {
            prefilled_option_index: -1,
            writein,
        }
    }

    pub fn from_index(index: usize) -> Answer {
        Answer {
            prefilled_option_index: index as i16,
            writein: "".to_string(),
        }
    }
}

#[eosio_table(popularpolls)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PollTease {
    #[primary]
    pub id: PollId,
    pub account: AccountName,
    pub title: String,
    pub create_time: Time,
    pub open_time: Time,
    pub close_time: Time,
    pub num_votes: u32,
    #[secondary]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "::eosio::json::f64_from_string")
    )]
    pub popularity: f64,
}

#[eosio_table(newdonations)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Donation {
    #[primary]
    pub id: u64,
    pub account: AccountName,
    pub donated: u64,
    pub memo: String,
    pub create_time: Time,
}

#[eosio_table(donors)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Donor {
    #[primary]
    pub account: AccountName,
    pub donated: u64,
    pub first_donation: Donation,
    pub last_donation: Donation,
}

#[derive(Debug, Read, Write, NumBytes, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AccountListPreset {
    pub description: String,
    pub account_list: Vec<AccountName>,
}

#[eosio_table(profiles)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Profile {
    #[primary]
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
