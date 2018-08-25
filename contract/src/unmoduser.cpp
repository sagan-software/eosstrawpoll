#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::unmoduser(
    const account_name account,
    const string &metadata)
{
    require_auth(_self);

    assert_metadata_len(metadata);

    eosio_assert(is_account(account), "account does not exist");

    // find user
    auto _user = _users.find(account);

    // check if user exists
    eosio_assert(_user != _users.end(), "user does not exist");
    eosio_assert(user->is_modded, "user is not modded");

    // update user ban
    _users.modify(_user, _self, [&](auto &u) {
        u.is_modded = false;
        u.mod_expiration = 0;
    });
};

} // namespace eosstrawpoll
