use crate::types::*;
use crate::constants::*;
use eosio::*;

#[eosio_action]
pub fn createpoll(
    id: PollId,
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
    require_auth(account);

    let num_prefilled_options = prefilled_options.len() as u16;

    eosio_assert(title.len() > 0, "title must not be empty");
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

    // TODO: check each option, make sure there are no empty options or duplicates

    let create_time = Time::now();
    let open_time = open_time.max(create_time);

    let poll = Poll {
        id,
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
        create_time,
    };

    let _self = AccountName::receiver();
    let table = Poll::table(_self, _self);
    table.emplace(account, &poll).assert("write");

    let tease = PollTease {
        id,
        account,
        title: poll.title,
        create_time,
        open_time,
        close_time,
        num_votes: 0,
        popularity: 0.0,
    };

    let popular_polls = PollTease::table(_self, POPULAR_SCOPE);
    if popular_polls.count() < MAX_POPULAR_POLLS {
        popular_polls.emplace(_self, &tease).assert("write");
    }

    let new_polls = PollTease::table(_self, NEW_SCOPE);
    new_polls.emplace(_self, &tease).assert("write");
}
