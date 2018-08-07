#include "include/contract.hpp"

namespace eosstrawpoll
{
// @abi action
void contract::setconfig(
    const uint16_t max_new_polls,
    const uint16_t max_popular_polls,
    const uint16_t max_closed_polls,
    const uint16_t max_top_donors,
    const uint16_t max_new_donations,
    const uint16_t max_choices_len,
    const uint16_t max_title_len,
    const uint16_t max_options_len,
    const uint16_t max_option_len,
    const uint16_t max_whitelist_len,
    const uint16_t max_blacklist_len,
    const timestamp min_duration,
    vector<account_name> &superusers,
    vector<account_name> &moderators,
    const vector<account_name> &blacklist,
    const vector<account_name> &graylist,
    const double popularity_gravity,
    const uint64_t max_metadata_size,
    const string &metadata){};
} // namespace eosstrawpoll