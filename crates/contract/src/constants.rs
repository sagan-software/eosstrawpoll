use eosio::{n, s};

pub const GRAVITY: f64 = 1.8;
pub const MAX_POPULAR_POLLS: usize = 50;
pub const MAX_NEW_POLLS: usize = 50;
pub const MAX_TITLE_LEN: usize = 100;
pub const MAX_OPTIONS_LEN: usize = 100;
pub const MAX_OPTION_LEN: usize = 100;
pub const MAX_ACCOUNT_LIST_LEN: usize = 300;
pub const MAX_WRITEIN_LEN: usize = 80;
pub const MAX_ANSWERS_LEN: usize = 100;
pub const PROFILE_UNLOCK_THRESHOLD: i64 = 1_0000;
pub const MAX_NEW_DONATIONS: usize = 50;
pub const SYS: u64 = s!(4, SYS);
pub const EOS: u64 = s!(4, EOS);
pub const TLOS: u64 = s!(4, TLOS);
pub const POPULAR_SCOPE: u64 = n!(popular);
pub const NEW_SCOPE: u64 = n!(new);
