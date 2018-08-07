#include "eosstrawpoll.hpp"

namespace eosstrawpoll
{
// @abi action
void contract::setconfig(
    const uint16_t max_recent_polls,
    const uint16_t max_popular_polls,
    const uint16_t max_closed_polls,
    const uint16_t max_top_donors,
    const uint16_t max_recent_donations,
    const uint16_t max_choices_len,
    const uint16_t max_title_len,
    const uint16_t max_options_len,
    const uint16_t max_option_len,
    const uint16_t max_whitelist_len,
    const uint16_t max_blacklist_len,
    const timestamp min_duration,
    const vector<account_name> &blacklist,
    const vector<account_name> &graylist,
    const double popularity_gravity,
    const string &metadata){};
}