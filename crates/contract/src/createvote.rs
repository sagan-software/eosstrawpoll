use crate::constants::*;
use crate::types::*;
use eosio::*;

#[eosio_action]
pub fn createvote(poll_id: PollId, account: AccountName, answers: Vec<Answer>) {
    require_auth(account);

    let _self = AccountName::receiver();
    let polls = Poll::table(_self, _self);
    let cursor = polls.find(poll_id).assert("poll doesn't exist");

    let poll = cursor.get().assert("read");
    eosio_assert(poll.has_opened(), "poll hasn't opened yet");
    eosio_assert(!poll.is_closed(), "poll is closed");

    let num_answers = answers.len() as u16;
    eosio_assert(num_answers >= poll.min_answers, "too few answers included");
    eosio_assert(num_answers <= poll.max_answers, "too many answers included");

    // Save the vote
    let votes = Vote::table(_self, poll_id);
    let cursor = votes.find(account);
    match cursor {
        Some(cursor) => {
            let mut vote = cursor.get().assert("read");
            eosio_assert(vote.answers != answers, "answers haven't changed");
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

    let num_votes = votes.count() as u32;
    let popularity = calculate_popularity(num_votes, poll.open_time);

    // Update new poll tease if it exists
    let new_polls = PollTease::table(_self, NEW_SCOPE);
    if let Some(cursor) = new_polls.find(poll_id) {
        let mut new_poll = cursor.get().assert("read");
        new_poll.num_votes = num_votes;
        new_poll.popularity = popularity;
        cursor.modify(None, &new_poll).assert("write");
    }

    let popular_polls = PollTease::table(_self, POPULAR_SCOPE);

    let mut lowest_popularity = ::std::f64::MAX;
    let mut has_updated = false;
    let mut num_popular_polls = 0;

    for cursor in popular_polls.iter() {
        let mut p = cursor.get().assert("read");

        if p.id == poll_id {
            has_updated = true;
            p.num_votes = num_votes;
        }

        p.popularity = calculate_popularity(p.num_votes, p.open_time);
        cursor.modify(None, &p).assert("write");

        if popularity < lowest_popularity {
            lowest_popularity = popularity;
        }

        num_popular_polls += 1;
    }

    if !has_updated && popularity > lowest_popularity {
        let tease = PollTease {
            id: poll.id,
            account: poll.account,
            title: poll.title,
            create_time: poll.create_time,
            open_time: poll.open_time,
            close_time: poll.close_time,
            num_votes,
            popularity,
        };
        popular_polls.emplace(_self, &tease).assert("write");
    }
}

fn calculate_popularity(num_votes: u32, open_time: Time) -> f64 {
    let now = Time::now();
    let elapsed_seconds = now.microseconds() - open_time.microseconds();
    let elapsed_hours = (elapsed_seconds as f64) / (Time::HOUR as f64);
    f64::from(num_votes) / (elapsed_hours + 1.0).powf(GRAVITY)
}
