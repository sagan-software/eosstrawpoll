#include <eosiolib/eosio.hpp>
#include <eosiolib/asset.hpp>
using namespace eosio;

class hello : public eosio::contract
{
public:
  using contract::contract;

  /// @abi action
  void hi(account_name user, eosio::asset test)
  {
    print("Hello, ", name{user}, ". ", test.symbol.value, " ! ", test.symbol.precision(), " ! ", test.symbol.name());
  }
};

EOSIO_ABI(hello, (hi))
