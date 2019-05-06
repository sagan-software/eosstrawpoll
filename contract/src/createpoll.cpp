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
    const bool voter_list_should_allow,
    const std::vector<eosio::name> &voter_list,
    const eosio::time_point_sec &min_voter_age,
    const std::vector<eosio::extended_asset> &min_voter_holdings,
    const eosio::time_point_sec &open_time,
    const eosio::time_point_sec &close_time)
{
    require_auth(account);

    // const auto num_prefilled_options = prefilled_options.size();
    // const auto title_len = title.size();

    // eosio::check(title_len != 0, "title must not be empty");
    // eosio::check(
    //     title_len <= global_config.max_title_len,
    //     "title is longer than allowed by the global config");
    // eosio::check(
    //     min_answers <= max_answers,
    //     "min_answers cannot be greater than max_answers");
    // eosio::check(
    //     max_answers <= global_config.max_answers_len,
    //     "max_answers is larger than allowed by the global config");
    // eosio::check(
    //     min_answers > 0,
    //     "min_answers must be greater than zero");
    // eosio::check(
    //     num_prefilled_options <= global_config.max_prefilled_options_len,
    //     "prefilled_options contains more options than allowed by the global config");
    // eosio::check(
    //     max_writein_answers <= max_answers,
    //     "max_writein_answers cannot be greater than max_answers");

    // if (max_writein_answers == 0)
    // {
    //     eosio::check(
    //         max_answers <= num_prefilled_options,
    //         "max_answers cannot be greater than the number of prefilled options when writein answers are disabled");
    //     eosio::check(
    //         num_prefilled_options >= 2,
    //         "prefilled_options must contain at least two options when writein answers are disabled");
    // }
    // else
    // {
    //     eosio::check(
    //         max_writein_answers + num_prefilled_options >= max_answers,
    //         "not enough writein answers or prefilled options to satisfy max_answers requirement");
    // }

    // std::map<string, bool> seen_prefilled_options;
    // for (auto &prefilled_option : prefilled_options)
    // {
    //     // TODO: check that options are not just empty whitespace
    //     const auto prefilled_option_len = prefilled_option.size();
    //     eosio::check(
    //         prefilled_option_len != 0,
    //         "empty prefilled options are not allowed");
    //     eosio::check(
    //         prefilled_option_len <= global_config.max_prefilled_option_len,
    //         "a prefilled option is longer than allowed by the global config");
    //     eosio::check(
    //         seen_prefilled_options.count(prefilled_option) == 0,
    //         "duplicate prefilled options are not allowed");
    //     seen_prefilled_options[prefilled_option] = true;
    // }

    // // check times
    // eosio::check(
    //     close_time == 0 || close_time > open_time,
    //     "close_time must be 0 or after open_time");

    // const time_t current_time = now();
    // eosio::check(
    //     close_time == 0 || close_time > current_time,
    //     "close_time must be 0 or in the future");

    // // check account_list
    // const auto account_list_len = account_list.size();
    // eosio::check(
    //     account_list_len <= global_config.max_account_list_len,
    //     "account_list is longer than allowed by the global config");
    // for (auto &account : account_list)
    // {
    //     eosio::check(
    //         is_account(account),
    //         "account_list contains an account that doesn't exist");
    // }

    // auto poll = polls_table.find(id);
    // eosio::check(
    //     poll == polls_table.end(),
    //     "poll already exists with this ID");

    // // Poll is valid!
    // time_t start_time = open_time;
    // if (start_time < current_time)
    // {
    //     start_time = current_time;
    // }

    // // create the poll
    // polls_table.emplace(account, [&](auto &p) {
    //     p.id = id;
    //     p.account = account;
    //     p.title = title;
    //     p.prefilled_options = prefilled_options;
    //     p.min_answers = min_answers;
    //     p.max_answers = max_answers;
    //     p.max_writein_answers = max_writein_answers;
    //     p.use_allow_list = use_allow_list;
    //     p.account_list = account_list;
    //     p.create_time = current_time;
    //     p.open_time = open_time;
    //     p.close_time = close_time;
    // });

    // new_polls_table.emplace(_self, [&](auto &p) {
    //     p.id = id;
    //     p.account = account;
    //     p.title = title;
    //     p.create_time = current_time;
    //     p.open_time = open_time;
    //     p.close_time = close_time;
    // });

    // prune_new_polls();

    // if (!is_popular_polls_full())
    // {
    //     popular_polls_table.emplace(_self, [&](auto &p) {
    //         p.id = id;
    //         p.account = account;
    //         p.title = title;
    //         p.create_time = current_time;
    //         p.open_time = open_time;
    //         p.close_time = close_time;
    //     });
    // }
}

} // namespace eosstrawpoll