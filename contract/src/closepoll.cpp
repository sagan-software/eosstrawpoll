#include "contract.hpp"

namespace eosstrawpoll
{

void contract::closepoll(const eosio::name poll_id)
{
    auto poll = polls_table.find(poll_id.value);

    eosio::check(poll != polls_table.end(), "poll does not exist");

    require_auth(poll->account);

    eosio::check(!poll->is_closed(), "poll is already closed");

    const auto close_time = current_time_point_sec();
    auto open_time = poll->open_time;
    if (open_time > close_time)
    {
        open_time = close_time;
    }

    polls_table.modify(poll, same_payer, [&](auto &p) {
        p.close_time = close_time;
        p.open_time = open_time;
    });

    auto popular_poll = popular_table.find(poll_id.value);
    if (popular_poll != popular_table.end())
    {
        popular_table.modify(popular_poll, same_payer, [&](auto &p) {
            p.close_time = close_time;
            p.open_time = open_time;
        });
    }

    auto new_poll = latest_table.find(poll_id.value);
    if (new_poll != latest_table.end())
    {
        latest_table.modify(new_poll, same_payer, [&](auto &p) {
            p.close_time = close_time;
            p.open_time = open_time;
        });
    }
};

} // namespace eosstrawpoll
