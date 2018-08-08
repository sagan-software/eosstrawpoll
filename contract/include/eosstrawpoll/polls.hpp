#pragma once

#include <eosstrawpoll/votes.hpp>

namespace eosstrawpoll
{

// @abi table polls i64
// @abi table popularpolls i64
// @abi table recentpolls i64
struct poll
{
    uint64_t id;
    poll_name name;
    account_name creator;
    std::string title;
    std::vector<std::string> options;
    std::vector<vote> votes;
    double popularity;
    uint16_t min_num_choices;
    uint16_t max_num_choices;
    std::vector<account_name> whitelist;
    std::vector<account_name> blacklist;
    timestamp create_time;
    timestamp open_time;
    timestamp close_time;
    bool allow_other;

    // Indexing functions
    uint64_t primary_key() const { return id; }
    uint64_t by_created() const { return create_time; }
    uint64_t by_closed() const { return close_time; }
    double by_popularity() const { return popularity; }

    // Helper functions
    bool has_opened() const;
    bool is_closed() const;
    double calculate_popularity(double popularity_gravity) const;

    EOSLIB_SERIALIZE(poll, (id)(name)(creator)(title)(options)(votes)(min_num_choices)(max_num_choices)(whitelist)(blacklist)(create_time)(open_time)(close_time)(allow_other))
};

typedef multi_index<N(polls), poll> polls_index;
typedef multi_index<
    N(popularpolls), poll,
    indexed_by<N(popularity), const_mem_fun<poll, double, &poll::by_popularity>>>
    popular_polls_index;
typedef multi_index<
    N(newpolls), poll,
    indexed_by<N(created), const_mem_fun<poll, uint64_t, &poll::by_created>>>
    new_polls_index;
typedef multi_index<
    N(closedpolls), poll,
    indexed_by<N(closed), const_mem_fun<poll, uint64_t, &poll::by_closed>>>
    closed_polls_index;

} // namespace eosstrawpoll