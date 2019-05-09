#pragma once

#include <eosiolib/eosio.hpp>
#include <eosiolib/singleton.hpp>
#include <eosiolib/asset.hpp>
#include <eosiolib/time.hpp>
#include <eosiolib/permission.h>

#include <string>
#include <optional>

namespace eosstrawpoll
{

using namespace eosio;

time_point current_time_point();
time_point_sec current_time_point_sec();

struct [[eosio::table]] global_config_t
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

typedef eosio::singleton<"globalconfig"_n, global_config_t> global_config_singleton_t;

struct [[eosio::table]] poll_t
{
    // Basics
    eosio::name id;
    eosio::name account;
    std::string title;

    // Options & choices
    std::vector<std::string> options;
    uint16_t min_answers;
    uint16_t max_answers;
    uint16_t min_writeins;
    uint16_t max_writeins;

    // Voter requirements
    bool use_allow_list;
    std::vector<eosio::name> voter_list;
    eosio::time_point_sec min_voter_age;
    std::vector<eosio::extended_asset> min_voter_holdings;

    // Times
    eosio::time_point_sec create_time;
    eosio::time_point_sec open_time;
    eosio::time_point_sec close_time;

    uint64_t primary_key() const { return id.value; }
    uint64_t by_account() const { return account.value; }

    bool has_opened() const
    {
        const static auto now = current_time_point_sec();
        return open_time == time_point_sec(0) || open_time <= now;
    }

    bool is_closed() const
    {
        const static auto now = current_time_point_sec();
        return close_time > time_point_sec(0) && close_time <= now;
    }

    bool is_open() const
    {
        return has_opened() && !is_closed();
    }
};

typedef eosio::multi_index<
    "polls"_n, poll_t,
    eosio::indexed_by<"account"_n, eosio::const_mem_fun<poll_t, uint64_t, &poll_t::by_account>>>
    polls_table_t;

struct answer_t
{
    int16_t option_index;
    std::string writein;

    friend bool operator==(const answer_t &a, const answer_t &b)
    {
        return a.option_index == b.option_index && a.writein == b.writein;
    }
};

struct [[eosio::table]] vote_t
{
    uint64_t id;
    eosio::name poll_id;
    eosio::name account;
    eosio::time_point_sec create_time;
    std::vector<answer_t> answers;

    uint64_t primary_key() const { return id; }
    uint64_t by_poll_id() const { return poll_id.value; }
    uint64_t by_account() const { return account.value; }
};

typedef eosio::multi_index<
    "votes"_n,
    vote_t,
    eosio::indexed_by<"pollid"_n, eosio::const_mem_fun<vote_t, uint64_t, &vote_t::by_poll_id>>,
    eosio::indexed_by<"account"_n, eosio::const_mem_fun<vote_t, uint64_t, &vote_t::by_account>>>
    votes_table_t;

struct [[eosio::table]] tease_t
{
    eosio::name id;
    eosio::name account;
    std::string title;
    eosio::time_point_sec create_time;
    eosio::time_point_sec open_time;
    eosio::time_point_sec close_time;
    uint32_t num_votes;
    double popularity;

    uint64_t primary_key() const { return id.value; }
    uint64_t by_created() const { return create_time.sec_since_epoch(); }
    double by_popularity() const { return popularity; }
};

typedef eosio::multi_index<
    "popular"_n, tease_t,
    eosio::indexed_by<"popularity"_n, eosio::const_mem_fun<tease_t, double, &tease_t::by_popularity>>>
    popular_table_t;

typedef eosio::multi_index<
    "latest"_n, tease_t,
    eosio::indexed_by<"created"_n, eosio::const_mem_fun<tease_t, uint64_t, &tease_t::by_created>>>
    latest_table_t;

struct [[eosio::table]] token_account_t
{
    asset balance;

    uint64_t primary_key() const { return balance.symbol.code().raw(); }
};

struct [[eosio::table]] token_stats_t
{
    asset supply;
    asset max_supply;
    name issuer;

    uint64_t primary_key() const { return supply.symbol.code().raw(); }
};

typedef eosio::multi_index<
    "accounts"_n, token_account_t>
    token_accounts_table_t;
typedef eosio::multi_index<
    "stat"_n, token_stats_t>
    token_stats_table_t;

class[[eosio::contract]] contract : public eosio::contract
{

private:
    global_config_singleton_t global_config_singleton;
    global_config_t global_config;
    polls_table_t polls_table;
    votes_table_t votes_table;
    popular_table_t popular_table;
    latest_table_t latest_table;

    // utils

    void prune_latest();
    void prune_popular();
    bool is_popular_full();
    uint32_t get_num_votes(const eosio::name poll_id);
    double calculate_popularity(const uint32_t num_votes, const eosio::time_point_sec start_time);

public:
    contract(eosio::name s, eosio::name code, eosio::datastream<const char *> ds);

    ACTION closepoll(const eosio::name poll_id);

    ACTION createpoll(
        const eosio::name &id,
        const eosio::name &account,
        const std::string &title,
        const std::vector<std::string> &options,
        const uint16_t min_answers,
        const uint16_t max_answers,
        const uint16_t min_writeins,
        const uint16_t max_writeins,
        const bool use_allow_list,
        const std::vector<eosio::name> &voter_list,
        const eosio::time_point_sec &min_account_age,
        const std::vector<eosio::extended_asset> &min_holdings,
        const eosio::time_point_sec &open_time,
        const eosio::time_point_sec &close_time);

    ACTION createvote(
        const eosio::name &poll_id,
        const eosio::name &account,
        const std::vector<answer_t> &answers);

    ACTION destroypoll(const eosio::name poll_id);

    ACTION destroyvote(
        const eosio::name poll_id,
        const eosio::name account);

    ACTION destroyvotes(const eosio::name poll_id);

    ACTION openpoll(const eosio::name poll_id);

    ACTION setconfig(
        const uint64_t max_latest,
        const uint64_t max_popular,
        const uint64_t max_title_len,
        const uint64_t max_options_len,
        const uint64_t max_option_len,
        const uint64_t max_voter_list_len,
        const uint64_t max_min_voter_holdings_len,
        const uint64_t max_writein_len,
        const uint64_t max_answers_len,
        const double popularity_gravity);
};

} // namespace eosstrawpoll