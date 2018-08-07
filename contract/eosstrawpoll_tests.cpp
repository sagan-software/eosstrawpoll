#include <boost/algorithm/string/predicate.hpp>
#include <boost/test/unit_test.hpp>
#include <eosio/chain/abi_serializer.hpp>
#include <eosio/testing/tester.hpp>
#include <regex>

#include <eosstrawpoll/eosstrawpoll.abi.hpp>
#include <eosstrawpoll/eosstrawpoll.wast.hpp>

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
        set_code(N(eosstrawpoll), eosstrawpoll_wast);
        set_abi(N(eosstrawpoll), eosstrawpoll_abi);
        const auto &accnt = control->db().get<account_object, by_name>(N(eosstrawpoll));
        abi_def abi;
        BOOST_REQUIRE_EQUAL(abi_serializer::to_abi(accnt.abi, abi), true);
        abi_ser.set_abi(abi, abi_serializer_max_time);
        produce_block();
    }

    abi_serializer abi_ser;
};

BOOST_AUTO_TEST_SUITE(eosstrawpoll_tests)

BOOST_FIXTURE_TEST_CASE(can_create_polls, eosstrawpoll_tester)
try
{
    const account_name creator = N(alice);
    const string title = "Test poll";
    const vector<string> options{"Option A", "Option B", "Option C"};
    const vector<account_name> blacklist{N(name1), N(name2)};
    const vector<account_name> whitelist{N(name3), N(name4)};
    const uint8_t min_num_choices = 1;
    const uint8_t max_num_choices = 3;
    const timestamp open_time = 0;
    const timestamp close_time = 0;

    const uuid id = require_create_poll(
        creator,
        title,
        options,
        whitelist,
        blacklist,
        min_num_choices,
        max_num_choices,
        open_time,
        close_time);

    const poll_t p = get_poll(creator, id);
    BOOST_REQUIRE_EQUAL(p.id, id);
    BOOST_REQUIRE_EQUAL(p.title, title);
    BOOST_REQUIRE_MESSAGE(p.options == options, "options are different");
    BOOST_REQUIRE_MESSAGE(p.blacklist == blacklist, "blacklist is different");
    BOOST_REQUIRE_MESSAGE(p.whitelist == whitelist, "whitelist is different");
    BOOST_REQUIRE_EQUAL(p.min_num_choices, min_num_choices);
    BOOST_REQUIRE_EQUAL(p.max_num_choices, max_num_choices);
    BOOST_REQUIRE_EQUAL(p.open_time, now());
    BOOST_REQUIRE_EQUAL(p.close_time, close_time);
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_destroy_polls, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B", "Option C"},
        vector<account_name>{N(name1), N(name2)},
        vector<account_name>{N(name3), N(name4)},
        1,
        1,
        0,
        0);

    require_success(destroy_poll(N(alice), id));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_close_polls, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B", "Option C"},
        vector<account_name>{N(name1), N(name2)},
        vector<account_name>{N(name3), N(name4)},
        1,
        1,
        0,
        0);

    require_success(close_poll(N(alice), id));

    const poll_t p = get_poll(N(alice), id);
    BOOST_REQUIRE_MESSAGE(p.close_time > 0, "close_time should be greater than 0");
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_cast_votes, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B", "Option C"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        1,
        0,
        0);

    poll_t p;
    vote_t v;

    p = get_poll(N(alice), id);
    BOOST_REQUIRE_EQUAL(p.votes.size(), 0);

    require_success(cast_vote(N(alice), id, N(bob), vector<uint8_t>{1}));
    p = get_poll(N(alice), id);
    BOOST_REQUIRE_EQUAL(p.votes.size(), 1);
    v = p.votes[0];
    BOOST_REQUIRE_EQUAL(v.voter, N(bob));
    BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{1}, "unexpected choices");

    require_success(cast_vote(N(alice), id, N(carol), vector<uint8_t>{2}));
    p = get_poll(N(alice), id);
    BOOST_REQUIRE_EQUAL(p.votes.size(), 2);
    v = p.votes[1];
    BOOST_REQUIRE_EQUAL(v.voter, N(carol));
    BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{2}, "unexpected choices");

    require_success(cast_vote(N(alice), id, N(bob), vector<uint8_t>{0}));
    p = get_poll(N(alice), id);
    BOOST_REQUIRE_EQUAL(p.votes.size(), 2);
    v = p.votes[0];
    BOOST_REQUIRE_EQUAL(v.voter, N(bob));
    BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{0}, "unexpected choices");
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(num_choices_are_clamped, eosstrawpoll_tester)
try
{
    {
        const uuid id = require_create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A", "Option B", "Option C"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            0);
        const poll_t p = get_poll(N(alice), id);
        BOOST_REQUIRE_EQUAL(p.min_num_choices, 1);
        BOOST_REQUIRE_EQUAL(p.max_num_choices, 1);
    };
    {
        BOOST_REQUIRE_EQUAL(
            wasm_assert_msg("min_num_choices cannot be greater than the total number of options"),
            create_poll(
                N(alice),
                "Test poll",
                vector<string>{"Option A", "Option B", "Option C"},
                vector<account_name>{},
                vector<account_name>{},
                4,
                4,
                0,
                0));
        BOOST_REQUIRE_EQUAL(
            wasm_assert_msg("max_num_choices cannot be less than min_num_choices"),
            create_poll(
                N(alice),
                "Test poll",
                vector<string>{"Option A", "Option B", "Option C"},
                vector<account_name>{},
                vector<account_name>{},
                3,
                2,
                0,
                0));
        BOOST_REQUIRE_EQUAL(
            wasm_assert_msg("max_num_choices cannot be greater than the total number of options"),
            create_poll(
                N(alice),
                "Test poll",
                vector<string>{"Option A", "Option B", "Option C"},
                vector<account_name>{},
                vector<account_name>{},
                2,
                4,
                0,
                0));
    };
    {
        const uuid id = require_create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A", "Option B", "Option C"},
            vector<account_name>{},
            vector<account_name>{},
            2,
            2,
            0,
            0);
        const poll_t p = get_poll(N(alice), id);
        BOOST_REQUIRE_EQUAL(p.min_num_choices, 2);
        BOOST_REQUIRE_EQUAL(p.max_num_choices, 2);
    };
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_too_few_options, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("must have at least 2 options"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            0));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("must have at least 2 options"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            0));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_empty_options, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("empty options are not allowed"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"A", "B", ""},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            0));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_duplicate_options, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("duplicate options are not allowed"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"A", "B", "B"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            0));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_invalid_open_time, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("close_time must be after open_time"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A", "Option B"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            2,
            1));

    require_success(
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A", "Option B"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            2,
            0));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_already_closed, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("close_time must not be in the past"),
        create_poll(
            N(alice),
            "Test poll",
            vector<string>{"Option A", "Option B"},
            vector<account_name>{},
            vector<account_name>{},
            1,
            1,
            0,
            1));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_on_nonexistant_polls, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll doesn't exist"),
        cast_vote(
            N(janice),
            1,
            N(bob),
            vector<uint8_t>{}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll doesn't exist"),
        cast_vote(
            N(alice),
            9999,
            N(bob),
            vector<uint8_t>{}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_on_unopened_polls, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        1,
        now() + 10,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll has not opened yet"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    produce_block(fc::seconds(9));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll has not opened yet"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    produce_block(fc::seconds(1));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    const uuid id2 = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        1,
        now() - 10,
        0);

    require_success(
        cast_vote(
            N(alice),
            id2,
            N(bob),
            vector<uint8_t>{0}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_on_closed_polls, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        1,
        0,
        0);

    require_success(
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    const poll_t p = get_poll(N(alice), id);
    require_success(close_poll(N(alice), id));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll is closed"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    const uuid id2 = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        1,
        0,
        now() + 10);

    require_success(
        cast_vote(
            N(alice),
            id2,
            N(bob),
            vector<uint8_t>{0}));

    produce_block(fc::seconds(5));

    require_success(
        cast_vote(
            N(alice),
            id2,
            N(bob),
            vector<uint8_t>{0}));

    produce_block(fc::seconds(5));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll is closed"),
        cast_vote(
            N(alice),
            id2,
            N(bob),
            vector<uint8_t>{0}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_with_too_few_choices, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        2,
        2,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("too few choices"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("too few choices"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_with_too_many_choices, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        2,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("too many choices"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1, 2, 3}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("too many choices"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1, 2}));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_with_duplicate_choices, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B", "Option C"},
        vector<account_name>{},
        vector<account_name>{},
        1,
        3,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("duplicate choices are not allowed"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{1, 1, 0}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("duplicate choices are not allowed"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{1, 0, 1}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("duplicate choices are not allowed"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1, 0}));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 1, 2}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_with_invalid_choices, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{},
        2,
        2,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("received invalid choice"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{2, 3}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("received invalid choice"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0, 3}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_if_not_whitelisted, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{N(carol)},
        vector<account_name>{},
        1,
        1,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("voter is not on whitelist"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(carol),
            vector<uint8_t>{0}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_vote_if_blacklisted, eosstrawpoll_tester)
try
{
    const uuid id = require_create_poll(
        N(alice),
        "Test poll",
        vector<string>{"Option A", "Option B"},
        vector<account_name>{},
        vector<account_name>{N(bob), N(carol)},
        1,
        1,
        0,
        0);

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("voter is blacklisted"),
        cast_vote(
            N(alice),
            id,
            N(bob),
            vector<uint8_t>{0}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("voter is blacklisted"),
        cast_vote(
            N(alice),
            id,
            N(carol),
            vector<uint8_t>{0}));

    require_success(
        cast_vote(
            N(alice),
            id,
            N(alice),
            vector<uint8_t>{0}));
}
FC_LOG_AND_RETHROW()

BOOST_AUTO_TEST_SUITE_END()