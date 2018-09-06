#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvotes(const poll_id_t poll_id)
{
    auto poll = polls_table.find(poll_id);
    eosio_assert(poll != polls_table.end(), "poll does not exist");
    require_auth(poll->account);

    auto pollid_index = votes_table.get_index<N(pollid)>();
    auto itr = pollid_index.lower_bound(poll_id);
    for (; itr != pollid_index.end() && itr->poll_id == poll_id;)
    {
        // itr = decltype(itr){pollid_index.erase(std::next(itr).base())};
    }

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
