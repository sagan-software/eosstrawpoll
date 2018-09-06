#pragma once

#include <eosiolib/multi_index.hpp>
#include <eosiolib/types.hpp>
#include <eosiolib/singleton.hpp>
#include <eosiolib/symbol.hpp>
#include <cmath>

namespace eosstrawpoll
{

using eosio::const_mem_fun;
using eosio::indexed_by;
using eosio::multi_index;
using eosio::symbol_name;
using std::string;
using std::vector;

// Custom scalars
typedef uint64_t time_t;
typedef account_name poll_id_t;

// Global configuration object
struct global_config_t
{
    // tables
    uint16_t max_new_polls = 100;
    uint16_t max_popular_polls = 100;
    uint16_t max_new_donations = 100;

    // poll fields
    uint16_t max_title_len = 100;
    uint16_t max_prefilled_options_len = 50;
    uint16_t max_prefilled_option_len = 80;
    uint16_t max_account_list_len = 300;
    uint16_t max_writein_len = 80;
    uint16_t max_answers_len = 100;

    // misc
    double popularity_gravity = 1.8;
    uint64_t profile_unlock_threshold = 10000; // 1 EOS

    EOSLIB_SERIALIZE(
        global_config_t,
        // tables
        (max_new_polls)(max_popular_polls)(max_new_donations)
        // polls fields
        (max_title_len)(max_prefilled_options_len)(max_prefilled_option_len)(max_account_list_len)(max_writein_len)(max_answers_len)
        // misc
        (popularity_gravity)(profile_unlock_threshold))
};

struct poll_t
{
    // Basics
    poll_id_t id;
    account_name account;
    string title;

    // Options & choices
    vector<string> prefilled_options;
    uint16_t min_answers;
    uint16_t max_answers;
    uint16_t max_writein_answers;

    // Voter requirements
    bool use_allow_list;
    vector<account_name> account_list;

    // Times
    time_t create_time;
    time_t open_time;
    time_t close_time;

    poll_id_t primary_key() const { return id; }
    account_name by_account() const { return account; }

    bool has_opened() const
    {
        return open_time == 0 || open_time <= now();
    }

    bool is_closed() const
    {
        return close_time > 0 && close_time <= now();
    }

    bool is_open() const
    {
        return has_opened() && !is_closed();
    }

    EOSLIB_SERIALIZE(
        poll_t,
        // basics
        (id)(account)(title)
        // voting
        (prefilled_options)(min_answers)(max_answers)(max_writein_answers)
        // voter requirements
        (use_allow_list)(account_list)
        // times
        (create_time)(open_time)(close_time))
};

struct answer_t
{
    int16_t prefilled_option_index;
    string writein;

    friend bool operator==(const answer_t &a, const answer_t &b)
    {
        return std::tie(a.prefilled_option_index, a.writein) == std::tie(b.prefilled_option_index, b.writein);
    }

    EOSLIB_SERIALIZE(answer_t, (prefilled_option_index)(writein))
};

struct vote_t
{
    uint64_t id;
    poll_id_t poll_id;
    account_name account;
    time_t create_time;
    vector<answer_t> answers;

    uint64_t primary_key() const { return id; }
    poll_id_t by_poll_id() const { return poll_id; }
    account_name by_account() const { return account; }

    EOSLIB_SERIALIZE(vote_t, (id)(poll_id)(account)(create_time)(answers))
};

struct poll_tease_t
{
    poll_id_t id;
    account_name account;
    string title;
    time_t create_time;
    time_t open_time;
    time_t close_time;
    uint32_t num_votes;
    double popularity;

    poll_id_t primary_key() const { return id; }
    time_t by_created() const { return create_time; }
    double by_popularity() const { return popularity; }

    EOSLIB_SERIALIZE(poll_tease_t, (id)(account)(title)(create_time)(open_time)(close_time)(num_votes)(popularity))
};

struct donation_t
{
    uint64_t id;
    account_name account;
    uint64_t donated;
    string memo;
    time_t create_time;

    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return create_time; }

    EOSLIB_SERIALIZE(donation_t, (id)(account)(donated)(memo)(create_time))
};

struct donor_t
{
    account_name account;
    uint64_t donated;
    donation_t first_donation;
    donation_t last_donation;

    uint64_t primary_key() const { return account; }
    uint64_t by_donated() const { return donated; }

    EOSLIB_SERIALIZE(donor_t, (account)(donated)(first_donation)(last_donation))
};

struct account_list_preset_t
{
    string description;
    vector<account_name> account_list;

    EOSLIB_SERIALIZE(account_list_preset_t, (description)(account_list))
};

struct profile_t
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
    vector<account_list_preset_t> account_list_presets;

    account_name primary_key() const { return account; }

    EOSLIB_SERIALIZE(
        profile_t,

        // basics
        (account)(url)(bio)(avatar_hash)(location)

        // unlockable social media fields
        (github_id)(twitter_id)(steem_id)(medium_id)(twitch_id)(youtube_id)(facebook_id)

        // settings
        (theme)(account_list_presets))
};

// Tables
typedef eosio::singleton<N(globalconfig), global_config_t> global_config_table_t;
typedef multi_index<
    N(polls), poll_t,
    indexed_by<N(account), const_mem_fun<poll_t, uint64_t, &poll_t::by_account>>>
    polls_table_t;
typedef multi_index<
    N(votes),
    vote_t,
    indexed_by<N(pollid), const_mem_fun<vote_t, uint64_t, &vote_t::by_poll_id>>,
    indexed_by<N(account), const_mem_fun<vote_t, uint64_t, &vote_t::by_account>>>
    votes_table_t;
typedef multi_index<
    N(popularpolls), poll_tease_t,
    indexed_by<N(popularity), const_mem_fun<poll_tease_t, double, &poll_tease_t::by_popularity>>>
    popular_polls_table_t;
typedef multi_index<
    N(newpolls), poll_tease_t,
    indexed_by<N(created), const_mem_fun<poll_tease_t, uint64_t, &poll_tease_t::by_created>>>
    new_polls_table_t;
typedef multi_index<N(newdonations), donation_t> new_donations_table_t;
typedef multi_index<N(donors), donor_t> donors_table_t;
typedef multi_index<N(profiles), profile_t> profiles_table_t;

} // namespace eosstrawpoll