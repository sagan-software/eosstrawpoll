use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn createpoll(
    name: PollName,
    account: AccountName,
    title: String,
    prefilled_options: Vec<String>,
    min_answers: u16,
    max_answers: u16,
    max_writein_answers: u16,
    use_allow_list: bool,
    account_list: Vec<AccountName>,
    open_time: Time,
    close_time: Time,
) {
    account.require_auth();

    let num_prefilled_options = prefilled_options.len() as u16;

    eosio_assert(title.len() != 0, "title must not be empty");
    eosio_assert(
        min_answers <= max_answers,
        "min_answers cannot be greater than max_answers",
    );
    eosio_assert(min_answers > 0, "min_answers must be greater than zero");
    eosio_assert(
        max_writein_answers <= max_answers,
        "max_writein_answers cannot be greater than max_answers",
    );

    if max_writein_answers == 0 {
        eosio_assert(
            max_answers <= num_prefilled_options,
            "max_answers cannot be greater than the number of prefilled options when writein answers are disabled");
        eosio_assert(
            num_prefilled_options >= 2,
            "prefilled_options must contain at least two options when writein answers are disabled",
        );
    } else {
        eosio_assert(
            max_writein_answers + num_prefilled_options >= max_answers,
            "not enough writein answers or prefilled options to satisfy max_answers requirement",
        );
    }

    eosio_assert(
        close_time.is_zero() || close_time > open_time,
        "close_time must be 0 or after open_time",
    );

    let poll = Poll {
        name,
        account,
        title,
        prefilled_options,
        min_answers,
        max_answers,
        max_writein_answers,
        use_allow_list,
        account_list,
        open_time,
        close_time,
        create_time: Time::now(),
    };

    let code = AccountName::receiver();
    let table = Poll::table(code, code);
    table.insert(code, &poll).assert("write");

    // let tease: PollTease = poll.into();

    // let table = get_popular_table();
    // table.emplace(code, tease.clone());

    // let table = get_new_table();
    // table.emplace(code, tease.clone());
}
