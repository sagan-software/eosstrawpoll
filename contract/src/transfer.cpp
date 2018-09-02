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

    const auto SYS = S(4, SYS);
    const auto EOS = S(4, EOS);
    const auto TLOS = S(4, TLOS);

    const auto donated_symbol = t.quantity.symbol;

    // only track CORE_SYMBOL transfers
    if (donated_symbol != SYS && donated_symbol != EOS && donated_symbol != TLOS)
    {
        return;
    }

    // only track CORE_SYMBOL transfers
    // if (t.quantity.symbol != CORE_SYMBOL)
    // {
    //     return;
    // }

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

    const auto dono = donation{
        .id = 0,
        .account = account,
        .donated = donated,
        .memo = t.memo,
        .created = now()};

    // update donors table
    auto d = _donors.find(account);
    if (d == _donors.end())
    {
        _donors.emplace(_self, [&](auto &d) {
            d.account = account;
            d.donated = donated;
            d.first_donation = dono;
            d.last_donation = dono;
        });
    }
    else
    {
        _donors.modify(d, _self, [&](auto &d) {
            d.donated += donated;
            d.last_donation = dono;
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
