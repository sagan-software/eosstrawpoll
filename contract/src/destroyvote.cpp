#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvote(
    const account_name creator,
    const poll_name slug,
    const account_name voter,
    const string &metadata)
{
    require_auth(voter);
    assert_metadata_len(metadata);

    // check if poll exists
    polls_table polls(_self, creator);
    auto p = polls.find(slug);
    eosio_assert(p != polls.end(), "poll doesn't exist");

    // check if voter has voted on this poll

    // destroy vote
};

} // namespace eosstrawpoll
