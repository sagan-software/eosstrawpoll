#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::clearprofile(const account_name account)
{
    require_auth(account);

    auto profile = profiles_table.find(account);
    eosio_assert(
        profile != profiles_table.end(),
        "poll does not exist");

    profiles_table.erase(profile);
};

} // namespace eosstrawpoll
