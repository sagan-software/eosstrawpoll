#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvotes(
    const account_name creator,
    const poll_name slug,
    const string &metadata)
{
    require_auth(creator);

    assert_metadata_len(metadata);

    // find poll
    polls_table _creator_polls(_self, creator);
    auto _poll = _creator_polls.find(slug);

    // check if poll exists
    eosio_assert(_poll != _creator_polls.end(), "poll does not exist");

    // check if there are any votes to clear
    eosio_assert(!_poll->votes.empty(), "there are no votes to clear");

    // close poll
    _creator_polls.modify(_poll, creator, [&](auto &p) {
        p.votes = {};
    });
};

} // namespace eosstrawpoll
