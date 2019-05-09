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

struct global_config_t
{
    uint64_t max_latest = 100;
    uint64_t max_popular = 100;
    uint64_t max_title_len = 100;
    uint64_t max_options_len = 50;
    uint64_t max_option_len = 80;
    uint64_t max_voter_list_len = 300;
    uint64_t max_min_voter_holdings_len = 5;
    uint64_t max_writein_len = 80;
    uint64_t max_answers_len = 100;
    double popularity_gravity = 1.8;
};

FC_REFLECT(
    global_config_t,
    (max_latest)(max_popular)(max_title_len)(max_options_len)(max_option_len)(max_voter_list_len)(max_min_voter_holdings_len)(max_writein_len)(max_answers_len)(popularity_gravity));

struct answer_t
{
    int16_t option_index;
    string writein;
};

FC_REFLECT(answer_t, (option_index)(writein));

struct vote_t
{
    uint64_t id;
    account_name poll_id;
    account_name account;
    time_point_sec create_time;
    vector<answer_t> answers;
};

FC_REFLECT(vote_t, (id)(poll_id)(account)(create_time)(answers));

struct poll_t
{
    // Basics
    account_name id;
    account_name account;
    string title;

    // Options & choices
    vector<string> options;
    uint16_t min_answers;
    uint16_t max_answers;
    uint16_t min_writeins;
    uint16_t max_writeins;

    // Voter requirements
    bool use_allow_list;
    vector<account_name> voter_list;
    time_point_sec min_voter_age;
    vector<extended_asset> min_voter_holdings;

    // Times
    time_point_sec create_time;
    time_point_sec open_time;
    time_point_sec close_time;

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
    (options)(min_answers)(max_answers)(min_writeins)(max_writeins)
    // voter requirements
    (use_allow_list)(voter_list)(min_voter_age)(min_voter_holdings)
    // times
    (create_time)(open_time)(close_time));
