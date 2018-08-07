#include "eosstrawpoll.hpp"

#include <cmath>
#include <map>

namespace eosstrawpoll
{

bool poll::has_opened() const
{
    return open_time == 0 || open_time <= now();
}

bool poll::is_closed() const
{
    return close_time > 0 && close_time <= now();
}

double poll::calculate_popularity(double popularity_gravity) const
{
    const double elapsed_seconds = now() - open_time;
    const double elapsed_hours = elapsed_seconds / 60.0 / 60.0;
    return votes.size() / std::pow(elapsed_hours + 2, popularity_gravity);
}

// @abi action
void contract::createpoll(
    const account_name creator,
    const poll_name name,
    const string &title,
    const vector<string> &options,
    const vector<account_name> &whitelist,
    const vector<account_name> &blacklist,
    const uint16_t min_num_choices,
    const uint16_t max_num_choices,
    const timestamp open_time,
    const timestamp close_time,
    const string &metadata)
{
    require_auth(creator);

    const uint16_t num_options = options.size();

    eosio_assert(num_options >= 2, "must have at least 2 options");

    // check that there are no empty options
    std::map<string, bool> seen_options;
    for (auto &option : options)
    {
        eosio_assert(!option.empty(), "empty options are not allowed");
        eosio_assert(seen_options.count(option) == 0, "duplicate options are not allowed");
        seen_options[option] = true;
    }

    // // check that title is less than or equal to the max title length

    // // check that options are less than or equal to the max option length

    // // check that accounts are not in both whitelist and blacklist

    // check that close time is not before open time
    eosio_assert(
        close_time == 0 || close_time > open_time,
        "close_time must be after open_time");
    eosio_assert(
        close_time == 0 || close_time > now(),
        "close_time must not be in the past");
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

    const timestamp current_time = now();
    timestamp actual_open_time = open_time;
    if (open_time < current_time)
    {
        actual_open_time = current_time;
    }

    // create the poll
    polls_index polls(_self, creator);
    auto p = polls.emplace(creator, [&](auto &p) {
        p.id = polls.available_primary_key();
        p.creator = creator;
        p.title = title;
        p.options = options;
        p.min_num_choices = min_num_choices;
        p.max_num_choices = max_num_choices;
        p.whitelist = whitelist;
        p.blacklist = blacklist;
        p.create_time = current_time;
        p.open_time = actual_open_time;
        p.close_time = close_time;
    });

    // add poll to recently created table
    auto new_poll = _new_polls.emplace(creator, [&](auto &p2) {
        p2.id = _new_polls.available_primary_key();
        // TODO
    });

    // prune recently created table
    auto time_index = _new_polls.get_index<N(created)>();
    auto num_left = 20; //MAX_new_POLLS;
    for (auto it = time_index.rbegin(); it != time_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){time_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }

    eosio::print("successfully created poll (creator=", creator, ", name=", name, ")");
}

// @abi action
void contract::closepoll(
    const account_name creator,
    const poll_name name,
    const string &metadata)
{
    require_auth(creator);

    // check if poll exists
    polls_index polls(_self, creator);
    auto p = polls.find(name);
    eosio_assert(p != polls.end(), "poll doesn't exist");

    // check if poll is scheduled to close
    eosio_assert(p->close_time == 0, "poll is already scheduled to close");

    // check if poll is already closed
    eosio_assert(!p->is_closed(), "poll is already closed");

    // close poll
    polls.modify(p, creator, [&](auto &p) {
        p.close_time = now();
    });

    eosio::print("successfully closed poll (creator=", creator, ", name=", name, ")");
}

// @abi action
void contract::destroypoll(
    const account_name creator,
    const poll_name name,
    const string &metadata)
{
    require_auth(creator);

    // check if poll exists
    polls_index polls(_self, creator);
    auto p = polls.find(name);
    eosio_assert(p != polls.end(), "poll doesn't exist");

    // erase poll
    polls.erase(p);

    // erase poll from recent polls table
    for (auto it = _new_polls.begin(); it != _new_polls.end();)
    {
        if (it->creator == creator && it->name == name)
        {
            it = _new_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }

    // erase poll from popular polls table
    for (auto it = _popular_polls.begin(); it != _popular_polls.end();)
    {
        if (it->creator == creator && it->name == name)
        {
            it = _popular_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }

    eosio::print("successfully destroyed poll (creator=", creator, ", name=", name, ")");
}

} // namespace eosstrawpoll