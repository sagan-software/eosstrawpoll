#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::unbanuser(
    const account_name moderator,
    const account_name account,
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
    eosio_assert(_user->is_banned, "user is not banned");

    // update user ban
    _users.modify(_user, _self, [&](auto &u) {
        u.is_banned = false;
        u.ban_expiration = 0;
        u.ban_reason = "";
    });
};

} // namespace eosstrawpoll
