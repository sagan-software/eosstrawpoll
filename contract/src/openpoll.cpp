#include "contract.hpp"

namespace eosstrawpoll
{

void contract::openpoll(const eosio::name poll_id)
{
    auto poll = polls_table.find(poll_id.value);
    eosio::check(poll != polls_table.end(), "poll does not exist");

    require_auth(poll->account);

    eosio::check(!poll->is_open(), "poll is already open");

    polls_table.modify(poll, same_payer, [&](auto &p) {
        p.open_time = current_time_point_sec();

        if (p.open_time > p.close_time)
        {
            p.close_time = time_point_sec(0);
        }
    });

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
