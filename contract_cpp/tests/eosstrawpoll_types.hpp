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
typedef uint64_t esptime_t;
typedef account_name poll_id_t;
} // namespace eosstrawpoll

using namespace eosstrawpoll;

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
};

FC_REFLECT(
    global_config_t,
    // tables
    (max_new_polls)(max_popular_polls)(max_new_donations)
    // polls fields
    (max_title_len)(max_prefilled_options_len)(max_prefilled_option_len)(max_account_list_len)(max_writein_len)(max_answers_len)
    // misc
    (popularity_gravity)(profile_unlock_threshold));

struct answer_t
{
    int16_t prefilled_option_index;
    string writein;
};

FC_REFLECT(answer_t, (prefilled_option_index)(writein));

struct vote_t
{
    uint64_t id;
    poll_id_t poll_id;
    account_name account;
    esptime_t create_time;
    vector<answer_t> answers;
};

FC_REFLECT(vote_t, (id)(poll_id)(account)(create_time)(answers));

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
    esptime_t create_time;
    esptime_t open_time;
    esptime_t close_time;

    // Indexing functions
    uint64_t primary_key() const { return id; }
    uint64_t by_account() const { return account; }

    // Helper functions
    bool has_opened() const;
    bool is_closed() const;
};

FC_REFLECT(
    poll_t,
    // basics
    (id)(account)(title)
    // voting
    (prefilled_options)(min_answers)(max_answers)(max_writein_answers)
    // voter requirements
    (use_allow_list)(account_list)
    // times
    (create_time)(open_time)(close_time));

struct donation_t
{
    uint64_t id;
    account_name account;
    uint64_t donated;
    string memo;
    esptime_t create_time;

    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return create_time; }
};

FC_REFLECT(donation_t, (id)(account)(donated)(memo)(create_time));

struct donor_t
{
    account_name account;
    uint64_t donated;
    donation_t first_donation;
    donation_t last_donation;

    uint64_t primary_key() const { return account; }
    uint64_t by_donated() const { return donated; }
};

FC_REFLECT(donor_t, (account)(donated)(first_donation)(last_donation));

struct account_list_preset_t
{
    string description;
    vector<account_name> account_list;
};

FC_REFLECT(account_list_preset_t, (description)(account_list));

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
};

FC_REFLECT(
    profile_t,

    // basics
    (account)(url)(bio)(avatar_hash)(location)

    // unlockable social media fields
    (github_id)(twitter_id)(steem_id)(medium_id)(twitch_id)(youtube_id)(facebook_id)

    // settings
    (theme)(account_list_presets));