use eosio::*;

// #[eosio_table(globalconfig)]
// pub struct GlobalConfig {
//     #[primary]
//     pub id: u64,
//     pub max_new_polls: u16,
//     pub max_popular_polls: u16,
//     pub max_new_donations: u16,
//     pub max_title_len: u16,
//     pub max_prefilled_options_len: u16,
//     pub max_prefilled_option_len: u16,
//     pub max_account_list_len: u16,
//     pub max_writein_len: u16,
//     pub max_answers_len: u16,
//     pub popularity_gravity: f64,
//     pub profile_unlock_threshold: u64,
// }

// impl Default for GlobalConfig {
//     fn default() -> Self {
//         GlobalConfig {
//             id: 1,
//             max_new_polls: 100,
//             max_popular_polls: 100,
//             max_new_donations: 100,
//             max_title_len: 100,
//             max_prefilled_options_len: 50,
//             max_prefilled_option_len: 80,
//             max_account_list_len: 300,
//             max_writein_len: 80,
//             max_answers_len: 100,
//             popularity_gravity: 1.8,
//             profile_unlock_threshold: 1_0000, // 1 EOS
//         }
//     }
// }

eosio_name!(PollName);

#[eosio_table(polls)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Poll {
    #[primary]
    pub name: PollName,
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
    pub id: u64,
    #[secondary]
    pub poll: PollName,
    pub account: AccountName,
    pub create_time: Time,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Read, Write, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
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

// #[eosio_table(popularpolls)]
// pub struct PollTease {
//     #[primary]
//     pub name: PollName,
//     pub account: AccountName,
//     pub title: String,
//     pub create_time: Time,
//     pub open_time: Time,
//     pub close_time: Time,
//     pub num_votes: u32,
//     #[secondary]
//     pub popularity: f64,
// }

// impl From<Poll> for PollTease {
//     fn from(p: Poll) -> Self {
//         PollTease {
//             name: p.name,
//             account: p.account,
//             // title: p.title,
//             title: "".to_string(),
//             create_time: p.create_time,
//             open_time: p.open_time,
//             close_time: p.close_time,
//             num_votes: 0,
//             popularity: 0.0,
//         }
//     }
// }

// #[eosio_table(donations)]
// pub struct Donation {
//     #[primary]
//     pub id: u64,
//     pub account: AccountName,
//     pub donated: u64,
//     pub memo: String,
//     pub create_time: Time,
// }

// #[eosio_table(donors)]
// pub struct Donor {
//     #[primary]
//     pub account: AccountName,
//     pub donated: u64,
//     pub first_donation: Donation,
//     pub last_donation: Donation,
// }

// #[eosio_table(profiles)]
// pub struct Profile {
//     #[primary]
//     pub account: AccountName,
//     pub url: String,
//     pub bio: String,
//     pub avatar_hash: String,
//     pub location: String,
//     pub github_id: String,
//     pub twitter_id: String,
//     pub steem_id: String,
//     pub medium_id: String,
//     pub twitch_id: String,
//     pub youtube_id: String,
//     pub facebook_id: String,
//     pub theme: String,
//     pub account_list_presets: Vec<AccountListPreset>,
// }

// #[derive(Debug, Read, Write, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
// pub struct AccountListPreset {
//     pub description: String,
//     pub account_list: Vec<AccountName>,
// }
