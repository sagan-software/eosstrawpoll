#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::closepoll(
    const account_name creator,
    const poll_name slug)
{
    require_auth(creator);

    // find poll
    polls_table _creator_polls(_self, creator);
    auto _poll = _creator_polls.find(slug);

    // check if poll exists
    eosio_assert(_poll != _creator_polls.end(), "poll does not exist");

    // check if poll is already closed
    eosio_assert(!_poll->is_closed(), "poll is already closed");

    // close poll
    _creator_polls.modify(_poll, creator, [&](auto &p) {
        p.close_time = now();

        // set open_time to close_time/now if necessary
        if (p.open_time > p.close_time)
        {
            p.open_time = p.close_time;
        }
    });
};

} // namespace eosstrawpoll
