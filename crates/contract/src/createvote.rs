use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn createvote(poll: PollName, account: AccountName, answers: Vec<Answer>) {
    require_auth(account);

    let code = AccountName::receiver();
    let polls = Poll::table(code, code);
    let cursor = polls.find(poll).assert("poll doesn't exist");
    {
        let poll = cursor.get().assert("read");
        eosio_assert(poll.has_opened(), "poll hasn't opened yet");
        eosio_assert(!poll.is_closed(), "poll is closed");

        let num_answers = answers.len() as u16;
        eosio_assert(num_answers >= poll.min_answers, "too few answers included");
        eosio_assert(num_answers <= poll.max_answers, "too many answers included");
    }

    let votes = Vote::poll(code, code);
    votes
        .insert(
            account,
            &Vote {
                id: 0,
                poll,
                account,
                create_time: Time::now(),
                answers,
            },
        )
        .assert("write");
}
