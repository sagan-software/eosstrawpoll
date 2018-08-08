#pragma once

#include <eosstrawpoll/types.hpp>
#include <eosiolib/serialize.hpp>

namespace eosstrawpoll
{

struct vote
{
    account_name voter;
    timestamp time;
    uint64_t staked;
    vector<uint16_t> choices;
    string other;

    EOSLIB_SERIALIZE(vote, (voter)(time)(staked)(choices)(other))
};

}