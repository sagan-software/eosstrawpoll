#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvotes(
    const account_name creator,
    const poll_name slug)
{
    require_auth(creator);

    // find poll
    polls_table _creator_polls(_self, creator);
    auto _poll = _creator_polls.find(slug);

    // check if poll exists
    eosio_assert(_poll != _creator_polls.end(), "poll does not exist");

    // check if there are any votes to clear
    eosio_assert(!_poll->votes.empty(), "there are no votes to clear");

    // clear votes
    _creator_polls.modify(_poll, creator, [&](auto &p) {
        p.votes = {};
    });
};

} // namespace eosstrawpoll
