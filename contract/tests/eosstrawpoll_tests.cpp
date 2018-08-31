#include <boost/test/unit_test.hpp>
#include <eosio/chain/contract_table_objects.hpp>
#include <eosio/chain/global_property_object.hpp>
#include <eosio/chain/resource_limits.hpp>
#include <eosio/chain/wast_to_wasm.hpp>
#include <cstdlib>
#include <iostream>
#include <fc/log/logger.hpp>
#include <eosio/chain/exceptions.hpp>
#include <Runtime/Runtime.h>

#include "eosstrawpoll_tester.hpp"

using namespace eosstrawpoll;

BOOST_AUTO_TEST_SUITE(eosstrawpoll_tests)

BOOST_FIXTURE_TEST_CASE(setconfig, eosstrawpoll_tester)
try
{
    {
        auto out = setconfig(
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
            1.1,
            1);
        BOOST_REQUIRE_EQUAL(success(), out);
    }
    {
        auto out = setconfig(
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
            1.1,
            1);
        BOOST_REQUIRE_EQUAL(
            "missing authority of eosstrawpoll",
            out);
    }
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(createpolls, eosstrawpoll_tester)
try
{
    {
        const account_name creator = N(alice1111111);
        const poll_name slug = N(test);
        const string title = "Test poll";
        const vector<string> options{"Option A", "Option B", "Option C"};
        const uint8_t min_choices = 1;
        const uint8_t max_choices = 3;
        const uint8_t max_writeins = 3;
        const bool use_allow_list = true;
        const vector<account_name> account_list{N(carol1111111)};
        const uint64_t min_staked = 0;
        const uint64_t min_value = 0;
        const esptime open_time = now();
        const esptime close_time = now() + 700;
        auto out = createpoll(
            creator,
            creator,
            slug,
            title,
            options,
            min_choices,
            max_choices,
            max_writeins,
            use_allow_list,
            account_list,
            min_staked,
            min_value,
            open_time,
            close_time);
        BOOST_REQUIRE_EQUAL(success(), out);
        const poll p = get_poll(creator, N(polls), slug);
        // BOOST_REQUIRE_EQUAL(p.id, slug);
        BOOST_REQUIRE_EQUAL(p.creator, creator);
        // BOOST_REQUIRE_EQUAL(p.slug, slug);
        BOOST_REQUIRE_EQUAL(p.title, title);
        BOOST_REQUIRE_MESSAGE(p.options == options, "options are different");
        BOOST_REQUIRE_EQUAL(p.min_choices, min_choices);
        BOOST_REQUIRE_EQUAL(p.max_choices, max_choices);
        BOOST_REQUIRE_EQUAL(p.max_writeins, max_writeins);
        BOOST_REQUIRE_MESSAGE(p.use_allow_list, use_allow_list);
        BOOST_REQUIRE_MESSAGE(p.account_list == account_list, "account_list is different");
        BOOST_REQUIRE_EQUAL(p.min_staked, min_staked);
        BOOST_REQUIRE_EQUAL(p.min_value, min_value);
        BOOST_REQUIRE_EQUAL(p.open_time, open_time);
        BOOST_REQUIRE_EQUAL(p.close_time, close_time);
    }
}
FC_LOG_AND_RETHROW()

BOOST_FIXTURE_TEST_CASE(createvotes, eosstrawpoll_tester)
try
{
    // create poll
    BOOST_REQUIRE_EQUAL(
        success(),
        createpoll(
            N(alice1111111),
            N(alice1111111),
            N(test),
            "Test poll",
            {"A", "B", "C"},
            1,
            2,
            2,
            true,
            {},
            0,
            0,
            0,
            0));

    // create basic vote
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(bob111111111),
            N(alice1111111),
            N(test),
            N(bob111111111),
            {{.option_index = 1, .writein = ""}}));
    poll p = get_poll(N(alice1111111), N(polls), N(test));
    BOOST_REQUIRE_EQUAL(p.votes.size(), 1);

    espvote v = p.votes[0];
    BOOST_REQUIRE_EQUAL(v.choices.size(), 1);

    choice c = v.choices[0];
    BOOST_REQUIRE_EQUAL(c.option_index, 1);
    BOOST_REQUIRE_EQUAL(c.writein, "");

    // create vote with writein
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(carol1111111),
            N(alice1111111),
            N(test),
            N(carol1111111),
            {{.option_index = -1, .writein = "testing"}}));
    p = get_poll(N(alice1111111), N(polls), N(test));
    BOOST_REQUIRE_EQUAL(p.votes.size(), 2);

    v = p.votes[1];
    BOOST_REQUIRE_EQUAL(v.choices.size(), 1);

    c = v.choices[0];
    BOOST_REQUIRE_EQUAL(c.option_index, -1);
    BOOST_REQUIRE_EQUAL(c.writein, "testing");

    // change writein vote
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(carol1111111),
            N(alice1111111),
            N(test),
            N(carol1111111),
            {{.option_index = -1, .writein = "testing changing votes"}}));
    p = get_poll(N(alice1111111), N(polls), N(test));
    BOOST_REQUIRE_EQUAL(p.votes.size(), 2);

    v = p.votes[1];
    BOOST_REQUIRE_EQUAL(v.choices.size(), 1);

    c = v.choices[0];
    BOOST_REQUIRE_EQUAL(c.option_index, -1);
    BOOST_REQUIRE_EQUAL(c.writein, "testing changing votes");

    // change writein vote to option_index
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(carol1111111),
            N(alice1111111),
            N(test),
            N(carol1111111),
            {{.option_index = 2, .writein = ""}}));
    p = get_poll(N(alice1111111), N(polls), N(test));
    BOOST_REQUIRE_EQUAL(p.votes.size(), 2);

    v = p.votes[1];
    BOOST_REQUIRE_EQUAL(v.choices.size(), 1);

    c = v.choices[0];
    BOOST_REQUIRE_EQUAL(c.option_index, 2);
    BOOST_REQUIRE_EQUAL(c.writein, "");

    // multiple writeins
    BOOST_REQUIRE_EQUAL(
        success(),
        createvote(
            N(alice1111111),
            N(alice1111111),
            N(test),
            N(alice1111111),
            {{.option_index = -1, .writein = "writein 1"}, {.option_index = -1, .writein = "writein 2"}}));
    p = get_poll(N(alice1111111), N(polls), N(test));
    BOOST_REQUIRE_EQUAL(p.votes.size(), 3);

    v = p.votes[2];
    BOOST_REQUIRE_EQUAL(v.choices.size(), 2);

    c = v.choices[0];
    BOOST_REQUIRE_EQUAL(c.option_index, -1);
    BOOST_REQUIRE_EQUAL(c.writein, "writein 1");

    c = v.choices[1];
    BOOST_REQUIRE_EQUAL(c.option_index, -1);
    BOOST_REQUIRE_EQUAL(c.writein, "writein 2");
}
FC_LOG_AND_RETHROW()

