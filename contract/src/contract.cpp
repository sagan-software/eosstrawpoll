#include "closepoll.cpp"
#include "contract.hpp"
#include "createpoll.cpp"
#include "createvote.cpp"
#include "destroypoll.cpp"
#include "destroyvote.cpp"
#include "destroyvotes.cpp"
#include "openpoll.cpp"
#include "setconfig.cpp"

namespace eosstrawpoll
{

contract::contract(eosio::name s, eosio::name code, eosio::datastream<const char *> ds)
    : eosio::contract(s, code, ds),
      global_config_singleton(s, s.value),
      polls_table(s, s.value),
      votes_table(s, s.value),
      popular_table(s, s.value),
      latest_table(s, s.value)
{
    global_config = global_config_singleton.exists() ? global_config_singleton.get() : global_config_t();
}

time_point current_time_point()
{
    const static eosio::time_point ct{eosio::microseconds{static_cast<int64_t>(current_time())}};
    return ct;
}

time_point_sec current_time_point_sec()
{
    const static eosio::time_point_sec cts{current_time_point()};
    return cts;
}

void contract::prune_latest()
{
    auto created_index = latest_table.get_index<"created"_n>();
    auto num_left = global_config.max_latest;
    for (auto it = created_index.rbegin(); it != created_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){created_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

void contract::prune_popular()
{
    auto popularity_index = popular_table.get_index<"popularity"_n>();
    auto num_left = global_config.max_popular;
    for (auto it = popularity_index.rbegin(); it != popularity_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){popularity_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

bool contract::is_popular_full()
{
    auto num_left = global_config.max_popular;
    for (auto it = popular_table.begin(); it != popular_table.end();)
    {
        num_left -= 1;
        if (num_left <= 0)
        {
            return true;
        }
        ++it;
    }

    return false;
}

uint32_t contract::get_num_votes(const eosio::name poll_id)
{
    uint32_t num_votes = 0;
    auto idx = votes_table.get_index<"pollid"_n>();
    auto itr = idx.lower_bound(poll_id.value);
    for (; itr != idx.end() && itr->poll_id == poll_id; ++itr)
    {
        num_votes++;
    }
    return num_votes;
}

double contract::calculate_popularity(
    const uint32_t num_votes,
    const eosio::time_point_sec start_time)
{
    const auto elapsed = current_time_point_sec() - start_time;
    const auto elapsed_hours = elapsed.to_seconds() / 60 / 60;
    return num_votes / std::pow(elapsed_hours + 2, global_config.popularity_gravity);
}

} // namespace eosstrawpoll

EOSIO_DISPATCH(eosstrawpoll::contract, (closepoll)(createpoll)(createvote)(destroypoll)(destroyvote)(destroyvotes)(openpoll)(setconfig))