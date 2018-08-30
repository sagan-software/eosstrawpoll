use eos::types::*;
use stdweb::unstable::TryInto;
use traits::ToAction;
use types::json::bool_to_u8;
use types::{Chain, Choice, PollName, Preset};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearProfile {
    pub account: AccountName,
}

impl ToAction for ClearProfile {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: "clearprofile".into(),
            authorization: vec![Authorization::active(self.account.clone())],
            data: self.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosePoll {
    pub creator: AccountName,
    pub slug: PollName,
}

// impl Action for ClosePoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "closepoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.creator.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePoll {
    pub creator: AccountName,
    pub slug: PollName,
    pub title: String,
    pub options: Vec<String>,
    pub min_choices: usize,
    pub max_choices: usize,
    pub max_writeins: usize,
    #[serde(serialize_with = "bool_to_u8")]
    pub use_allow_list: bool,
    pub account_list: Vec<AccountName>,
    pub min_staked: u64,
    pub min_value: u64,
    pub open_time: u32,
    pub close_time: u32,
}

impl Default for CreatePoll {
    fn default() -> CreatePoll {
        CreatePoll {
            creator: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            options: vec!["".to_string(), "".to_string(), "".to_string()],
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

impl ToAction for CreatePoll {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: "createpoll".into(),
            authorization: vec![Authorization::active(self.creator.clone())],
            data: self.clone(),
        }
    }
}

impl CreatePoll {
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
pub struct CreateVote {
    pub creator: AccountName,
    pub slug: PollName,
    pub voter: AccountName,
    pub choices: Vec<Choice>,
}

impl ToAction for CreateVote {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: "createvote".into(),
            authorization: vec![Authorization::active(self.voter.clone())],
            data: self.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyPoll {
    pub creator: AccountName,
    pub slug: PollName,
}

// impl Action for DestroyPoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "destroypoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.creator.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyVote {
    pub creator: AccountName,
    pub slug: Name,
    pub voter: AccountName,
}

// impl Action for DestroyVote {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "destroyvote".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.voter.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyVotes {
    pub creator: AccountName,
    pub slug: PollName,
}

// impl Action for DestroyVotes {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "destroyvotes".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.creator.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OpenPoll {
    pub creator: AccountName,
    pub slug: PollName,
}

// impl Action for OpenPoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> Name {
//         "openpoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.creator.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfig {
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

// impl Action for SetConfig {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> Name {
//         "setconfig".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SetProfile {
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
    pub presets: Vec<Preset>,
}

// impl Action for SetProfile {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> Name {
//         "setprofile".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.account.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transfer {
    pub from: AccountName,
    pub to: AccountName,
    pub quantity: Asset,
    pub memo: String,
}

impl ToAction for Transfer {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.eosio_token_account.clone(),
            name: "transfer".into(),
            authorization: vec![Authorization::active(self.from.clone())],
            data: self.clone(),
        }
    }
}
