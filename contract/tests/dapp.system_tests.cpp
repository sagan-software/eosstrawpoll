#include <boost/test/unit_test.hpp>
#include <eosio/testing/tester.hpp>
#include <eosio/chain/abi_serializer.hpp>
#include "eosio.system_tester.hpp"

#include "Runtime/Runtime.h"

#include <fc/variant_object.hpp>

using namespace eosio::testing;
using namespace eosio;
using namespace eosio::chain;
using namespace eosio::testing;
using namespace fc;
using namespace std;

using mvo = fc::mutable_variant_object;

class dapp_system_tester : public tester
{
 public:
   dapp_system_tester()
   {
      produce_blocks(2);

      create_accounts({N(alice), N(bob), N(carol), N(dapp.system)});
      produce_blocks(2);

      set_code(N(dapp.system), dapp::testing::contracts::system_wasm());
      set_abi(N(dapp.system), dapp::testing::contracts::system_abi().data());

      produce_blocks();

      const auto &accnt = control->db().get<account_object, by_name>(N(dapp.system));
      abi_def abi;
      BOOST_REQUIRE_EQUAL(abi_serializer::to_abi(accnt.abi, abi), true);
      abi_ser.set_abi(abi, abi_serializer_max_time);
   }

   action_result push_action(const account_name &signer, const action_name &name, const variant_object &data)
   {
      string action_type_name = abi_ser.get_action_type(name);

      action act;
      act.account = N(dapp.system);
      act.name = name;
      act.data = abi_ser.variant_to_binary(action_type_name, data, abi_serializer_max_time);

      return base_tester::push_action(std::move(act), uint64_t(signer));
   }

   fc::variant get_stats(const string &symbolname)
   {
      auto symb = eosio::chain::symbol::from_string(symbolname);
      auto symbol_code = symb.to_symbol_code().value;
      vector<char> data = get_row_by_account(N(dapp.system), symbol_code, N(stat), symbol_code);
      return data.empty() ? fc::variant() : abi_ser.binary_to_variant("currency_stats", data, abi_serializer_max_time);
   }

   fc::variant get_account(account_name acc, const string &symbolname)
   {
      auto symb = eosio::chain::symbol::from_string(symbolname);
      auto symbol_code = symb.to_symbol_code().value;
      vector<char> data = get_row_by_account(N(dapp.system), acc, N(accounts), symbol_code);
      return data.empty() ? fc::variant() : abi_ser.binary_to_variant("account", data, abi_serializer_max_time);
   }

   action_result create(account_name issuer,
                        asset maximum_supply)
   {

      return push_action(N(dapp.system), N(create), mvo()("issuer", issuer)("maximum_supply", maximum_supply));
   }

   action_result issue(account_name issuer, account_name to, asset quantity, string memo)
   {
      return push_action(issuer, N(issue), mvo()("to", to)("quantity", quantity)("memo", memo));
   }

   action_result retire(account_name issuer, asset quantity, string memo)
   {
      return push_action(issuer, N(retire), mvo()("quantity", quantity)("memo", memo));
   }

   action_result transfer(account_name from,
                          account_name to,
                          asset quantity,
                          string memo)
   {
      return push_action(from, N(transfer), mvo()("from", from)("to", to)("quantity", quantity)("memo", memo));
   }

   action_result open(account_name owner,
                      const string &symbolname,
                      account_name ram_payer)
   {
      return push_action(ram_payer, N(open), mvo()("owner", owner)("symbol", symbolname)("ram_payer", ram_payer));
   }

   action_result close(account_name owner,
                       const string &symbolname)
   {
      return push_action(owner, N(close), mvo()("owner", owner)("symbol", "0,CERO"));
   }

   abi_serializer abi_ser;
};

BOOST_AUTO_TEST_SUITE(dapp_system_tests)

BOOST_FIXTURE_TEST_CASE(newproject, dapp_system_tester)
try
{
   // can create a new project

   // project is saved properly

   // can't create a project with a name that is already taken

   // can't create a project without creator authority

   BOOST_REQUIRE_EQUAL(true, true);
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(savereward, dapp_system_tester)
try
{
   // can save a reward

   // reward is saved properly

   // can't save reward for project that doesn't exist

   // can't save reward for project that isn't ours

   // can't save reward with invalid reward index

   // can update rewards with valid reward indexes

   BOOST_REQUIRE_EQUAL(true, true);
}
FC_LOG_AND_RETHROW()

BOOST_AUTO_TEST_SUITE_END()
