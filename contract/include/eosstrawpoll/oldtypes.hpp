#pragma once

#include <eosiolib/multi_index.hpp>
#include <eosiolib/types.hpp>
#include <eosiolib/singleton.hpp>
#include <eosiolib/symbol.hpp>
#include <cmath>

namespace old
{

using eosio::const_mem_fun;
using eosio::indexed_by;
using eosio::multi_index;
using eosio::symbol_name;
using std::string;
using std::vector;

struct choice
{
    int16_t option_index;
    string writein;

    EOSLIB_SERIALIZE(choice, (option_index)(writein))
};

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
    account_name slug;
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

struct user
{
    account_name account;
    time first_seen;

    account_name primary_key() const { return account; }

    EOSLIB_SERIALIZE(user, (account)(first_seen))
};

typedef multi_index<N(users), user> users_table;

} // namespace old