#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroytable(const string &table)
{
    require_auth(_self);

    const bool is_popularpolls = table == "popularpolls";
    const bool is_newpolls = table == "newpolls";

    eosio_assert(
        is_popularpolls || is_newpolls,
        "table must be popularpolls or newpolls");

    if (is_popularpolls)
    {
        for (auto itr = popular_polls_table.begin(); itr != popular_polls_table.end();)
        {
            itr = popular_polls_table.erase(itr);
        }
    }
    else if (is_newpolls)
    {
        for (auto itr = new_polls_table.begin(); itr != new_polls_table.end();)
        {
            itr = new_polls_table.erase(itr);
        }
    }

    // TODO: update popular polls, new polls
};

} // namespace eosstrawpoll
