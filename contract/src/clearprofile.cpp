#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::clearprofile(
    const account_name account,
    const string &metadata)
{
    require_auth(account);

    assert_metadata_len(metadata);

    // check if profile exists
    userprofile_table profile(_self, account);
    eosio_assert(profile.exists(), "no profile to clear");

    // destroy profile
    profile.remove();
};

} // namespace eosstrawpoll
