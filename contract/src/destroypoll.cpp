#include <eosstrawpoll/contract.hpp>
#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::destroypoll(
    const account_name creator,
    const poll_name slug,
    const string &metadata)
{
    require_auth(creator);
    eosio_assert(metadata.size() <= _config.max_metadata_len, "metadata is too long");

    // check if poll exists
    polls_table polls(_self, creator);
    auto p = polls.find(slug);
    eosio_assert(p != polls.end(), "poll doesn't exist");

    // erase poll
    polls.erase(p);

    // erase poll from recent polls table
    for (auto it = _new_polls.begin(); it != _new_polls.end();)
    {
        if (it->creator == creator && it->slug == slug)
        {
            it = _new_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }

    // erase poll from popular polls table
    for (auto it = _popular_polls.begin(); it != _popular_polls.end();)
    {
        if (it->creator == creator && it->slug == slug)
        {
            it = _popular_polls.erase(it);
        }
        else
        {
            ++it;
        }
    }
}

} // namespace eosstrawpoll