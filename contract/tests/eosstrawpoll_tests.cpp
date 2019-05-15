#include <boost/test/unit_test.hpp>
#include <eosio/chain/contract_table_objects.hpp>
#include <eosio/chain/global_property_object.hpp>
#include <eosio/chain/resource_limits.hpp>
#include <eosio/chain/wast_to_wasm.hpp>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <fc/log/logger.hpp>
#include <eosio/chain/exceptions.hpp>
#include <Runtime/Runtime.h>

#include "eosstrawpoll_tester.hpp"

using namespace eosstrawpoll;

BOOST_AUTO_TEST_SUITE(eosstrawpoll_tests)

BOOST_FIXTURE_TEST_CASE(can_set_config, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        success(),
        setconfig(
            N(eosstrawpoll),
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1.1));

    BOOST_REQUIRE_EQUAL(
        "missing authority of eosstrawpoll",
        setconfig(
            N(alice1111111),
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1.1));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_create_polls, eosstrawpoll_tester)
try
{
    {
        const account_name account = N(alice1111111);
        const account_name id = N(test);
        const string title = "Test poll";
        const vector<string> options{"Option A", "Option B", "Option C"};
        const uint16_t min_answers = 1;
        const uint16_t max_answers = 3;
        const uint16_t min_writeins = 3;
        const uint16_t max_writeins = 3;
        const bool use_allow_list = true;
        const vector<account_name> voter_list{N(carol1111111)};
        const uint32_t min_voter_age_sec = 15;
        const vector<extended_asset> min_holdings{};
        const time_point_sec open_time = current_time_point_sec();
        const time_point_sec close_time = current_time_point_sec() + fc::seconds(700);
        BOOST_REQUIRE_EQUAL(
            success(),
            createpoll(
                account,
                id,
                account,
                title,
                options,
                min_answers,
                max_answers,
                min_writeins,
                max_writeins,
                use_allow_list,
                voter_list,
                min_voter_age_sec,
                min_holdings,
                open_time,
                close_time));
        const auto p = get_poll(N(polls), id);
        BOOST_REQUIRE_EQUAL(p.id, id);
        BOOST_REQUIRE_EQUAL(p.account, account);
        BOOST_REQUIRE_EQUAL(p.title, title);
        BOOST_REQUIRE_MESSAGE(p.options == options, "options are different");
        BOOST_REQUIRE_EQUAL(p.min_answers, min_answers);
        BOOST_REQUIRE_EQUAL(p.max_answers, max_answers);
        BOOST_REQUIRE_EQUAL(p.max_writeins, max_writeins);
        BOOST_REQUIRE_EQUAL(p.use_allow_list, use_allow_list);
        BOOST_REQUIRE_MESSAGE(p.voter_list == voter_list, "voter_list is different");
        BOOST_REQUIRE_EQUAL(p.open_time.to_iso_string(), open_time.to_iso_string());
        BOOST_REQUIRE_EQUAL(p.close_time.to_iso_string(), close_time.to_iso_string());
    }
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_empty_title, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("title must not be empty"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_whitespace_title, eosstrawpoll_tester)
try
{
    // TODO
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_too_long_title, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("title is longer than allowed by the global config"),
        createpoll(
            N(alice1111111),
            N(test1),
            N(alice1111111),
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));

    BOOST_REQUIRE_EQUAL(
        success(),
        setconfig(
            N(eosstrawpoll),
            100,
            100,
            5,
            50,
            80,
            300,
            5,
            80,
            100,
            1.1));

    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(test2),
            N(alice1111111),
            "12345",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));

    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("title is longer than allowed by the global config"),
        createpoll(
            N(alice1111111),
            N(test3),
            N(alice1111111),
            "123456",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_answers_gt_max_answers, eosstrawpoll_tester)
try
{
    // TODO
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_too_many_options, eosstrawpoll_tester)
try
{
    // TODO
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_max_writeins_gt_max_answers, eosstrawpoll_tester)
try
{
    // TODO
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_empty_options, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("empty options are not allowed"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B", ""},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_duplicate_options, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("duplicate options are not allowed"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_create_poll_with_min_voter_holdings, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.0000 TST"), N(eosio.token))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_voter_holdings_no_account, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min_voter_holdings references an account that doesn't exist"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.0000 TST"), N(notreal))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_voter_holdings_bad_contract, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min_voter_holdings references a symbol that doesn't exist"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.0000 TST"), N(eosio.saving))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_voter_holdings_bad_symbol, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min_voter_holdings references a symbol that doesn't exist"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.0000 SAGAN"), N(eosio.token))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_voter_holdings_bad_precision, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min_voter_holdings symbol precision mismatch"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.00 TST"), N(eosio.token))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_poll_with_min_voter_holdings_bad_quantity, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min_voter_holdings quantity is greater than supply"),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test poll",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1000000000.0001 TST"), N(eosio.token))},
            {},
            {}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(can_create_vote, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, ""}}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(cannot_create_invalid_vote, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("poll doesn't exist"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, ""}}));
    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{},
            {},
            {}));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("invalid answer: must have either a option index OR a writein answer, not neither or both. set option index to -1 or writein to an empty string"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{-1, ""}}));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("invalid answer: must have either a option index OR a writein answer, not neither or both. set option index to -1 or writein to an empty string"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, "A"}}));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("invalid answer: option index cannot be less than -1"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{-2, "A"}}));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("invalid answer: option index cannot be greater than the total number of options"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{2, ""}}));
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(create_vote_with_min_holdings, eosstrawpoll_tester)
try
{
    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(test),
            N(alice1111111),
            "Test",
            vector<string>{"A", "B"},
            1,
            1,
            0,
            0,
            true,
            vector<account_name>{},
            {},
            vector<extended_asset>{
                extended_asset(asset::from_string("1.0000 TST"), N(eosio.token))},
            {},
            {}));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min voter holdings criteria not met -- no balance object found"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, ""}}));
    issue(N(alice1111111), asset::from_string("0.9999 TST"));
    BOOST_REQUIRE_EQUAL(
        wasm_assert_msg("min voter holdings criteria not met -- balance not high enough"),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, ""}}));
    issue(N(alice1111111), asset::from_string("0.0001 TST"));
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(alice1111111),
            N(test),
            N(alice1111111),
            vector<answer_t>{{0, ""}}));
}
FC_LOG_AND_RETHROW()

BOOST_AUTO_TEST_SUITE_END()
