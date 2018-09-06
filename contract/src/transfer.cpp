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
    new_donations_table.emplace(_self, [&](auto &d) {
        d.id = new_donations_table.available_primary_key();
        d.account = account;
        d.donated = donated;
        d.memo = t.memo;
        d.create_time = now();
    });

    prune_new_donations();

    const auto donation = donation_t{
        .id = 0,
        .account = account,
        .donated = donated,
        .memo = t.memo,
        .create_time = now()};

    // update donors table
    auto donor = donors_table.find(account);
    if (donor == donors_table.end())
    {
        donors_table.emplace(_self, [&](auto &d) {
            d.account = account;
            d.donated = donated;
            d.first_donation = donation;
            d.last_donation = donation;
        });
    }
    else
    {
        donors_table.modify(donor, _self, [&](auto &d) {
            d.donated += donated;
            d.last_donation = donation;
        });
    }
};

void contract::prune_new_donations()
{
    auto num_left = global_config.max_new_donations;
    for (auto it = new_donations_table.rbegin(); it != new_donations_table.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){new_donations_table.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

} // namespace eosstrawpoll
