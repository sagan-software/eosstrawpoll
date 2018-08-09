#pragma once

#include <boost/algorithm/string/predicate.hpp>
#include <boost/test/unit_test.hpp>
#include <eosio/chain/abi_serializer.hpp>
#include <eosio/testing/tester.hpp>
#include <regex>

#include "contracts.hpp"

#include <Runtime/Runtime.h>

#include <fc/io/json.hpp>
#include <fc/variant_object.hpp>

using namespace eosio;
using namespace eosio::chain;
using namespace eosio::testing;
using namespace fc;
using namespace std;

using mvo = fc::mutable_variant_object;

typedef uint64_t uuid;
typedef uint32_t timestamp;

struct vote_t
{
    account_name voter;
    timestamp time;
    asset holdings;
    vector<uint8_t> choices;
};
FC_REFLECT(vote_t, (voter)(time)(holdings)(choices));

struct poll_t
{
    uuid id;
    account_name creator;
    string title;
    vector<string> options;
    vector<vote_t> votes;
    uint8_t min_num_choices;
    uint8_t max_num_choices;
    vector<account_name> whitelist;
    vector<account_name> blacklist;
    timestamp create_time;
    timestamp open_time;
    timestamp close_time;
};
FC_REFLECT(poll_t, (id)(creator)(title)(options)(votes)(min_num_choices)(max_num_choices)(whitelist)(blacklist)(create_time)(open_time)(close_time));

class eosstrawpoll_tester : public tester
{
  public:
    action_result push_action(
        const account_name &signer,
        const action_name &name,
        const variant_object &data)
    {
        string action_type_name = abi_ser.get_action_type(name);

        action act;
        act.account = N(eosstrawpoll);
        act.name = name;
        act.authorization = vector<permission_level>{{signer, config::active_name}};
        act.data = abi_ser.variant_to_binary(action_type_name, data, abi_serializer_max_time);

        signed_transaction trx;
        trx.actions.emplace_back(std::move(act));
        set_transaction_headers(trx);
        trx.sign(get_private_key(signer, "active"), control->get_chain_id());
        transaction_trace_ptr trace;
        try
        {
            trace = push_transaction(trx);

            produce_block();
            BOOST_REQUIRE_EQUAL(true, chain_has_transaction(trx.id()));
            return trace->action_traces.front().console;
        }
        catch (const fc::exception &ex)
        {
            //edump((ex.to_detail_string()));
            return error(ex.top_message()); // top_message() is assumed by many tests; otherwise they fail
                                            //return error(ex.to_detail_string());
        }
    }

    auto create_poll(
        const account_name creator,
        const string &title,
        const vector<string> &options,
        const vector<account_name> &whitelist,
        const vector<account_name> &blacklist,
        const uint8_t min_num_choices,
        const uint8_t max_num_choices,
        const timestamp open_time,
        const timestamp close_time)
    {
        return push_action(
            creator,
            N(create),
            mvo()("creator", creator)("title", title)("options", options)("whitelist", whitelist)("blacklist", blacklist)("min_num_choices", min_num_choices)("max_num_choices", max_num_choices)("open_time", open_time)("close_time", close_time));
    }

    auto close_poll(
        const account_name creator,
        const uuid poll_id)
    {
        return push_action(
            creator,
            N(close),
            mvo()("creator", creator)("poll_id", poll_id));
    }

    auto destroy_poll(
        const account_name creator,
        const uuid poll_id)
    {
        return push_action(
            creator,
            N(destroy),
            mvo()("creator", creator)("poll_id", poll_id));
    }

    auto cast_vote(
        const account_name creator,
        const uuid poll_id,
        const account_name voter,
        const vector<uint8_t> &choices)
    {
        return push_action(
            voter,
            N(vote),
            mvo()("creator", creator)("poll_id", poll_id)("voter", voter)("choices", choices));
    }

    uuid extract_uuid(string console)
    {
        const regex re("id=(\\d+)");
        smatch match;

        regex_search(console, match, re);

        BOOST_REQUIRE_EQUAL(match.size(), 2);

        const string id_str = match.str(1);
        const uuid id = std::stoull(id_str);
        return id;
    }

    poll_t get_poll(const account_name creator, const uuid poll_id)
    {
        poll_t p;
        get_table_entry(p, N(eosstrawpoll), creator, N(polls), poll_id);
        return p;
    }

    bool starts_with(const string &haystack, const string &needle)
    {
        return needle.length() <= haystack.length() && equal(needle.begin(), needle.end(), haystack.begin());
    }

    void require_success(action_result result)
    {
        BOOST_REQUIRE_MESSAGE(
            "success" == result.substr(0, 7),
            "action did not succeed: " << result);
    }

    uuid require_create_poll(
        const account_name creator,
        const string &title,
        const vector<string> &options,
        const vector<account_name> &whitelist,
        const vector<account_name> &blacklist,
        const uint8_t min_num_choices,
        const uint8_t max_num_choices,
        const timestamp open_time,
        const timestamp close_time)
    {
        const action_result result = create_poll(
            creator,
            title,
            options,
            whitelist,
            blacklist,
            min_num_choices,
            max_num_choices,
            open_time,
            close_time);
        require_success(result);
        return extract_uuid(result);
    }

    timestamp now()
    {
        return control->pending_block_time().time_since_epoch().count() / 1000000;
    }

    eosstrawpoll_tester()
    {
        create_accounts({N(alice), N(bob), N(carol), N(eosstrawpoll)});
        set_code(N(eosstrawpoll), eosstrawpoll::testing::contracts::wasm());
        set_abi(N(eosstrawpoll), eosstrawpoll::testing::contracts::abi().data());
        const auto &accnt = control->db().get<account_object, by_name>(N(eosstrawpoll));
        abi_def abi;
        BOOST_REQUIRE_EQUAL(abi_serializer::to_abi(accnt.abi, abi), true);
        abi_ser.set_abi(abi, abi_serializer_max_time);
        produce_block();
    }

    abi_serializer abi_ser;
};