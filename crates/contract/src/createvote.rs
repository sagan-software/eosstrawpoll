use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn createvote(poll_id: PollId, account: AccountName, answers: Vec<Answer>) {
    require_auth(account);

    let code = AccountName::receiver();
    let polls = Poll::table(code, code);
    let cursor = polls.find(poll_id).assert("poll doesn't exist");

    let poll = cursor.get().assert("read");
    eosio_assert(poll.has_opened(), "poll hasn't opened yet");
    eosio_assert(!poll.is_closed(), "poll is closed");

    let num_answers = answers.len() as u16;
    eosio_assert(num_answers >= poll.min_answers, "too few answers included");
    eosio_assert(num_answers <= poll.max_answers, "too many answers included");

    // Save the vote
    let votes = Vote::table(code, poll_id);
    let cursor = votes.find(account);
    match cursor {
        Some(cursor) => {
            let mut vote = cursor.get().assert("read");
            vote.answers = answers;
            cursor.modify(None, &vote).assert("write");
        }
        None => {
            let vote = Vote {
                account,
                create_time: Time::now(),
                answers,
            };
            votes.emplace(account, &vote).assert("write");
        }
    }

    // Update popular polls table
}

fn calculate_popularity(num_votes: u32, open_time: Time, gravity: f64) -> f64 {
    let now = Time::now();
    let elapsed_seconds = now.seconds() - open_time.seconds();
    let elapsed_hours = f64::from(elapsed_seconds) / 60. / 60.;
    f64::from(num_votes) / (elapsed_hours + 2.).powf(gravity)
}

fn get_num_votes(poll_id: PollId) -> u32 {
    let code = AccountName::receiver();
    let votes = Vote::table(code, poll_id);
    match votes.lower_bound(0u64) {
        Some(cursor) => cursor.into_iter().count() as u32,
        None => 0,
    }
}
