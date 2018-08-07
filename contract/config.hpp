#pragma once

#include "types.hpp"
#include <eosiolib/singleton.hpp>

namespace eosstrawpoll
{
// @abi table config i64
struct config
{
    uint16_t max_recent_polls = 50;
    uint16_t max_popular_polls = 50;
    uint16_t max_closed_polls = 50;
    uint16_t max_top_donors = 100;
    uint16_t max_recent_donations = 100;
    uint16_t max_choices_len = 50;
    uint16_t max_title_len = 144;
    uint16_t max_options_len = 50;
    uint16_t max_option_len = 144;
    uint16_t max_whitelist_len = 500;
    uint16_t max_blacklist_len = 500;
    uint32_t min_duration = 60 * 5;
    vector<account_name> blacklist;
    vector<account_name> graylist;
    double popularity_gravity = 1.8;

    EOSLIB_SERIALIZE(config, (max_recent_polls)(max_popular_polls)(max_closed_polls)(max_top_donors)(max_recent_donations)(max_choices_len)(max_title_len)(max_options_len)(max_option_len)(max_whitelist_len)(max_blacklist_len)(min_duration)(blacklist)(graylist)(popularity_gravity))
};

typedef eosio::singleton<N(config), config>
    config_index;
} // namespace eosstrawpoll