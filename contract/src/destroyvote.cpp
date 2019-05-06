#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvote(
    const poll_id_t poll_id,
    const eosio::name account)
{
    require_auth(account);

    auto poll = polls_table.find(poll_id);
    eosio::check(poll != polls_table.end(), "poll doesn't exist");

    auto pollid_index = votes_table.get_index<N(pollid)>();
    auto itr = pollid_index.lower_bound(poll_id);
    bool found_vote = false;
    for (; itr != pollid_index.end() && itr->poll_id == poll_id;)
    {
        if (itr->account == account)
        {
            // itr = decltype(itr){pollid_index.erase(std::next(itr).base())};
            found_vote = true;
        }
        else
        {
            ++itr;
        }
    }

    eosio::check(found_vote, "no vote found for this poll_id and account combination");

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
