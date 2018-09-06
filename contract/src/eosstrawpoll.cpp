#include <eosiolib/dispatcher.hpp>
#include <eosstrawpoll/contract.hpp>

#include "clearprofile.cpp"
#include "closepoll.cpp"
#include "createpoll.cpp"
#include "createvote.cpp"
#include "destroypoll.cpp"
#include "destroyvote.cpp"
#include "destroyvotes.cpp"
#include "openpoll.cpp"
#include "setconfig.cpp"
#include "setprofile.cpp"
#include "transfer.cpp"

namespace eosstrawpoll
{

contract::contract(account_name self)
    : _self(self),
      global_config_table(self, self),
      polls_table(self, self),
      votes_table(self, self),
      popular_polls_table(self, self),
      new_polls_table(self, self),
      new_donations_table(self, self),
      donors_table(self, self),
      profiles_table(self, self)
{
    global_config =
        global_config_table.exists()
            ? global_config_table.get()
            : global_config_t();
}

void contract::prune_new_polls()
{
    auto created_index = new_polls_table.get_index<N(created)>();
    auto num_left = global_config.max_new_polls;
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

bool contract::is_popular_polls_full()
{
    auto num_left = global_config.max_popular_polls;
    for (auto it = popular_polls_table.begin(); it != popular_polls_table.end();)
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

void contract::prune_popular_polls()
{
    auto popularity_index = popular_polls_table.get_index<N(popularity)>();
    auto num_left = global_config.max_popular_polls;
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

uint32_t contract::get_num_votes(const poll_id_t poll_id)
{
    uint32_t num_votes = 0;
    auto pollid_index = votes_table.get_index<N(pollid)>();
    auto itr = pollid_index.lower_bound(poll_id);
    for (; itr != pollid_index.end() && itr->poll_id == poll_id; ++itr)
    {
        num_votes++;
    }
    return num_votes;
}

double contract::calculate_popularity(
    const uint32_t num_votes,
    const time_t start_time)
{
    const double elapsed_seconds = now() - start_time;
    const double elapsed_hours = elapsed_seconds / 60.0 / 60.0;
    return num_votes / std::pow(elapsed_hours + 2, global_config.popularity_gravity);
}

void contract::apply(
    const account_name code,
    const account_name action)
{
    if (action == N(transfer))
    {
        transfer(eosio::unpack_action_data<eosio::currency::transfer>(), code);
        return;
    }

    if (code != _self)
    {
        return;
    }

    auto &thiscontract = *this;
    switch (action)
    {
        EOSIO_API(
            contract,
            (clearprofile)(closepoll)(createpoll)(createvote)(destroypoll)(destroyvote)(destroyvotes)(openpoll)(setconfig)(setprofile))
    };
}

} // namespace eosstrawpoll

extern "C"
{
    [[noreturn]] void apply(
        uint64_t receiver,
        uint64_t code,
        uint64_t action) {
        eosstrawpoll::contract c(receiver);
        c.apply(code, action);
        eosio_exit(0);
    }
}