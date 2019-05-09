#include "contract.hpp"
#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::destroypoll(const eosio::name poll_id)
{
    auto poll = polls_table.find(poll_id.value);
    eosio::check(poll != polls_table.end(), "poll doesn't exist");

    require_auth(poll->account);

    polls_table.erase(poll);

    auto popular_poll = popular_table.find(poll_id.value);
    if (popular_poll != popular_table.end())
    {
        popular_table.erase(popular_poll);
    }

    auto new_poll = latest_table.find(poll_id.value);
    if (new_poll != latest_table.end())
    {
        latest_table.erase(new_poll);
    }
}

} // namespace eosstrawpoll