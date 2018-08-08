#pragma once

#include <eosstrawpoll/types.hpp>
#include <eosiolib/asset.hpp>
#include <eosiolib/multi_index.hpp>

namespace eosstrawpoll
{

// @abi table donations
struct donation
{
    uint64_t id;
    account_name name;
    eosio::asset quantity;
    string memo;
    uint64_t time;

    uint64_t primary_key() const { return id; }
    uint64_t by_time() const { return time; }

    EOSLIB_SERIALIZE(donation, (id)(name)(quantity)(memo)(time))
};

typedef multi_index<
    N(donations), donation,
    indexed_by<N(time), const_mem_fun<donation, uint64_t, &donation::by_time>>>
    donations_index;

// @abi table topdonors
struct donor
{
    account_name name;
    uint64_t donated;

    account_name primary_key() const { return name; }
    uint64_t by_donated() const { return donated; }

    EOSLIB_SERIALIZE(donor, (name)(donated))
};

typedef multi_index<
    N(topdonors), donor,
    indexed_by<N(donated), const_mem_fun<donor, uint64_t, &donor::by_donated>>>
    top_donors_index;

} // namespace eosstrawpoll