#include <eosiolib/dispatcher.hpp>
#include <eosstrawpoll/contract.hpp>

#include "createpoll.cpp"
#include "createvote.cpp"
#include "destroypoll.cpp"
#include "destroyvote.cpp"
#include "setconfig.cpp"
#include "transfer.cpp"

namespace eosstrawpoll
{

contract::contract(account_name self)
    : _self(self),
      _configs(self, self),
      _new_donations(self, self),
      _donors(self, self),
      _popular_polls(self, self),
      _new_polls(self, self)
{
    _config = _configs.exists() ? _configs.get() : config();
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
            (setconfig)(createpoll)(destroypoll)(createvote)(destroyvote))
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