BOOST_AUTO_TEST_SUITE_END()

// BOOST_FIXTURE_TEST_CASE(can_close_polls, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B", "Option C"},
//         vector<account_name>{N(name1), N(name2)},
//         vector<account_name>{N(name3), N(name4)},
//         1,
//         1,
//         0,
//         0);

//     require_success(close_poll(N(alice), id));

//     const poll_t p = get_poll(N(alice), id);
//     BOOST_REQUIRE_MESSAGE(p.close_time > 0, "close_time should be greater than 0");
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(can_cast_votes, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B", "Option C"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         1,
//         0,
//         0);

//     poll_t p;
//     vote_t v;

//     p = get_poll(N(alice), id);
//     BOOST_REQUIRE_EQUAL(p.votes.size(), 0);

//     require_success(cast_vote(N(alice), id, N(bob), vector<uint8_t>{1}));
//     p = get_poll(N(alice), id);
//     BOOST_REQUIRE_EQUAL(p.votes.size(), 1);
//     v = p.votes[0];
//     BOOST_REQUIRE_EQUAL(v.voter, N(bob));
//     BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{1}, "unexpected choices");

//     require_success(cast_vote(N(alice), id, N(carol), vector<uint8_t>{2}));
//     p = get_poll(N(alice), id);
//     BOOST_REQUIRE_EQUAL(p.votes.size(), 2);
//     v = p.votes[1];
//     BOOST_REQUIRE_EQUAL(v.voter, N(carol));
//     BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{2}, "unexpected choices");

//     require_success(cast_vote(N(alice), id, N(bob), vector<uint8_t>{0}));
//     p = get_poll(N(alice), id);
//     BOOST_REQUIRE_EQUAL(p.votes.size(), 2);
//     v = p.votes[0];
//     BOOST_REQUIRE_EQUAL(v.voter, N(bob));
//     BOOST_REQUIRE_MESSAGE(v.choices == vector<uint8_t>{0}, "unexpected choices");
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(num_choices_are_clamped, eosstrawpoll_tester)
// try
// {
//     {
//         const uuid id = require_create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A", "Option B", "Option C"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             0);
//         const poll_t p = get_poll(N(alice), id);
//         BOOST_REQUIRE_EQUAL(p.min_choices, 1);
//         BOOST_REQUIRE_EQUAL(p.max_choices, 1);
//     };
//     {
//         BOOST_REQUIRE_EQUAL(
//             wasm_assert_msg("min_choices cannot be greater than the total number of options"),
//             create_poll(
//                 N(alice),
//                 "Test poll",
//                 vector<string>{"Option A", "Option B", "Option C"},
//                 vector<account_name>{},
//                 vector<account_name>{},
//                 4,
//                 4,
//                 0,
//                 0));
//         BOOST_REQUIRE_EQUAL(
//             wasm_assert_msg("max_choices cannot be less than min_choices"),
//             create_poll(
//                 N(alice),
//                 "Test poll",
//                 vector<string>{"Option A", "Option B", "Option C"},
//                 vector<account_name>{},
//                 vector<account_name>{},
//                 3,
//                 2,
//                 0,
//                 0));
//         BOOST_REQUIRE_EQUAL(
//             wasm_assert_msg("max_choices cannot be greater than the total number of options"),
//             create_poll(
//                 N(alice),
//                 "Test poll",
//                 vector<string>{"Option A", "Option B", "Option C"},
//                 vector<account_name>{},
//                 vector<account_name>{},
//                 2,
//                 4,
//                 0,
//                 0));
//     };
//     {
//         const uuid id = require_create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A", "Option B", "Option C"},
//             vector<account_name>{},
//             vector<account_name>{},
//             2,
//             2,
//             0,
//             0);
//         const poll_t p = get_poll(N(alice), id);
//         BOOST_REQUIRE_EQUAL(p.min_choices, 2);
//         BOOST_REQUIRE_EQUAL(p.max_choices, 2);
//     };
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_too_few_options, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("must have at least 2 options"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             0));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("must have at least 2 options"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             0));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_empty_options, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("empty options are not allowed"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"A", "B", ""},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             0));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_duplicate_options, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("duplicate options are not allowed"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"A", "B", "B"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             0));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_create_polls_with_invalid_open_time, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("close_time must be after open_time"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A", "Option B"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             2,
//             1));

//     require_success(
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A", "Option B"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             2,
//             0));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_create_polls_already_closed, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("close_time must not be in the past"),
//         create_poll(
//             N(alice),
//             "Test poll",
//             vector<string>{"Option A", "Option B"},
//             vector<account_name>{},
//             vector<account_name>{},
//             1,
//             1,
//             0,
//             1));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_on_nonexistant_polls, eosstrawpoll_tester)
// try
// {
//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll doesn't exist"),
//         cast_vote(
//             N(janice),
//             1,
//             N(bob),
//             vector<uint8_t>{}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll doesn't exist"),
//         cast_vote(
//             N(alice),
//             9999,
//             N(bob),
//             vector<uint8_t>{}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_on_unopened_polls, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         1,
//         now() + 10,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll has not opened yet"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     produce_block(fc::seconds(9));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll has not opened yet"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     produce_block(fc::seconds(1));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     const uuid id2 = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         1,
//         now() - 10,
//         0);

//     require_success(
//         cast_vote(
//             N(alice),
//             id2,
//             N(bob),
//             vector<uint8_t>{0}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_on_closed_polls, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         1,
//         0,
//         0);

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     const poll_t p = get_poll(N(alice), id);
//     require_success(close_poll(N(alice), id));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll is closed"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     const uuid id2 = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         1,
//         0,
//         now() + 10);

//     require_success(
//         cast_vote(
//             N(alice),
//             id2,
//             N(bob),
//             vector<uint8_t>{0}));

//     produce_block(fc::seconds(5));

//     require_success(
//         cast_vote(
//             N(alice),
//             id2,
//             N(bob),
//             vector<uint8_t>{0}));

//     produce_block(fc::seconds(5));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("poll is closed"),
//         cast_vote(
//             N(alice),
//             id2,
//             N(bob),
//             vector<uint8_t>{0}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_with_too_few_choices, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         2,
//         2,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("too few choices"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("too few choices"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_with_too_many_choices, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         2,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("too many choices"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1, 2, 3}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("too many choices"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1, 2}));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_with_duplicate_choices, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B", "Option C"},
//         vector<account_name>{},
//         vector<account_name>{},
//         1,
//         3,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("duplicate choices are not allowed"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{1, 1, 0}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("duplicate choices are not allowed"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{1, 0, 1}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("duplicate choices are not allowed"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1, 0}));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 1, 2}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_with_invalid_choices, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{},
//         2,
//         2,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("received invalid choice"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{2, 3}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("received invalid choice"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0, 3}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_if_not_whitelisted, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{N(carol)},
//         vector<account_name>{},
//         1,
//         1,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("voter is not on whitelist"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(carol),
//             vector<uint8_t>{0}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_FIXTURE_TEST_CASE(cannot_vote_if_blacklisted, eosstrawpoll_tester)
// try
// {
//     const uuid id = require_create_poll(
//         N(alice),
//         "Test poll",
//         vector<string>{"Option A", "Option B"},
//         vector<account_name>{},
//         vector<account_name>{N(bob), N(carol)},
//         1,
//         1,
//         0,
//         0);

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("voter is blacklisted"),
//         cast_vote(
//             N(alice),
//             id,
//             N(bob),
//             vector<uint8_t>{0}));

//     BOOST_REQUIRE_EQUAL(
//         wasm_assert_msg("voter is blacklisted"),
//         cast_vote(
//             N(alice),
//             id,
//             N(carol),
//             vector<uint8_t>{0}));

//     require_success(
//         cast_vote(
//             N(alice),
//             id,
//             N(alice),
//             vector<uint8_t>{0}));
// }
// FC_LOG_AND_RETHROW()

// BOOST_AUTO_TEST_SUITE_END()