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
    const uint16_t min_num_choices,
    const uint16_t max_num_choices,
    const vector<account_name> &whitelist,
    const vector<account_name> &blacklist,
    const time open_time,
    const time close_time,
    const string &metadata)
{
    require_auth(creator);
    eosio_assert(metadata.size() <= _config.max_metadata_size, "metadata is too long");

    // check title
    eosio_assert(!title.empty(), "title must not be empty");
    eosio_assert(title.size() <= _config.max_title_size, "title is too long");

    // check options
    const uint16_t num_options = options.size();
    eosio_assert(num_options >= 2, "must have at least 2 options");
    eosio_assert(num_options <= _config.max_options_size, "too many options");
    std::map<string, bool> seen_options;
    for (auto &option : options)
    {
        eosio_assert(!option.empty(), "empty options are not allowed");
        eosio_assert(option.size() <= _config.max_option_size, "an option is too long");
        eosio_assert(seen_options.count(option) == 0, "duplicate options are not allowed");
        seen_options[option] = true;
    }

    // check num choices
    eosio_assert(
        min_num_choices > 0,
        "min_num_choices must be greater than 0");
    eosio_assert(
        min_num_choices <= num_options,
        "min_num_choices cannot be greater than the total number of options");
    eosio_assert(
        max_num_choices >= min_num_choices,
        "max_num_choices cannot be less than min_num_choices");
    eosio_assert(
        max_num_choices <= num_options,
        "max_num_choices cannot be greater than the total number of options");

    // check times
    const time current_time = now();
    const time start_time = std::max(current_time, open_time);
    eosio_assert(
        open_time == 0 || open_time > current_time,
        "open_time must be 0 or in the future");
    eosio_assert(
        close_time == 0 || close_time > start_time + _config.min_duration,
        "poll is not open long enough");
    eosio_assert(
        close_time == 0 || close_time > open_time,
        "close_time must be 0 or after open_time");
    eosio_assert(
        close_time == 0 || close_time > current_time,
        "close_time must be 0 or in the future");

    // doesn't make sense to have a whitelist and blacklist
    auto whitelist_size = whitelist.size();
    auto blacklist_size = blacklist.size();
    if (whitelist_size > 0)
    {
        eosio_assert(blacklist_size == 0, "whitelist and blacklist cannot both be populated -- pick one or the other");
    }

    // check whitelist
    eosio_assert(
        whitelist_size <= _config.max_whitelist_size,
        "whitelist is too long");
    for (auto &account : whitelist)
    {
        eosio_assert(is_account(account), "whitelist contains an account that doesn't exist");
    }

    // check blacklist
    eosio_assert(
        blacklist_size <= _config.max_blacklist_size,
        "blacklist is too long");
    for (auto &account : blacklist)
    {
        eosio_assert(is_account(account), "blacklist contains an account that doesn't exist");
    }

    // check if poll already exists
    polls_table _creator_polls(_self, creator);
    {
        auto p = _creator_polls.find(slug);
        eosio_assert(p == _creator_polls.end(), "poll already exists with this slug");
    }

    // create the poll
    _creator_polls.emplace(creator, [&](auto &p) {
        p.id = slug;
        p.creator = creator;
        p.slug = slug;
        p.title = title;
        p.options = options;
        p.min_num_choices = min_num_choices;
        p.max_num_choices = max_num_choices;
        p.whitelist = whitelist;
        p.blacklist = blacklist;
        p.create_time = current_time;
        p.open_time = open_time;
        p.close_time = close_time;
        p.metadata = metadata;
    });

    // add to new polls table
    _new_polls.emplace(creator, [&](auto &p) {
        p.id = _new_polls.available_primary_key();
        p.creator = creator;
        p.slug = slug;
        p.title = title;
        p.options = options;
        p.min_num_choices = min_num_choices;
        p.max_num_choices = max_num_choices;
        p.whitelist = whitelist;
        p.blacklist = blacklist;
        p.create_time = current_time;
        p.open_time = open_time;
        p.close_time = close_time;
        p.metadata = metadata;
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
            p.min_num_choices = min_num_choices;
            p.max_num_choices = max_num_choices;
            p.whitelist = whitelist;
            p.blacklist = blacklist;
            p.create_time = current_time;
            p.open_time = open_time;
            p.close_time = close_time;
            p.metadata = metadata;
        });
    }
}

void contract::prune_new_polls()
{
    auto created_index = _new_polls.get_index<N(created)>();
    auto num_left = _config.max_new_polls;
    for (auto it = created_index.rbegin(); it != created_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){created_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

bool contract::is_popular_polls_full()
{
    auto num_left = _config.max_popular_polls;
    for (auto it = _popular_polls.begin(); it != _popular_polls.end();)
    {
        num_left -= 1;
        if (num_left <= 0)
        {
            return true;
        }
        ++it;
    }

    return false;
}

} // namespace eosstrawpoll