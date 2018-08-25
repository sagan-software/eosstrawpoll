#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::transfer(
    const eosio::currency::transfer &t,
    const account_name code)
{
    if (t.to != _self || code != N(eosio.token))
    {
        return;
    }

    // only track CORE_SYMBOL transfers
    if (t.quantity.symbol != CORE_SYMBOL)
    {
        return;
    }

    const account_name account = t.from;
    const uint64_t donated = t.quantity.amount;

    // add new donation
    _new_donations.emplace(_self, [&](auto &d) {
        d.id = _new_donations.available_primary_key();
        d.account = account;
        d.donated = donated;
        d.memo = t.memo;
        d.created = now();
    });

    prune_new_donations();

    // update donors table
    auto d = _donors.find(account);
    if (d == _donors.end())
    {
        _donors.emplace(_self, [&](auto &d) {
            d.account = account;
            d.donated = donated;
            d.last_donation = donation{
                .id = 0,
                .account = account,
                .donated = donated,
                .memo = t.memo,
                .created = now()};
        });
    }
    else
    {
        _donors.modify(d, _self, [&](auto &d) {
            d.donated += donated;
        });
    }
};

void contract::prune_new_donations()
{
    auto created_index = _new_donations.get_index<N(created)>();
    auto num_left = _config.max_new_donations;
    for (auto it = created_index.rbegin(); it != created_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){created_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

} // namespace eosstrawpoll
