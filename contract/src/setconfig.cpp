#include "contract.hpp"

namespace eosstrawpoll
{

void contract::setconfig(
    const uint64_t max_latest,
    const uint64_t max_popular,
    const uint64_t max_title_len,
    const uint64_t max_options_len,
    const uint64_t max_option_len,
    const uint64_t max_voter_list_len,
    const uint64_t max_min_voter_holdings_len,
    const uint64_t max_writein_len,
    const uint64_t max_answers_len,
    const double popularity_gravity)
{
    require_auth(_self);

    global_config = global_config_t{
        .max_latest = max_latest,
        .max_popular = max_popular,
        .max_title_len = max_title_len,
        .max_options_len = max_options_len,
        .max_option_len = max_option_len,
        .max_voter_list_len = max_voter_list_len,
        .max_min_voter_holdings_len = max_min_voter_holdings_len,
        .max_writein_len = max_writein_len,
        .max_answers_len = max_answers_len,
        .popularity_gravity = popularity_gravity};
    global_config_singleton.set(global_config, _self);

    prune_latest();
    prune_popular();
};
} // namespace eosstrawpoll