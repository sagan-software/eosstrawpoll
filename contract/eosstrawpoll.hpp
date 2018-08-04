#pragma once

#include <eosiolib/action.hpp>
#include <eosiolib/asset.hpp>
#include <eosiolib/contract.hpp>
#include <eosiolib/eosio.hpp>
#include <eosiolib/multi_index.hpp>
#include <eosiolib/print.hpp>
#include <eosiolib/singleton.hpp>
#include <sstream>
#include <string>

using eosio::asset;
using eosio::const_mem_fun;
using eosio::indexed_by;
using std::set;
using std::string;
using std::vector;

typedef uint64_t uuid;
typedef uint32_t timestamp;

constexpr uint8_t MAX_RECENT_POLLS = 5;
constexpr uint8_t MAX_POPULAR_POLLS = 5;
constexpr uint8_t MAX_RECENT_VOTES = 5;
constexpr uint8_t MAX_NUM_CHOICES = 50;
constexpr uint8_t MAX_TITLE_LENGTH = 100;
constexpr uint8_t MAX_OPTION_LENGTH = 200;
constexpr float POPULARITY_GRAVITY = 1.8f;

template <typename T>
T clamp(const T &n, const T &lower, const T &upper)
{
    return std::max(lower, std::min(n, upper));
}

template <typename T>
bool has_duplicates(const vector<T> &items)
{
    vector<T> sorted(items);
    auto end = sorted.end();
    std::sort(sorted.begin(), end);
    return std::adjacent_find(sorted.begin(), end) != end;
}

struct vote_t
{
    account_name voter;
    timestamp time;
    asset holdings;
    vector<uint8_t> choices;

    EOSLIB_SERIALIZE(vote_t, (voter)(time)(holdings)(choices))
};

// @abi table polls i64
struct poll_t
{
    uuid id;
    account_name creator;
    string title;
    vector<string> options;
    vector<vote_t> votes;
    uint8_t min_num_choices;
    uint8_t max_num_choices;
    vector<account_name> whitelist;
    vector<account_name> blacklist;
    timestamp create_time;
    timestamp open_time;
    timestamp close_time;

    uuid primary_key() const { return id; }

    bool has_opened() const { return open_time == 0 || open_time <= now(); }

    bool is_closed() const { return close_time > 0 && close_time <= now(); }

    double calculate_popularity() const
    {
        const double elapsed_seconds = now() - open_time;
        const double elapsed_hours = elapsed_seconds / 60.0 / 60.0;
        return votes.size() / std::pow(elapsed_hours + 2, POPULARITY_GRAVITY);
    }

    EOSLIB_SERIALIZE(poll_t, (id)(creator)(title)(options)(votes)(min_num_choices)(max_num_choices)(whitelist)(blacklist)(create_time)(open_time)(close_time))
};

typedef eosio::multi_index<N(polls), poll_t> polls_index;

// @abi table popularpolls i64
// @abi table recentpolls i64
// @abi table closedpolls i64
struct poll_ref_t
{
    uuid id;
    uuid poll_id;
    account_name poll_creator;
    string title;
    vector<string> options;
    uint64_t num_votes;
    double popularity;
    vector<account_name> whitelist;
    vector<account_name> blacklist;
    timestamp create_time;
    timestamp open_time;
    timestamp close_time;

    uuid primary_key() const { return id; }
    uint64_t by_time() const { return create_time; }
    double by_popularity() const { return popularity; }

    void from_poll(const poll_t poll)
    {
        poll_id = poll.id;
        poll_creator = poll.creator;
        title = poll.title;
        options = poll.options;
        num_votes = poll.votes.size();
        popularity = poll.calculate_popularity();
        whitelist = poll.whitelist;
        blacklist = poll.blacklist;
        create_time = poll.create_time;
        open_time = poll.open_time;
        close_time = poll.close_time;
    }

    friend bool operator==(const poll_ref_t &poll_ref, const poll_t &poll)
    {
        return poll_ref.poll_id == poll.id && poll_ref.poll_creator == poll.creator;
    }

    friend bool operator!=(const poll_ref_t &poll_ref, const poll_t &poll)
    {
        return !(poll_ref == poll);
    }

    EOSLIB_SERIALIZE(poll_ref_t, (id)(poll_id)(poll_creator)(title)(options)(num_votes)(popularity)(whitelist)(blacklist)(create_time)(open_time)(close_time))
};

// @abi table recentvotes i64
struct vote_ref_t
{
    uuid id;
    uuid poll_id;
    account_name poll_creator;
    string poll_title;
    account_name voter;
    vector<string> choices;
    timestamp time;

    uuid primary_key() const { return id; }
    uint64_t by_time() const { return time; }

    EOSLIB_SERIALIZE(vote_ref_t, (id)(poll_id)(poll_creator)(poll_title)(voter)(choices)(time))
};

typedef eosio::multi_index<
    N(recentvotes), vote_ref_t,
    eosio::indexed_by<N(time), const_mem_fun<vote_ref_t, uint64_t, &vote_ref_t::by_time>>>
    recent_votes_index;

class eosstrawpoll : public eosio::contract
{

  public:
    eosio::multi_index<
        N(recentpolls), poll_ref_t,
        eosio::indexed_by<N(created), const_mem_fun<poll_ref_t, uint64_t, &poll_ref_t::by_time>>>
        recent_polls;
    eosio::multi_index<
        N(popularpolls), poll_ref_t,
        eosio::indexed_by<N(popularity), const_mem_fun<poll_ref_t, double, &poll_ref_t::by_popularity>>>
        popular_polls;
    recent_votes_index recent_votes;

    eosstrawpoll(account_name self)
        : contract(self),
          recent_polls(_self, _self),
          popular_polls(_self, _self),
          recent_votes(_self, _self)
    {
    }

    void create(
        const account_name creator,
        const string &title,
        const vector<string> &options,
        const vector<account_name> &whitelist,
        const vector<account_name> &blacklist,
        const uint8_t min_num_choices,
        const uint8_t max_num_choices,
        const timestamp open_time,
        const timestamp close_time);

    void close(
        const account_name creator,
        const uuid poll_id);

    void destroy(
        const account_name creator,
        const uuid poll_id);

    void vote(
        const account_name creator,
        const uuid poll_id,
        const account_name voter,
        const vector<uint8_t> &choices);

  private:
    asset calculate_holdings(const account_name account);
};