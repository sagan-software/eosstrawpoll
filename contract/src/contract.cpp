#include "contract.hpp"
#include "createpoll.cpp"
#include "createvote.cpp"

// #include "closepoll.cpp"
// #include "createvote.cpp"
// #include "destroypoll.cpp"
// #include "destroyvote.cpp"
// #include "destroyvotes.cpp"
// #include "openpoll.cpp"
// #include "setconfig.cpp"

namespace eosstrawpoll
{

contract::contract(eosio::name s, eosio::name code, eosio::datastream<const char *> ds)
    : eosio::contract(s, code, ds),
      _global(s, s.value),
      _polls(s, s.value),
      _votes(s, s.value),
      _popular(s, s.value),
      _latest(s, s.value)
{
    _gstate = _global.exists() ? _global.get() : global_state();
}

} // namespace eosstrawpoll
// void eosstrawpoll::prune_new_polls()
// {
//     auto created_index = new_polls_table.get_index<N(created)>();
//     auto num_left = global_config.max_new_polls;
//     for (auto it = created_index.rbegin(); it != created_index.rend();)
//     {
//         if (num_left <= 0)
//         {
//             it = decltype(it){created_index.erase(std::next(it).base())};
//         }
//         else
//         {
//             num_left -= 1;
//             ++it;
//         }
//     }
// }

// bool eosstrawpoll::is_popular_polls_full()
// {
//     auto num_left = global_config.max_popular_polls;
//     for (auto it = popular_polls_table.begin(); it != popular_polls_table.end();)
//     {
//         num_left -= 1;
//         if (num_left <= 0)
//         {
//             return true;
//         }
//         ++it;
//     }

//     return false;
// }

// void eosstrawpoll::prune_popular_polls()
// {
//     auto popularity_index = popular_polls_table.get_index<N(popularity)>();
//     auto num_left = global_config.max_popular_polls;
//     for (auto it = popularity_index.rbegin(); it != popularity_index.rend();)
//     {
//         if (num_left <= 0)
//         {
//             it = decltype(it){popularity_index.erase(std::next(it).base())};
//         }
//         else
//         {
//             num_left -= 1;
//             ++it;
//         }
//     }
// }

// uint32_t eosstrawpoll::get_num_votes(const poll_id_t poll_id)
// {
//     uint32_t num_votes = 0;
//     auto pollid_index = votes_table.get_index<N(pollid)>();
//     auto itr = pollid_index.lower_bound(poll_id);
//     for (; itr != pollid_index.end() && itr->poll_id == poll_id; ++itr)
//     {
//         num_votes++;
//     }
//     return num_votes;
// }

// double eosstrawpoll::calculate_popularity(
//     const uint32_t num_votes,
//     const time_t start_time)
// {
//     const double elapsed_seconds = now() - start_time;
//     const double elapsed_hours = elapsed_seconds / 60.0 / 60.0;
//     return num_votes / std::pow(elapsed_hours + 2, global_config.popularity_gravity);
// }
