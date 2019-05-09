#include "contract.hpp"

namespace eosstrawpoll
{

void contract::createvote(
    const eosio::name &poll_id,
    const eosio::name &account,
    const std::vector<answer_t> &answers)
{
    require_auth(account);

    // check if poll exists
    auto poll = polls_table.find(poll_id.value);
    check(poll != polls_table.end(), "poll doesn't exist");

    // check if poll has opened yet
    check(poll->has_opened(), "poll has not opened yet");

    // check if poll is closed
    check(!poll->is_closed(), "poll is closed");

    const auto num_answers = answers.size();
    check(
        num_answers >= poll->min_answers,
        "too few answers included");
    check(
        num_answers <= poll->max_answers,
        "too many answers included");

    // check if answers are valid
    const auto max_option_index = poll->options.size() - 1;
    std::map<uint8_t, bool> seen_option_indexes;
    std::map<std::string, bool> seen_writeins;
    for (auto &answer : answers)
    {
        const auto option_index = answer.option_index;
        const auto writein = answer.writein;
        const bool has_option_index = option_index >= 0;
        const bool has_writein = !writein.empty();
        check(
            (has_option_index && !has_writein) ||
                (!has_option_index && has_writein),
            "invalid answer: must have either a option index OR a writein answer, not neither or both. set option index to -1 or writein to an empty string");
        check(option_index >= -1, "invalid answer: option index cannot be less than -1");

        if (has_option_index)
        {
            check(
                option_index <= max_option_index,
                "invalid answer: option index cannot be greater than the total number of options");
            check(
                seen_option_indexes.count(option_index) == 0,
                "invalid choices: duplicate option indexes are not allowed");
            seen_option_indexes[option_index] = true;
        }
        else if (has_writein)
        {
            check(
                writein.size() <= global_config.max_writein_len,
                "invalid answer: a writein answer is longer than allowed by the global config");
            check(
                seen_writeins.count(writein) == 0,
                "invalid answer: duplicate writein answers are not allowed");
            // TODO: check that writein is not all whitespace characters
            seen_writeins[writein] = true;
            check(
                seen_writeins.size() <= poll->max_writeins,
                "invalid answers: too many writein answers");
        }
    }

    const auto now = current_time_point_sec();

    // check account list
    const auto voter_list = poll->voter_list;
    if (!voter_list.empty())
    {
        const bool voter_is_on_list = std::find(voter_list.begin(), voter_list.end(), account) != voter_list.end();
        if (poll->use_allow_list)
        {
            check(voter_is_on_list, "voter is not on the allow list");
        }
        else
        {
            check(!voter_is_on_list, "voter is on the deny list");
        }
    }

    if (poll->min_voter_age > time_point_sec(0))
    {
        const auto creation_time = time_point_sec(time_point(microseconds(::get_account_creation_time(account.value))));
        const auto voter_age = time_point_sec(time_point(now - creation_time));
        check(
            voter_age >= poll->min_voter_age,
            "voter account is not old enough");
    }

    for (auto &min_voter_holding : poll->min_voter_holdings)
    {
        const auto symbol_code_raw = min_voter_holding.quantity.symbol.code().raw();
        const auto contract = min_voter_holding.contract;
        const auto token_accounts_table = token_accounts_table_t(
            contract,
            account.value);
        const auto holding = token_accounts_table.get(
            symbol_code_raw,
            "min voter holdings criteria not met -- no balance object found");
        check(
            holding.balance >= min_voter_holding.quantity,
            "min voter holdings criteria not met -- balance not high enough");
    }

    auto pollid_index = votes_table.get_index<"pollid"_n>();
    auto itr = pollid_index.lower_bound(poll_id.value);
    bool updated_vote = false;
    uint32_t num_votes = 0;
    for (; itr != pollid_index.end() && itr->poll_id == poll_id; ++itr)
    {
        if (itr->account == account)
        {
            check(
                itr->answers != answers,
                "answers have not changed");
            pollid_index.modify(itr, same_payer, [&](auto &v) {
                v.create_time = now;
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
            v.create_time = now;
            v.answers = answers;
        });
        num_votes++;
    }

    // check popular poll's table
    double lowest_popularity = 999999999;
    bool updated_poll = false;
    auto num_left = global_config.max_popular;
    for (auto itr = popular_table.begin(); itr != popular_table.end();)
    {
        const auto num_votes2 = get_num_votes(itr->id);
        const auto popularity = calculate_popularity(num_votes2, itr->open_time);

        popular_table.modify(itr, same_payer, [&](auto &p) {
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
        popular_table.emplace(_self, [&](auto &p) {
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

    prune_popular();

    auto new_poll = latest_table.find(poll_id.value);
    if (new_poll != latest_table.end())
    {
        latest_table.modify(new_poll, same_payer, [&](auto &p) {
            p.num_votes = num_votes;
            p.popularity = popularity;
        });
    }
}

} // namespace eosstrawpoll
