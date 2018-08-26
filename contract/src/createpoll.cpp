#include <eosstrawpoll/contract.hpp>
#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::createpoll(
    const account_name creator,
    const poll_name slug,
    const string &title,
    const vector<string> &options,
    const uint16_t min_choices,
    const uint16_t max_choices,
    const uint16_t max_writeins,
    const bool use_allow_list,
    const vector<account_name> &account_list,
    const uint64_t min_staked,
    const uint64_t min_value,
    const time open_time,
    const time close_time)
{
    require_auth(creator);

    // check title
    eosio_assert(!title.empty(), "title must not be empty");
    eosio_assert(title.size() <= _config.max_title_len, "title is too long");

    // check options
    const uint16_t num_options = options.size();
    eosio_assert(num_options <= _config.max_options_len, "too many options");
    eosio_assert(num_options >= 2 || max_writeins >= 1, "must have at least 2 options or allow writeins");

    std::map<string, bool> seen_options;
    for (auto &option : options)
    {
        // TODO: check that options are not just empty whitespace
        eosio_assert(!option.empty(), "empty options are not allowed");
        eosio_assert(option.size() <= _config.max_option_len, "an option is too long");
        eosio_assert(seen_options.count(option) == 0, "duplicate options are not allowed");
        seen_options[option] = true;
    }

    // check num choices
    eosio_assert(
        min_choices > 0,
        "min_choices must be greater than 0");
    eosio_assert(
        min_choices <= max_choices,
        "max_choices cannot be less than min_choices");
    eosio_assert(
        max_writeins <= max_choices,
        "max_writeins cannot be greater than max_choices");
    eosio_assert(
        min_choices <= num_options + max_writeins,
        "min_choices cannot be greater than the total number of options and max writeins");
    eosio_assert(
        max_choices <= num_options + max_writeins,
        "max_choices cannot be greater than the total number of options and max writeins");
    eosio_assert(
        max_choices <= _config.max_choices_len,
        "max_choices is too large"
    );

    // check times
    eosio_assert(
        close_time == 0 || close_time > open_time,
        "close_time must be 0 or after open_time");

    const time current_time = now();
    eosio_assert(
        close_time == 0 || close_time > current_time,
        "close_time must be 0 or in the future");

    // check account_list
    auto account_list_len = account_list.size();
    eosio_assert(
        account_list_len <= _config.max_account_list_len,
        "account_list is too long");
    for (auto &account : account_list)
    {
        eosio_assert(is_account(account), "account_list contains an account that doesn't exist");
    }

    // check if poll already exists
    polls_table _creator_polls(_self, creator);
    {
        auto p = _creator_polls.find(slug);
        eosio_assert(p == _creator_polls.end(), "poll already exists with this slug");
    }

    // Poll is valid!

    // add user
    ensure_user(creator);

    // create the poll
    _creator_polls.emplace(creator, [&](auto &p) {
        p.id = slug;
        p.creator = creator;
        p.slug = slug;
        p.title = title;
        p.options = options;
        p.min_choices = min_choices;
        p.max_choices = max_choices;
        p.max_writeins = max_writeins;
        p.use_allow_list = use_allow_list;
        p.account_list = account_list;
        p.min_staked = min_staked;
        p.min_value = min_value;
        p.open_time = open_time;
        p.close_time = close_time;
        p.create_time = current_time;
    });

    // add to new polls table
    _new_polls.emplace(creator, [&](auto &p) {
        p.id = _new_polls.available_primary_key();
        p.creator = creator;
        p.slug = slug;
        p.title = title;
        p.options = options;
        p.min_choices = min_choices;
        p.max_choices = max_choices;
        p.max_writeins = max_writeins;
        p.use_allow_list = use_allow_list;
        p.account_list = account_list;
        p.min_staked = min_staked;
        p.min_value = min_value;
        p.open_time = open_time;
        p.close_time = close_time;
        p.create_time = current_time;
    });

    prune_new_polls();

    if (!is_popular_polls_full())
    {
        _popular_polls.emplace(creator, [&](auto &p) {
            p.id = _popular_polls.available_primary_key();
            p.creator = creator;
            p.slug = slug;
            p.title = title;
            p.options = options;
            p.min_choices = min_choices;
            p.max_choices = max_choices;
            p.max_writeins = max_writeins;
            p.use_allow_list = use_allow_list;
            p.account_list = account_list;
            p.min_staked = min_staked;
            p.min_value = min_value;
            p.open_time = open_time;
            p.close_time = close_time;
            p.create_time = current_time;
        });
    }
}

} // namespace eosstrawpoll