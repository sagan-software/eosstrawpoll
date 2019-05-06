#pragma once

#include <eosio/eosio.hpp>
#include <eosio/singleton.hpp>
#include <eosio/asset.hpp>

#include <string>
#include <optional>

namespace eosstrawpoll
{

struct [[eosio::table]] global_state
{
    uint64_t max_new_polls = 100;
    uint64_t max_popular_polls = 100;
    uint64_t max_title_len = 100;
    uint64_t max_options_len = 50;
    uint64_t max_option_len = 80;
    uint64_t max_account_list_len = 300;
    uint64_t max_writein_len = 80;
    uint64_t max_answers_len = 100;
    double popularity_gravity = 1.8;
};

typedef eosio::singleton<"globalstate"_n, global_state> global_state_singleton;

struct [[eosio::table]] poll
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
    bool voter_list_should_allow;
    std::vector<eosio::name> voter_list;
    eosio::time_point_sec min_voter_age;
    std::vector<eosio::extended_asset> min_voter_holdings;

    // Times
    eosio::time_point_sec create_time;
    eosio::time_point_sec open_time;
    eosio::time_point_sec close_time;

    uint64_t primary_key() const { return id.value; }
    uint64_t by_account() const { return account.value; }

    // bool has_opened() const
    // {
    //     const static eosio::time_point_sec now{eosio::current_time_point()};
    //     return open_time == 0 || open_time <= now;
    // }

    // bool is_closed() const
    // {
    //     const static eosio::time_point_sec now{eosio::current_time_point()};
    //     return close_time > 0 && close_time <= now();
    // }

    // bool is_open() const
    // {
    //     return has_opened() && !is_closed();
    // }
};

typedef eosio::multi_index<
    "polls"_n, poll,
    eosio::indexed_by<"account"_n, eosio::const_mem_fun<poll, uint64_t, &poll::by_account>>>
    polls_table;

struct answer
{
    int16_t option_index;
    std::string writein;

    friend bool operator==(const answer &a, const answer &b)
    {
        return a.option_index == b.option_index && a.writein == b.writein;
    }
};

struct [[eosio::table]] vote
{
    uint64_t id;
    eosio::name poll_id;
    eosio::name account;
    eosio::time_point_sec create_time;
    std::vector<answer> answers;

    uint64_t primary_key() const { return id; }
    uint64_t by_poll_id() const { return poll_id.value; }
    uint64_t by_account() const { return account.value; }
};

typedef eosio::multi_index<
    "votes"_n,
    vote,
    eosio::indexed_by<"pollid"_n, eosio::const_mem_fun<vote, uint64_t, &vote::by_poll_id>>,
    eosio::indexed_by<"account"_n, eosio::const_mem_fun<vote, uint64_t, &vote::by_account>>>
    votes_table;

struct [[eosio::table]] tease
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
    "teases"_n, tease,
    eosio::indexed_by<"popularity"_n, eosio::const_mem_fun<tease, double, &tease::by_popularity>>,
    eosio::indexed_by<"created"_n, eosio::const_mem_fun<tease, uint64_t, &tease::by_created>>>
    teases_table;

class[[eosio::contract]] contract : public eosio::contract
{

private:
    global_state_singleton _global;
    global_state _gstate;
    polls_table _polls;
    votes_table _votes;
    teases_table _popular;
    teases_table _latest;

    // // utils
    // void prune_new_polls();
    // void prune_popular_polls();
    // bool is_popular_polls_full();
    // void ensure_user(const eosio::name account);
    // uint32_t get_num_votes(const eosio::name poll_id);
    // double calculate_popularity(const uint32_t num_votes, const eosio::time_point_sec start_time);

public:
    contract(eosio::name s, eosio::name code, eosio::datastream<const char *> ds);

    // ACTION closepoll(const eosio::name poll_id);

    [[eosio::action]] void createpoll(
        const eosio::name &id,
        const eosio::name &account,
        const std::string &title,
        const std::vector<std::string> &options,
        const uint16_t min_answers,
        const uint16_t max_answers,
        const uint16_t min_writeins,
        const uint16_t max_writeins,
        const bool use_allow_list,
        const std::vector<eosio::name> &account_list,
        const eosio::time_point_sec &min_account_age,
        const std::vector<eosio::extended_asset> &min_holdings,
        const eosio::time_point_sec &open_time,
        const eosio::time_point_sec &close_time);

    [[eosio::action]] void createvote(
        const eosio::name &poll_id,
        const eosio::name &account,
        const std::vector<answer> &answers);

    // ACTION destroypoll(const eosio::name poll_id);

    // ACTION destroyvote(
    //     const eosio::name poll_id,
    //     const eosio::name account);

    // ACTION destroyvotes(const eosio::name poll_id);

    // ACTION openpoll(const eosio::name poll_id);

    // ACTION setconfig(
    //     const uint16_t max_new_polls,
    //     const uint16_t max_popular_polls,
    //     const uint16_t max_new_donations,
    //     const uint16_t max_title_len,
    //     const uint16_t max_writein_len,
    //     const uint16_t max_answers_len,
    //     const double popularity_gravity);
};

} // namespace eosstrawpoll