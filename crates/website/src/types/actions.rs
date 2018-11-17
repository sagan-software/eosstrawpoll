use contract::{AccountListPreset, Answer, PollId};
use crate::eos::types::*;
use crate::traits::ToAction;
use crate::types::Chain;
use eosio::{n, AccountName, Action, Authorization};
use stdweb::unstable::TryInto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearProfile {
    pub account: AccountName,
}

impl ToAction for ClearProfile {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: n!(clearprofile).into(),
            authorization: vec![Authorization::active(self.account.clone())],
            data: self.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosePoll {
    pub slug: PollId,
}

// impl Action for ClosePoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "closepoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.account.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePoll {
    pub id: PollId,
    pub account: AccountName,
    pub title: String,
    pub prefilled_options: Vec<String>,
    pub min_answers: usize,
    pub max_answers: usize,
    pub max_writein_answers: usize,
    #[serde(serialize_with = "::eosio::json::bool_to_u8")]
    pub use_allow_list: bool,
    pub account_list: Vec<AccountName>,
    pub open_time: u32,
    pub close_time: u32,
}

impl Default for CreatePoll {
    fn default() -> CreatePoll {
        CreatePoll {
            id: 0.into(),
            account: 0.into(),
            title: "".to_string(),
            prefilled_options: vec!["".to_string(), "".to_string(), "".to_string()],
            min_answers: 1,
            max_answers: 1,
            max_writein_answers: 0,
            use_allow_list: true,
            account_list: Vec::new(),
            open_time: 0,
            close_time: 0,
        }
    }
}

impl ToAction for CreatePoll {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: n!(createpoll).into(),
            authorization: vec![Authorization::active(self.account.clone())],
            data: self.clone(),
        }
    }
}

impl CreatePoll {
    pub fn random_slug(&mut self) {
        let id_str: String = js! {
            var text = "";
            var possible = "abcdefghijklmnopqrstuvwxyz12345";
            for (var i = 0; i < 12; i++) {
                text += possible.charAt(Math.floor(Math.random() * possible.length));
            }
            return text;
        }
        .try_into()
        .unwrap();
        self.id = PollId::from_string(id_str).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateVote {
    pub poll_id: PollId,
    pub account: AccountName,
    pub answers: Vec<Answer>,
}

impl ToAction for CreateVote {
    fn to_action(&self, chain: &Chain) -> Action<Self> {
        Action {
            account: chain.code_account.clone(),
            name: n!(createvote).into(),
            authorization: vec![Authorization::active(self.account.clone())],
            data: self.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyPoll {
    pub poll_id: PollId,
}

// impl Action for DestroyPoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "destroypoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.account.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DestroyVote {
    pub poll_id: PollId,
    pub account: AccountName,
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
    pub poll_id: PollId,
}

// impl Action for DestroyVotes {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> ActionName {
//         "destroyvotes".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.account.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OpenPoll {
    pub poll_id: PollId,
}

// impl Action for OpenPoll {
//     fn code(&self) -> AccountName {
//         "eosstrawpoll".to_string()
//     }

//     fn name(&self) -> Name {
//         "openpoll".to_string()
//     }

//     fn actor(&self) -> AccountName {
//         self.account.clone()
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfig {
    pub max_new_polls: usize,
    pub max_popular_polls: usize,
    pub max_new_donations: usize,
    pub max_title_len: usize,
    pub max_prefilled_options_len: usize,
    pub max_prefilled_option_len: usize,
    pub max_account_list_len: usize,
    pub max_writein_len: usize,
    pub max_answers_len: usize,
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
    pub account_list_presets: Vec<AccountListPreset>,
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
            name: n!(transfer).into(),
            authorization: vec![Authorization::active(self.from.clone())],
            data: self.clone(),
        }
    }
}
