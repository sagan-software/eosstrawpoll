#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::setconfig(
    const uint16_t max_new_polls,
    const uint16_t max_popular_polls,
    const uint16_t max_new_donations,
    const uint16_t max_title_len,
    const uint16_t max_prefilled_options_len,
    const uint16_t max_prefilled_option_len,
    const uint16_t max_account_list_len,
    const uint16_t max_writein_len,
    const uint16_t max_answers_len,
    const double popularity_gravity,
    const uint64_t profile_unlock_threshold)
{
    require_auth(_self);

    global_config = global_config_t{
        .max_new_polls = max_new_polls,
        .max_popular_polls = max_popular_polls,
        .max_new_donations = max_new_donations,
        .max_title_len = max_title_len,
        .max_prefilled_options_len = max_prefilled_options_len,
        .max_prefilled_option_len = max_prefilled_option_len,
        .max_account_list_len = max_account_list_len,
        .max_writein_len = max_writein_len,
        .max_answers_len = max_answers_len,
        .popularity_gravity = popularity_gravity,
        .profile_unlock_threshold = profile_unlock_threshold};
    global_config_table.set(global_config, _self);

    prune_new_donations();
    prune_new_polls();
    prune_popular_polls();
};
} // namespace eosstrawpoll