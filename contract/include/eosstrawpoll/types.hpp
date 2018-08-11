#pragma once

#include <eosiolib/multi_index.hpp>
#include <eosiolib/types.hpp>
#include <eosiolib/singleton.hpp>
#include <cmath>

namespace eosstrawpoll
{

using eosio::const_mem_fun;
using eosio::indexed_by;
using eosio::multi_index;
using std::string;
using std::vector;
typedef eosio::name poll_name;
typedef uint32_t time;

struct config
{
    uint16_t max_new_polls = 50;
    uint16_t max_popular_polls = 50;
    uint16_t max_new_donations = 100;
    uint16_t max_title_size = 144;
    uint16_t max_options_size = 50;
    uint16_t max_option_size = 144;
    uint16_t max_whitelist_size = 500;
    uint16_t max_blacklist_size = 500;
    uint32_t min_duration = 60;
    vector<account_name> blacklist;
    vector<account_name> graylist;
    double popularity_gravity = 1.8;
    uint64_t max_metadata_size = 10000;
    string metadata = "";

    EOSLIB_SERIALIZE(
        config,
        // tables
        (max_new_polls)(max_popular_polls)(max_new_donations)
        // polls fields
        (max_title_size)(max_options_size)(max_option_size)(max_whitelist_size)(max_blacklist_size)(min_duration)
        // account lists
        (blacklist)(graylist)
        // misc
        (popularity_gravity)(max_metadata_size)(metadata))
};

typedef eosio::singleton<N(config), config>
    config_table;

struct vote
{
    account_name voter;
    time created;
    uint64_t staked;
    vector<uint16_t> choices;

    EOSLIB_SERIALIZE(vote, (voter)(created)(staked)(choices))
};

struct poll
{
    uint64_t id;
    account_name creator;
    poll_name slug;
    string title;
    vector<string> options;
    uint16_t min_num_choices;
    uint16_t max_num_choices;
    vector<vote> votes;
    vector<account_name> whitelist;
    vector<account_name> blacklist;
    time create_time;
    time open_time;
    time close_time;
    string metadata;
    double popularity;

    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return create_time; }
    double by_popularity() const { return popularity; }

    bool has_opened() const
    {
        return open_time == 0 || open_time <= now();
    }

    bool is_closed() const
    {
        return close_time > 0 && close_time <= now();
    }

    double calculate_popularity(double popularity_gravity) const
    {
        const double elapsed_seconds = now() - open_time;
        const double elapsed_hours = elapsed_seconds / 60.0 / 60.0;
        return votes.size() / std::pow(elapsed_hours + 2, popularity_gravity);
    }

    EOSLIB_SERIALIZE(
        poll,
        // basics
        (id)(creator)(slug)(title)
        // voting
        (options)(min_num_choices)(max_num_choices)(votes)
        // account lists
        (whitelist)(blacklist)
        // times
        (create_time)(open_time)(close_time)
        // misc
        (metadata)(popularity))
};

typedef multi_index<N(polls), poll> polls_table;
typedef multi_index<
    N(popularpolls), poll,
    indexed_by<N(popularity), const_mem_fun<poll, double, &poll::by_popularity>>>
    popular_polls_table;
typedef multi_index<
    N(newpolls), poll,
    indexed_by<N(created), const_mem_fun<poll, uint64_t, &poll::by_created>>>
    new_polls_table;

struct donation
{
    uint64_t id;
    account_name account;
    uint64_t donated;
    string memo;
    uint64_t created;

    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return created; }

    EOSLIB_SERIALIZE(donation, (id)(account)(donated)(memo)(created))
};

typedef multi_index<
    N(newdonations), donation,
    indexed_by<N(created), const_mem_fun<donation, uint64_t, &donation::by_created>>>
    new_donations_table;

struct donor
{
    account_name account;
    uint64_t donated;

    account_name primary_key() const { return account; }
    uint64_t by_donated() const { return donated; }

    EOSLIB_SERIALIZE(donor, (account)(donated))
};

typedef multi_index<
    N(donors), donor,
    indexed_by<N(donated), const_mem_fun<donor, uint64_t, &donor::by_donated>>>
    donors_table;

} // namespace eosstrawpoll