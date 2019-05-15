#include "contract.hpp"

namespace eosstrawpoll
{

void contract::setconfig(
    const optional<uint64_t> max_latest,
    const optional<uint64_t> max_popular,
    const optional<uint64_t> max_title_len,
    const optional<uint64_t> max_options_len,
    const optional<uint64_t> max_option_len,
    const optional<uint64_t> max_voter_list_len,
    const optional<uint64_t> max_min_voter_holdings_len,
    const optional<uint64_t> max_writein_len,
    const optional<uint64_t> max_answers_len,
    const optional<double> popularity_gravity)
{
    require_auth(_self);

    if (max_latest)
    {
        global_config.max_latest = *max_latest;
    }
    if (max_popular)
    {
        global_config.max_popular = *max_popular;
    }
    if (max_title_len)
    {
        global_config.max_title_len = *max_title_len;
    }
    if (max_options_len)
    {
        global_config.max_options_len = *max_options_len;
    }
    if (max_option_len)
    {
        global_config.max_option_len = *max_option_len;
    }
    if (max_voter_list_len)
    {
        global_config.max_voter_list_len = *max_voter_list_len;
    }
    if (max_writein_len)
    {
        global_config.max_writein_len = *max_writein_len;
    }
    if (max_answers_len)
    {
        global_config.max_answers_len = *max_answers_len;
    }
    if (popularity_gravity)
    {
        global_config.popularity_gravity = *popularity_gravity;
    }

    global_config_singleton.set(global_config, _self);

    prune_latest();
    prune_popular();
};
} // namespace eosstrawpoll