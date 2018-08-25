#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::moduser(
    const account_name account,
    const time expiration,
    const string &metadata)
{
    require_auth(_self);

    assert_metadata_len(metadata);

    eosio_assert(is_account(account), "account does not exist");

    // find user
    auto _user = _users.find(account);

    // check if user exists
    eosio_assert(_user != _users.end(), "user does not exist");

    // update user ban
    _users.modify(_user, _self, [&](auto &u) {
        u.is_banned = false;
        u.ban_expiration = 0;
        u.ban_reason = "";
        u.is_modded = true;
        u.mod_expiration = expiration;
    });
};

} // namespace eosstrawpoll
