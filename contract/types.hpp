#pragma once

#include <eosiolib/multi_index.hpp>
#include <eosiolib/types.hpp>

namespace eosstrawpoll
{
using std::string;
using std::vector;
using eosio::const_mem_fun;
using eosio::indexed_by;
using eosio::multi_index;
typedef account_name poll_name;
typedef uint32_t timestamp;
} // namespace eosstrawpoll