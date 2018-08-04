
#include <algorithm>
#include <cmath>
#include <eosiolib/action.hpp>
#include <eosiolib/contract.hpp>
#include <eosiolib/eosio.hpp>
#include <eosiolib/print.hpp>
#include <eosiolib/transaction.hpp>
#include <inttypes.h>
#include <string>

#include "eosstrawpoll.hpp"

using eosio::asset;
using eosio::const_mem_fun;
using eosio::indexed_by;
using std::string;
using std::vector;

// @abi action
void eosstrawpoll::create(
    const account_name creator,
    const string &title,
    const vector<string> &options,
    const vector<account_name> &whitelist,
    const vector<account_name> &blacklist,
    const uint8_t min_num_choices,
    const uint8_t max_num_choices,
    const timestamp open_time,
    const timestamp close_time)
{
    require_auth(creator);

    // check that at least 2 options have been specified
    eosio_assert(options.size() >= 2, "must have at least 2 options");

    // check that there are no empty options
    for (auto &option : options)
    {
        eosio_assert(!option.empty(), "empty options are not allowed");
    }

    // check that there are no duplicate options
    eosio_assert(!has_duplicates(options), "duplicate options are not allowed");

    // check that title is less than or equal to the max title length

    // check that options are less than or equal to the max option length

    // check that accounts are not in both whitelist and blacklist

    // check that close time is not before open time
    eosio_assert(
        close_time == 0 || close_time > open_time,
        "close time must be after open time");

    // check that close time has not already past
    eosio_assert(
        close_time == 0 || close_time > now(),
        "close time must not be in the past");

    // create the poll
    polls_index polls(_self, creator);
    const uint8_t num_options = options.size();
    auto poll = polls.emplace(creator, [&](auto &p) {
        p.id = polls.available_primary_key();
        p.creator = creator;
        p.title = title;
        p.options = options;
        p.min_num_choices = clamp<uint8_t>(min_num_choices, 1, num_options);
        p.max_num_choices = clamp<uint8_t>(max_num_choices, p.min_num_choices, num_options);
        p.whitelist = whitelist;
        p.blacklist = blacklist;
        p.create_time = now();
        p.open_time = std::max(open_time, now());
        p.close_time = close_time;
    });

    // add poll to recently created table
    auto recent_poll = recent_polls.emplace(creator, [&](auto &p) {
        p.from_poll(*poll);
        p.id = recent_polls.available_primary_key();
    });

    // prune recently created table
    auto time_index = recent_polls.get_index<N(created)>();
    auto num_left = MAX_RECENT_POLLS;
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

    eosio::print("successfully created poll (creator=", creator, ", poll_id=", poll->id, ")");
}

// @abi action
void eosstrawpoll::close(
    const account_name creator,
    const uuid poll_id)
{
    require_auth(creator);

    // check if poll exists
    polls_index polls(_self, creator);
    auto poll = polls.find(poll_id);
    eosio_assert(poll != polls.end(), "poll doesn't exist");

    // check if poll is scheduled to close
    eosio_assert(poll->close_time == 0, "poll is already scheduled to close");

    // check if poll is already closed
    eosio_assert(!poll->is_closed(), "poll is already closed");

    // close poll
    polls.modify(poll, creator, [&](auto &p) {
        p.close_time = now();
    });

    eosio::print("successfully closed poll (creator=", creator, ", poll_id=", poll->id, ")");
}

