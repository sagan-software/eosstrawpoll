#include <eosstrawpoll/contract.hpp>
#include <eosstrawpoll/oldtypes.hpp>
#include <cmath>
#include <map>

namespace eosstrawpoll
{

void contract::destroyold(const string &table, const account_name scope)
{
    require_auth(_self);

    const bool is_popularpolls = table == "popularpolls";
    const bool is_newpolls = table == "newpolls";
    const bool is_polls = table == "polls";
    const bool is_users = table == "users";

    eosio_assert(
        is_popularpolls || is_newpolls || is_polls || is_users,
        "table must be popularpolls, newpolls, polls, or users");

    if (is_popularpolls)
    {
        old::popular_polls_table t(_self, _self);
        for (auto itr = t.begin(); itr != t.end();)
        {
            itr = t.erase(itr);
        }
    }
    else if (is_newpolls)
    {
        old::new_polls_table t(_self, _self);
        for (auto itr = t.begin(); itr != t.end();)
        {
            itr = t.erase(itr);
        }
    }
    else if (is_polls)
    {
        eosio_assert(is_account(scope), "provided scope is not an account");
        old::polls_table t(_self, scope);
        for (auto itr = t.begin(); itr != t.end();)
        {
            itr = t.erase(itr);
        }
    }
    else if (is_users)
    {
        old::users_table t(_self, _self);
        for (auto itr = t.begin(); itr != t.end();)
        {
            itr = t.erase(itr);
        }
    }
}

} // namespace eosstrawpoll