#include "eosstrawpoll_tester.hpp"

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

// BOOST_FIXTURE_TEST_CASE(can_destroy_polls, eosstrawpoll_tester)
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

//     require_success(destroy_poll(N(alice), id));
// }
// FC_LOG_AND_RETHROW()

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
//         BOOST_REQUIRE_EQUAL(p.min_num_choices, 1);
//         BOOST_REQUIRE_EQUAL(p.max_num_choices, 1);
//     };
//     {
//         BOOST_REQUIRE_EQUAL(
//             wasm_assert_msg("min_num_choices cannot be greater than the total number of options"),
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
//             wasm_assert_msg("max_num_choices cannot be less than min_num_choices"),
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
//             wasm_assert_msg("max_num_choices cannot be greater than the total number of options"),
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
//         BOOST_REQUIRE_EQUAL(p.min_num_choices, 2);
//         BOOST_REQUIRE_EQUAL(p.max_num_choices, 2);
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

BOOST_AUTO_TEST_SUITE_END()