#include "contract.hpp"

namespace eosstrawpoll
{

void contract::destroyvote(
    const eosio::name poll_id,
    const eosio::name account)
{
    require_auth(account);

    auto poll = polls_table.find(poll_id.value);
    eosio::check(poll != polls_table.end(), "poll doesn't exist");

    auto pollid_index = votes_table.get_index<"pollid"_n>();
    auto itr = pollid_index.lower_bound(poll_id.value);
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
