use stdweb::unstable::TryInto;
use traits::Action;
use types::json::bool_to_u8;
use types::{Choice, Preset};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearProfile {
    pub account: String,
}

impl Action for ClearProfile {
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
pub struct ClosePoll {
    pub creator: String,
    pub slug: String,
}

impl Action for ClosePoll {
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
pub struct CreatePoll {
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

impl Action for CreatePoll {
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
    pub creator: String,
    pub slug: String,
    pub voter: String,
    pub choices: Vec<Choice>,
}

impl Action for CreateVote {
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
pub struct DestroyPoll {
    pub creator: String,
    pub slug: String,
}

impl Action for DestroyPoll {
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
pub struct DestroyVote {
    pub creator: String,
    pub slug: String,
    pub voter: String,
}

impl Action for DestroyVote {
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
pub struct DestroyVotes {
    pub creator: String,
    pub slug: String,
}

impl Action for DestroyVotes {
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
pub struct OpenPoll {
    pub creator: String,
    pub slug: String,
}

impl Action for OpenPoll {
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

impl Action for SetConfig {
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
pub struct SetProfile {
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

impl Action for SetProfile {
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
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub quantity: String,
    pub memo: String,
}

impl Action for Transfer {
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
