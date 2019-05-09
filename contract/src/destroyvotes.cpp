#include "contract.hpp"

namespace eosstrawpoll
{

void contract::destroyvotes(const eosio::name poll_id)
{
    auto poll = polls_table.find(poll_id.value);
    eosio::check(poll != polls_table.end(), "poll does not exist");
    require_auth(poll->account);

    auto pollid_index = votes_table.get_index<"pollid"_n>();
    auto itr = pollid_index.lower_bound(poll_id.value);
    for (; itr != pollid_index.end() && itr->poll_id == poll_id;)
    {
        itr = pollid_index.erase(itr);
    }

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
