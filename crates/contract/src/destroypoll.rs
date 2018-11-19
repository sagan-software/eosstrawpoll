use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn destroypoll(name: PollId) {
    let code = AccountName::receiver();
    let table = Poll::table(code, code);
    let cursor = table.find(name).assert("poll doesn't exist");

    let poll = cursor.get().assert("read");
    require_auth(poll.account);

    cursor.erase().assert("erase");

    let table = PollTease::table(code, code);
    match table.find(name) {
        Some(c) => c.erase().assert("erase"),
        None => return,
    };
}
