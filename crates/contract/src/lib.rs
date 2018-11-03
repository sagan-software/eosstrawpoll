#![feature(proc_macro_hygiene)]

extern crate eosio;

#[cfg(feature = "serde")]
extern crate serde;

mod createpoll;
// mod createvote;
// mod destroypoll;
mod types;

pub use self::types::*;

use self::createpoll::createpoll;
// use self::createvote::createvote;
// use self::destroypoll::destroypoll;
use eosio::*;

eosio_abi!(createpoll);
