#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::clearprofile(const account_name account)
{
    require_auth(account);

    // check if profile exists
    profile_table _profile(_self, account);
    eosio_assert(_profile.exists(), "no profile to clear");

    // destroy profile
    _profile.remove();
};

} // namespace eosstrawpoll
