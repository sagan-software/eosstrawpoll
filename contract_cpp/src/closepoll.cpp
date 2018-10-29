#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::closepoll(const poll_id_t poll_id)
{
    auto poll = polls_table.find(poll_id);

    eosio_assert(poll != polls_table.end(), "poll does not exist");

    require_auth(poll->account);

    eosio_assert(!poll->is_closed(), "poll is already closed");

    const auto close_time = now();
    auto open_time = poll->open_time;
    if (open_time > close_time)
    {
        open_time = close_time;
    }

    polls_table.modify(poll, poll->account, [&](auto &p) {
        p.close_time = close_time;
        p.open_time = open_time;
    });

    auto popular_poll = popular_polls_table.find(poll_id);
    if (popular_poll != popular_polls_table.end())
    {
        popular_polls_table.modify(popular_poll, _self, [&](auto &p) {
            p.close_time = close_time;
            p.open_time = open_time;
        });
    }

    auto new_poll = new_polls_table.find(poll_id);
    if (new_poll != new_polls_table.end())
    {
        new_polls_table.modify(new_poll, _self, [&](auto &p) {
            p.close_time = close_time;
            p.open_time = open_time;
        });
    }
};

} // namespace eosstrawpoll
