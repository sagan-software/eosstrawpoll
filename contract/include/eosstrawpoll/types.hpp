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
    // tables
    uint16_t max_new_polls = 100;
    uint16_t max_popular_polls = 100;
    uint16_t max_new_donations = 100;

    // poll fields
    uint16_t max_title_len = 100;
    uint16_t max_options_len = 50;
    uint16_t max_option_len = 80;
    uint16_t max_account_list_len = 300;
    uint16_t max_writein_len = 20;
    uint16_t max_choices_len = 100;

    // misc
    double popularity_gravity = 1.8;
    uint16_t max_metadata_len = 10000;
    uint32_t profile_unlock_threshold = 10000; // 1 EOS

    EOSLIB_SERIALIZE(
        config,
        // tables
        (max_new_polls)(max_popular_polls)(max_new_donations)
        // polls fields
        (max_title_len)(max_options_len)(max_option_len)(max_account_list_len)(max_writein_len)(max_choices_len)
        // misc
        (popularity_gravity)(max_metadata_len)(profile_unlock_threshold))
};

typedef eosio::singleton<N(config), config> config_table;

struct choice
{
    int16_t option_index;
    string writein;

    EOSLIB_SERIALIZE(choice, (option_index)(writein))
}

struct vote
{
    account_name voter;
    time created;
    vector<choice> choices;
    uint64_t staked = 0;
    uint64_t value = 0;

    EOSLIB_SERIALIZE(vote, (voter)(created)(choices)(staked)(value))
};

struct poll
{
    // Basics
    uint64_t id;
    account_name creator;
    poll_name slug;
    string title;

    // Options & choices
    vector<string> options;
    uint16_t min_choices;
    uint16_t max_choices;
    uint16_t max_writeins;

    // Voter requirements
    bool use_allow_list;
    vector<account_name> account_list;
    uint64_t min_staked;
    uint64_t min_value;

    // Times
    time open_time;
    time close_time;

    // Internal
    time create_time;
    vector<vote> votes;
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
        (options)(min_choices)(max_choices)(max_writeins)
        // voter requirements
        (use_allow_list)(account_list)(min_staked)(min_value)
        // times
        (open_time)(close_time)(create_time)
        // misc
        (votes)(popularity))
};

typedef multi_index<
    N(polls), poll,
    indexed_by<N(popularity), const_mem_fun<poll, double, &poll::by_popularity>>,
    indexed_by<N(created), const_mem_fun<poll, uint64_t, &poll::by_created>>>
    polls_table;
typedef multi_index<
    N(popularpolls), poll,
    indexed_by<N(popularity), const_mem_fun<poll, double, &poll::by_popularity>>,
    indexed_by<N(created), const_mem_fun<poll, uint64_t, &poll::by_created>>>
    popular_polls_table;
typedef multi_index<
    N(newpolls), poll,
    indexed_by<N(popularity), const_mem_fun<poll, double, &poll::by_popularity>>,
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
    N(donations), donation,
    indexed_by<N(created), const_mem_fun<donation, uint64_t, &donation::by_created>>>
    donations_table;
typedef multi_index<
    N(newdonations), donation,
    indexed_by<N(created), const_mem_fun<donation, uint64_t, &donation::by_created>>>
    new_donations_table;

struct donor
{
    account_name account;
    uint64_t donated;
    donation last_donation;

    account_name primary_key() const { return account; }
    uint64_t by_donated() const { return donated; }

    EOSLIB_SERIALIZE(donor, (account)(donated)(last_donation))
};

typedef multi_index<
    N(donors), donor,
    indexed_by<N(donated), const_mem_fun<donor, uint64_t, &donor::by_donated>>>
    donors_table;

struct user
{
    account_name account;
    time first_seen;
    bool is_banned = false;
    time ban_expiration = 0;
    string ban_reason = "";
    bool is_modded = false;
    time mod_expiration = 0;

    account_name primary_key() const { return account; }

    EOSLIB_SERIALIZE(user, (account)(first_seen)(is_banned)(ban_expiration)(ban_reason)(is_modded)(mod_expiration))
};

typedef multi_index<N(users), user> users_table;

struct preset
{
    string description;
    bool use_allow_list;
    vector<account_name> account_list;
    uint64_t min_staked;
    uint64_t min_value;

    EOSLIB_SERIALIZE(preset, (description)(use_allow_list)(account_list)(min_staked)(min_value))
};

struct profile
{
    // basic fields
    account_name account;
    string url;
    string bio;
    string avatar_hash;
    string location;

    // social media fields
    string github_id;
    string twitter_id;
    string steem_id;
    string medium_id;
    string twitch_id;
    string youtube_id;
    string facebook_id;

    // settings
    string theme;
    vector<preset> presets;

    account_name primary_key() const { return account; }

    EOSLIB_SERIALIZE(
        userprofile,

        // basics
        (account)(url)(bio)(avatar_hash)(location)

        // unlockable social media fields
        (github_id)(twitter_id)(steem_id)(medium_id)(twitch_id)(youtube_id)(facebook_id)

        // settings
        (theme)(presets))
};

typedef eosio::singleton<N(profile), profile> profile_table;

} // namespace eosstrawpoll