#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::createvote(
    const poll_id_t poll_id,
    const account_name account,
    const vector<answer_t> &answers)
{
    require_auth(account);

    // check if poll exists
    auto poll = polls_table.find(poll_id);
    eosio_assert(poll != polls_table.end(), "poll doesn't exist");

    // check if poll has opened yet
    eosio_assert(poll->has_opened(), "poll has not opened yet");

    // check if poll is closed
    eosio_assert(!poll->is_closed(), "poll is closed");

    const auto num_answers = answers.size();
    eosio_assert(
        num_answers >= poll->min_answers,
        "too few answers included");
    eosio_assert(
        num_answers <= poll->max_answers,
        "too many answers included");

    // check if answers are valid
    const auto max_prefilled_option_index = poll->prefilled_options.size() - 1;
    std::map<uint8_t, bool> seen_prefilled_option_indexes;
    std::map<string, bool> seen_writeins;
    for (auto &answer : answers)
    {
        const auto prefilled_option_index = answer.prefilled_option_index;
        const auto writein = answer.writein;
        const bool has_prefilled_option_index = prefilled_option_index >= 0;
        const bool has_writein = !writein.empty();
        eosio_assert(
            (has_prefilled_option_index && !has_writein) ||
                (!has_prefilled_option_index && has_writein),
            "invalid answer: must have either a prefilled option index OR a writein answer, not neither or both. set option index to -1 or writein to an empty string");
        eosio_assert(prefilled_option_index >= -1, "invalid answer: option index cannot be less than -1");

        if (has_prefilled_option_index)
        {
            eosio_assert(
                prefilled_option_index <= max_prefilled_option_index,
                "invalid answer: prefilled option index cannot be greater than the total number of prefilled options");
            eosio_assert(
                seen_prefilled_option_indexes.count(prefilled_option_index) == 0,
                "invalid choices: duplicate prefilled option indexes are not allowed");
            seen_prefilled_option_indexes[prefilled_option_index] = true;
        }
        else if (has_writein)
        {
            eosio_assert(
                writein.size() <= global_config.max_writein_len,
                "invalid answer: a writein answer is longer than allowed by the global config");
            eosio_assert(
                seen_writeins.count(writein) == 0,
                "invalid answer: duplicate writein answers are not allowed");
            // TODO: check that writein is not all whitespace characters
            seen_writeins[writein] = true;
            eosio_assert(
                seen_writeins.size() <= poll->max_writein_answers,
                "invalid answers: too many writein answers");
        }
    }

    // check account list
    const auto account_list = poll->account_list;
    if (!account_list.empty())
    {
        const bool voter_is_on_list = std::find(account_list.begin(), account_list.end(), account) != account_list.end();
        if (poll->use_allow_list)
        {
            eosio_assert(voter_is_on_list, "voter is not on the allow list");
        }
        else
        {
            eosio_assert(!voter_is_on_list, "voter is on the deny list");
        }
    }

    auto pollid_index = votes_table.get_index<N(pollid)>();
    auto itr = pollid_index.lower_bound(poll_id);
    bool updated_vote = false;
    uint32_t num_votes = 0;
    for (; itr != pollid_index.end() && itr->poll_id == poll_id; ++itr)
    {
        if (itr->account == account)
        {
            eosio_assert(
                itr->answers != answers,
                "answers have not changed");
            pollid_index.modify(itr, account, [&](auto &v) {
                v.create_time = now();
                v.answers = answers;
            });
            updated_vote = true;
        }
        num_votes++;
    }

    if (!updated_vote)
    {
        votes_table.emplace(account, [&](auto &v) {
            v.id = votes_table.available_primary_key();
            v.poll_id = poll_id;
            v.account = account;
            v.create_time = now();
            v.answers = answers;
        });
        num_votes++;
    }

    // check popular poll's table
    double lowest_popularity = 999999999;
    bool updated_poll = false;
    auto num_left = global_config.max_popular_polls;
    for (auto itr = popular_polls_table.begin(); itr != popular_polls_table.end();)
    {
        const auto num_votes2 = get_num_votes(itr->id);
        const auto popularity = calculate_popularity(num_votes2, itr->open_time);

        popular_polls_table.modify(itr, _self, [&](auto &p) {
            p.num_votes = num_votes2;
            p.popularity = popularity;
        });

        if (itr->id == poll_id)
        {
            updated_poll = true;
        }

        if (popularity < lowest_popularity)
        {
            lowest_popularity = popularity;
        }

        num_left -= 1;
        ++itr;
    }

    // check if we should put this poll in the popular poll's table
    const double popularity = calculate_popularity(num_votes, poll->open_time);
    const bool should_emplace =
        !updated_poll &&
        (num_left > 0 || popularity > lowest_popularity);
    if (should_emplace)
    {
        popular_polls_table.emplace(_self, [&](auto &p) {
            p.id = poll_id;
            p.account = poll->account;
            p.title = poll->title;
            p.create_time = poll->create_time;
            p.open_time = poll->open_time;
            p.close_time = poll->close_time;
            p.num_votes = num_votes;
            p.popularity = popularity;
        });
    }

    prune_popular_polls();

    auto new_poll = new_polls_table.find(poll_id);
    if (new_poll != new_polls_table.end())
    {
        new_polls_table.modify(new_poll, _self, [&](auto &p) {
            p.num_votes = num_votes;
            p.popularity = popularity;
        });
    }
}

} // namespace eosstrawpoll
