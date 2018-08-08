#include <eosstrawpoll/contract.hpp>
#include <eosiolib/dispatcher.hpp>

#include "config.cpp"
#include "polls.cpp"
#include "votes.cpp"

EOSIO_ABI(
    eosstrawpoll::contract,
    // config.cpp
    (setconfig)
    // polls.cpp
    (createpoll)(closepoll)(destroypoll)
    // votes.cpp
    (createvote)(destroyvote))