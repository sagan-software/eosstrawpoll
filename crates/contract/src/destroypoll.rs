use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn destroypoll(name: PollId) {
    let code = AccountName::receiver();
    let table = Poll::table(code, code);
    let cursor = table.find(name).assert("poll doesn't exist");

    let poll = cursor.get().assert("read");
    require_auth(poll.account);

    cursor.remove().assert("read");

    // let table = get_popular_table();
    // let itr = table.find(name);
    // if !table.is_end(itr) {
    //     table.erase(itr);
    // }

    // let table = get_new_table();
    // let itr = table.find(name);
    // if !table.is_end(itr) {
    //     table.erase(itr);
    // }
}
