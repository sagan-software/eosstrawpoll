#pragma once

#include "types.hpp"
#include <eosiolib/singleton.hpp>

namespace eosstrawpoll
{

// @abi table config i64
struct config
{
    uint16_t max_new_polls = 50;
    uint16_t max_popular_polls = 50;
    uint16_t max_closed_polls = 50;
    uint16_t max_top_donors = 100;
    uint16_t max_donations = 100;
    uint16_t max_choices_size = 50;
    uint16_t max_title_size = 144;
    uint16_t max_options_size = 50;
    uint16_t max_option_size = 144;
    uint16_t max_whitelist_size = 500;
    uint16_t max_blacklist_size = 500;
    uint32_t min_duration = 60 * 5;
    vector<account_name> superusers;
    vector<account_name> moderators;
    vector<account_name> blacklist;
    vector<account_name> graylist;
    double popularity_gravity = 1.8;
    uint64_t max_metadata_size = 10000;
    string metadata = "";

    EOSLIB_SERIALIZE(
        config,
        // tables
        (max_new_polls)(max_popular_polls)(max_closed_polls)
        // donations
        (max_top_donors)(max_donations)
        // polls
        (max_choices_size)(max_title_size)(max_options_size)(max_option_size)(max_whitelist_size)(max_blacklist_size)(min_duration)
        // account lists
        (superusers)(moderators)(blacklist)(graylist)
        // misc
        (popularity_gravity)(max_metadata_size)(metadata))
};

typedef eosio::singleton<N(config), config>
    config_index;
} // namespace eosstrawpoll