#pragma once

#include <eosio/testing/tester.hpp>
#include <eosio/chain/abi_serializer.hpp>
#include "contracts.hpp"

#include <fc/io/json.hpp>
#include <fc/variant_object.hpp>
#include <fstream>

using std::string;
using std::vector;
using namespace eosio::chain;
using namespace eosio::testing;
using namespace fc;

namespace eosstrawpoll
{
typedef name poll_name;
typedef uint32_t esptime;
} // namespace eosstrawpoll

using namespace eosstrawpoll;

struct espconfig
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
    uint16_t max_writein_len = 80;
    uint16_t max_choices_len = 100;

    // misc
    double popularity_gravity = 1.8;
    uint64_t profile_unlock_threshold = 10000; // 1 EOS
};

FC_REFLECT(
    espconfig,
    // tables
    (max_new_polls)(max_popular_polls)(max_new_donations)
    // polls fields
    (max_title_len)(max_options_len)(max_option_len)(max_account_list_len)(max_writein_len)(max_choices_len)
    // misc
    (popularity_gravity)(profile_unlock_threshold));

struct choice
{
    int16_t option_index;
    string writein;
};

FC_REFLECT(choice, (option_index)(writein));

struct espvote
{
    account_name voter;
    esptime created;
    vector<choice> choices;
    uint64_t staked = 0;
    uint64_t value = 0;
};

FC_REFLECT(espvote, (voter)(created)(choices)(staked)(value));

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
    esptime open_time;
    esptime close_time;

    // Internal
    esptime create_time;
    vector<espvote> votes;
    double popularity;

    // Indexing functions
    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return create_time; }
    uint64_t by_closed() const { return close_time; }
    double by_popularity() const { return popularity; }

    // Helper functions
    bool has_opened() const;
    bool is_closed() const;
    double calculate_popularity(double gravity) const;
};

FC_REFLECT(
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
    (votes)(popularity));

struct donation
{
    uint64_t id;
    account_name account;
    uint64_t donated;
    string memo;
    uint64_t created;

    uint64_t primary_key() const { return id; }
    uint64_t by_time() const { return created; }
};

FC_REFLECT(donation, (id)(account)(donated)(memo)(created));

struct donor
{
    account_name account;
    uint64_t donated;
    donation last_donation;

    account_name primary_key() const { return account; }
    uint64_t by_donated() const { return donated; }
};

FC_REFLECT(donor, (account)(donated)(last_donation));

struct user
{
    account_name account;
    esptime first_seen;

    account_name primary_key() const { return account; }
};

FC_REFLECT(user, (account)(first_seen));

struct preset
{
    string description;
    bool use_allow_list;
    vector<account_name> account_list;
    uint64_t min_staked;
    uint64_t min_value;
};

FC_REFLECT(preset, (description)(use_allow_list)(account_list)(min_staked)(min_value));

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
};

FC_REFLECT(
    profile,

    // basics
    (account)(url)(bio)(avatar_hash)(location)

    // unlockable social media fields
    (github_id)(twitter_id)(steem_id)(medium_id)(twitch_id)(youtube_id)(facebook_id)

    // settings
    (theme)(presets));