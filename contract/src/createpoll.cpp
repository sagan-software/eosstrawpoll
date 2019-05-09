#include "contract.hpp"

#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::createpoll(
    const eosio::name &id,
    const eosio::name &account,
    const std::string &title,
    const std::vector<std::string> &options,
    const uint16_t min_answers,
    const uint16_t max_answers,
    const uint16_t min_writeins,
    const uint16_t max_writeins,
    const bool use_allow_list,
    const std::vector<eosio::name> &voter_list,
    const eosio::time_point_sec &min_voter_age,
    const std::vector<eosio::extended_asset> &min_voter_holdings,
    const eosio::time_point_sec &open_time,
    const eosio::time_point_sec &close_time)
{
    require_auth(account);

    const auto num_options = options.size();
    const auto title_len = title.size();

    check(title_len != 0, "title must not be empty");
    check(
        title_len <= global_config.max_title_len,
        "title is longer than allowed by the global config");
    check(
        min_answers <= max_answers,
        "min_answers cannot be greater than max_answers");
    check(
        max_answers <= global_config.max_answers_len,
        "max_answers is larger than allowed by the global config");
    check(
        min_answers > 0,
        "min_answers must be greater than zero");
    check(
        num_options <= global_config.max_options_len,
        "options contains more options than allowed by the global config");
    check(
        max_writeins <= max_answers,
        "max_writeins cannot be greater than max_answers");
    check(
        min_writeins <= max_writeins,
        "min_writeins cannot be greather than max_writeins");

    if (max_writeins == 0)
    {
        check(
            max_answers <= num_options,
            "max_answers cannot be greater than the number of prefilled options when writein answers are disabled");
        check(
            num_options >= 2,
            "options must contain at least two options when writein answers are disabled");
    }
    else
    {
        check(
            max_writeins + num_options >= max_answers,
            "not enough writein answers or prefilled options to satisfy max_answers requirement");
    }

    std::map<std::string, bool> seen_options;
    for (auto &option : options)
    {
        // TODO: check that options are not just empty whitespace
        const auto option_len = option.size();
        check(
            option_len != 0,
            "empty options are not allowed");
        check(
            option_len <= global_config.max_option_len,
            "an option is longer than allowed by the global config");
        check(
            seen_options.count(option) == 0,
            "duplicate options are not allowed");
        seen_options[option] = true;
    }

    // check times
    check(
        close_time == time_point_sec(0) || close_time > open_time,
        "close_time must be 0 or after open_time");

    const auto now = current_time_point_sec();
    check(
        close_time == time_point_sec(0) || close_time > now,
        "close_time must be 0 or in the future");

    auto poll = polls_table.find(id.value);
    check(
        poll == polls_table.end(),
        "poll already exists with this ID");

    // check voter_list
    check(
        voter_list.size() <= global_config.max_voter_list_len,
        "voter_list is longer than allowed by the global config");
    for (auto &account : voter_list)
    {
        check(
            is_account(account),
            "voter_list contains an account that doesn't exist");
    }

    // Check min_voter_holdings
    check(
        min_voter_holdings.size() <= global_config.max_min_voter_holdings_len,
        "min_voter_holdings is longer than allowed by the global config");
    for (auto &min_voter_holding : min_voter_holdings)
    {
        const auto symbol_code_raw = min_voter_holding.quantity.symbol.code().raw();
        const auto contract = min_voter_holding.contract;
        check(
            is_account(contract),
            "min_voter_holdings references an account that doesn't exist");
        const auto token_stats_table = token_stats_table_t(
            contract,
            symbol_code_raw);
        const auto stats = token_stats_table.get(
            symbol_code_raw,
            "min_voter_holdings references a symbol that doesn't exist");
        check(
            min_voter_holding.quantity.symbol == stats.supply.symbol,
            "min_voter_holdings symbol precision mismatch");
        check(
            min_voter_holding.quantity <= stats.supply,
            "min_voter_holdings quantity is greater than supply");
    }

    // // Poll is valid!
    // const auto start_time = open_time;
    // if (start_time < now)
    // {
    //     start_time = now;
    // }

    // create the poll
    polls_table.emplace(account, [&](auto &p) {
        p.id = id;
        p.account = account;
        p.title = title;
        p.options = options;
        p.min_answers = min_answers;
        p.max_answers = max_answers;
        p.min_writeins = min_writeins;
        p.max_writeins = max_writeins;
        p.use_allow_list = use_allow_list;
        p.voter_list = voter_list;
        p.min_voter_age = min_voter_age;
        p.min_voter_holdings = min_voter_holdings;
        p.create_time = now;
        p.open_time = open_time;
        p.close_time = close_time;
    });

    latest_table.emplace(_self, [&](auto &p) {
        p.id = id;
        p.account = account;
        p.title = title;
        p.create_time = now;
        p.open_time = open_time;
        p.close_time = close_time;
    });

    prune_latest();

    if (!is_popular_full())
    {
        popular_table.emplace(_self, [&](auto &p) {
            p.id = id;
            p.account = account;
            p.title = title;
            p.create_time = now;
            p.open_time = open_time;
            p.close_time = close_time;
        });
    }
}

} // namespace eosstrawpoll