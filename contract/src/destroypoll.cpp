#include <eosstrawpoll/contract.hpp>
#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::destroypoll(const poll_id_t poll_id)
{
    auto poll = polls_table.find(poll_id);
    eosio::check(poll != polls_table.end(), "poll doesn't exist");

    require_auth(poll->account);

    polls_table.erase(poll);

    auto popular_poll = popular_polls_table.find(poll_id);
    if (popular_poll != popular_polls_table.end())
    {
        popular_polls_table.erase(popular_poll);
    }

    auto new_poll = new_polls_table.find(poll_id);
    if (new_poll != new_polls_table.end())
    {
        new_polls_table.erase(new_poll);
    }
}

} // namespace eosstrawpoll