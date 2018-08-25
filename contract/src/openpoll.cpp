#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::openpoll(
    const account_name creator,
    const poll_name slug,
    const string &metadata)
{
    require_auth(voter);

    // banned users cannot open polls
    assert_not_banned(voter);

    assert_metadata_len(metadata);

    // find poll
    polls_table _creator_polls(_self, creator);
    auto _poll = _creator_polls.find(slug);

    // check if poll exists
    eosio_assert(_poll != _creator_polls.end(), "poll does not exist");

    // check if poll is already open
    eosio_assert(!_poll->is_open(), "poll is already open");

    // open poll
    _creator_polls.modify(_poll, creator, [&](auto &p) {
        p.open_time = now();

        // set close_time to 0 if necessary
        if (p.open_time > p.close_time)
        {
            p.close_time = 0;
        }
    });
};

} // namespace eosstrawpoll
