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
      _configs(self, self),
      _new_donations(self, self),
      _donors(self, self),
      _users(self, self),
      _popular_polls(self, self),
      _new_polls(self, self)
{
    _config = _configs.exists() ? _configs.get() : config();
}

void contract::ensure_user(const account_name account)
{
    auto u = _users.find(account);
    if (u == _users.end())
    {
        _users.emplace(_self, [&](auto &u) {
            u.account = account;
            u.first_seen = now();
        });
    }
}

void contract::prune_new_polls()
{
    auto created_index = _new_polls.get_index<N(created)>();
    auto num_left = _config.max_new_polls;
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
    auto num_left = _config.max_popular_polls;
    for (auto it = _popular_polls.begin(); it != _popular_polls.end();)
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
    auto popularity_index = _popular_polls.get_index<N(popularity)>();
    auto num_left = _config.max_popular_polls;
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