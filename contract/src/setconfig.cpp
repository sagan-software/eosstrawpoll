#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::setconfig(
    const uint16_t max_new_polls,
    const uint16_t max_popular_polls,
    const uint16_t max_new_donations,
    const uint16_t max_title_size,
    const uint16_t max_options_size,
    const uint16_t max_option_size,
    const uint16_t max_whitelist_size,
    const uint16_t max_blacklist_size,
    const uint32_t min_duration,
    const vector<account_name> &blacklist,
    const vector<account_name> &graylist,
    const double popularity_gravity,
    const uint16_t max_metadata_size,
    const string &metadata)
{
    require_auth(_self);

    _config = config{
        .max_new_polls = max_new_polls,
        .max_popular_polls = max_popular_polls,
        .max_new_donations = max_new_donations,
        .max_title_size = max_title_size,
        .max_options_size = max_options_size,
        .max_option_size = max_option_size,
        .max_whitelist_size = max_whitelist_size,
        .max_blacklist_size = max_blacklist_size,
        .min_duration = min_duration,
        .blacklist = blacklist,
        .graylist = graylist,
        .popularity_gravity = popularity_gravity,
        .max_metadata_size = max_metadata_size,
        .metadata = metadata};
    _configs.set(_config, _self);

    prune_new_donations();
    prune_new_polls();
    prune_popular_polls();
};
} // namespace eosstrawpoll