// @abi action
void eosstrawpoll::destroy(
    const account_name creator,
    const uuid poll_id)
{
    require_auth(creator);

    // check if poll exists
    polls_index polls(_self, creator);
    auto poll = polls.find(poll_id);
    eosio_assert(poll != polls.end(), "poll doesn't exist");

    // erase poll
    polls.erase(poll);

    // erase poll from recent polls table
    for (auto it = recent_polls.begin(); it != recent_polls.end();)
    {
        if (it->poll_creator == creator && it->poll_id == poll_id)
        {
            it = recent_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }

    // erase poll from popular polls table
    for (auto it = popular_polls.begin(); it != popular_polls.end();)
    {
        if (it->poll_creator == creator && it->poll_id == poll_id)
        {
            it = popular_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }

    // erase votes from recent votes table
    for (auto it = recent_votes.begin(); it != recent_votes.end();)
    {
        if (it->poll_creator == creator && it->poll_id == poll_id)
        {
            it = recent_votes.erase(it);
        }
        else
        {
            ++it;
        }
    }

    eosio::print("successfully destroyed poll (creator=", creator, ", poll_id=", poll_id, ")");
}

// @abi action
void eosstrawpoll::vote(
    const account_name creator,
    const uuid poll_id,
    const account_name voter,
    const vector<uint8_t> &choices)
{
    require_auth(voter);

    // check if poll exists
    polls_index polls(_self, creator);
    auto poll = polls.find(poll_id);
    eosio_assert(poll != polls.end(), "poll doesn't exist");

    // check if poll has opened yet
    eosio_assert(poll->has_opened(), "poll has not opened yet");

    // check if poll is closed
    eosio_assert(!poll->is_closed(), "poll is closed");

    // check number choices can be selected
    const auto num_choices = choices.size();
    eosio_assert(num_choices >= poll->min_num_choices, "too few choices");
    eosio_assert(num_choices <= poll->max_num_choices, "too many choices");

    // check for duplicates
    eosio_assert(!has_duplicates(choices), "duplicate choices are not allowed");

    // check if choices are valid
    const auto max_choice = poll->options.size() - 1;
    for (auto &choice : choices)
    {
        char error_msg[99];
        sprintf(error_msg, "received invalid choice %d, must be within 0-%d", choice, max_choice);
        eosio_assert(choice >= 0 && choice <= max_choice, error_msg);
    }

    // check if poll has whitelist, and if voter is on whitelist
    const auto wl = poll->whitelist;
    eosio_assert(
        wl.empty() || std::find(wl.begin(), wl.end(), voter) != wl.end(),
        "voter is not on whitelist");

    // check if voter is on blacklist
    const auto bl = poll->blacklist;
    eosio_assert(
        bl.empty() || std::find(bl.begin(), bl.end(), voter) == bl.end(),
        "voter is blacklisted");

    // create vote
    vote_t v;
    v.voter = voter;
    v.choices = choices;
    v.time = now();
    v.holdings = calculate_holdings(voter);

    // cast vote
    polls.modify(poll, voter, [&](auto &p) {
        // check if voter has voted already
        for (size_t i = 0; i < p.votes.size(); i++)
        {
            const vote_t current_vote = p.votes[i];
            if (current_vote.voter == voter)
            {
                p.votes[i] = v;
                return;
            }
        }

        // new voter, add to end of vector
        p.votes.push_back(v);
    });

    // check for vote reference in recent votes table
    {
        vector<string> choice_strs{};
        for (auto &i : choices)
        {
            choice_strs.push_back(poll->options[i]);
        }

        bool updated_vote_ref = false;
        for (auto &item : recent_votes)
        {
            if (item.poll_id == poll_id && item.poll_creator == creator && item.voter == voter)
            {
                recent_votes.modify(item, voter, [&](auto &v) {
                    v.choices = choice_strs;
                });
                updated_vote_ref = true;
            }
        }

        // add vote reference to recent votes table
        if (!updated_vote_ref)
        {
            recent_votes.emplace(voter, [&](auto &v) {
                v.id = recent_votes.available_primary_key();
                v.poll_id = poll_id;
                v.poll_creator = creator;
                v.poll_title = poll->title;
                v.voter = voter;
                v.choices = choice_strs;
                v.time = now();
            });
        }

        // prune recent votes table
        auto time_index = recent_votes.get_index<N(time)>();
        auto num_left = MAX_RECENT_VOTES;
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
    };

    // check popular poll's table
    double lowest_popularity = 999999999;
    bool updated_poll = false;
    auto num_left = MAX_POPULAR_POLLS;
    for (auto poll_ref = popular_polls.begin(); poll_ref != popular_polls.end();)
    {
        updated_poll =
            updated_poll ||
            (poll_ref->poll_id == poll_id &&
             poll_ref->poll_creator == creator);

        // get the poll object from the creator's polls table
        polls_index creator_polls(_self, poll_ref->poll_creator);
        auto creator_poll = creator_polls.find(poll_ref->poll_id);

        // check if the poll exists
        if (creator_poll != creator_polls.end())
        {
            // update the poll reference
            popular_polls.modify(poll_ref, voter, [&](auto &p) {
                p.num_votes = creator_poll->votes.size();
                p.popularity = creator_poll->calculate_popularity();
            });

            // save the lowest popularity for later
            if (poll_ref->popularity < lowest_popularity)
            {
                lowest_popularity = poll_ref->popularity;
            }
            num_left -= 1;
            ++poll_ref;
        }
        else
        {
            // no poll found in the creator's polls table so erase the reference
            poll_ref = popular_polls.erase(poll_ref);
        }
    }

    // check if we should put this poll in the popular poll's table
    const double poll_popularity = poll->calculate_popularity();
    const bool should_emplace =
        !updated_poll &&
        (num_left > 0 || poll_popularity > lowest_popularity);
    if (should_emplace)
    {
        popular_polls.emplace(voter, [&](auto &p) {
            p.from_poll(*poll);
            p.id = popular_polls.available_primary_key();
        });
    }

    // prune popular table
    auto popularity_index = popular_polls.get_index<N(popularity)>();
    num_left = MAX_POPULAR_POLLS;
    for (auto it = popularity_index.rbegin(); it != popularity_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){popularity_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }

    eosio::print("successfully cast vote on poll (id=", poll_id, ", poll_ref_id=", "", ")");
}

asset eosstrawpoll::calculate_holdings(const account_name account)
{
    // get unstaked balance from table: eosio.token account accounts

    // get staked balance and RAM from table: eosio account userres

    // get exchange rate of RAM for system token from table: eosio eosio rammarket

    // sum up unstaked balance, staked balance, and RAM value

    return asset();
}

EOSIO_ABI(eosstrawpoll, (create)(close)(destroy)(vote))