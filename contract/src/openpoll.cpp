#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::openpoll(const poll_id_t poll_id)
{
    auto poll = polls_table.find(poll_id);
    eosio::check(poll != polls_table.end(), "poll does not exist");

    require_auth(poll->account);

    eosio::check(!poll->is_open(), "poll is already open");

    polls_table.modify(poll, poll->account, [&](auto &p) {
        p.open_time = now();

        if (p.open_time > p.close_time)
        {
            p.close_time = 0;
        }
    });

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
