#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::banuser(
    const account_name moderator,
    const account_name account,
    const time expiration,
    const string &reason,
    const string &metadata)
{
    require_auth(moderator);
    assert_metadata_len(metadata);
    assert_is_moderator(moderator);

    eosio_assert(is_account(account), "account does not exist");

    // find user
    auto _user = _users.find(account);

    // check if user exists
    eosio_assert(_user != _users.end(), "user does not exist");

    // update user ban
    _users.modify(_user, _self, [&](auto &u) {
        u.is_banned = true;
        u.ban_expiration = expiration;
        u.ban_reason = reason;
        u.is_modded = false;
        u.mod_expiration = 0;
    });
};

} // namespace eosstrawpoll
