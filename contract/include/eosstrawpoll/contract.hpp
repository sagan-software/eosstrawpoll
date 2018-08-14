#pragma once

#include <eosstrawpoll/types.hpp>
#include <eosiolib/currency.hpp>

namespace eosstrawpoll
{

class contract
{

  private:
    account_name _self;
    config_table _configs;
    config _config;
    new_donations_table _new_donations;
    donors_table _donors;
    popular_polls_table _popular_polls;
    new_polls_table _new_polls;

    void prune_new_donations();
    void prune_new_polls();
    void prune_popular_polls();
    bool is_popular_polls_full();

  public:
    contract(account_name self);

    inline account_name get_self() const { return _self; }

    void setconfig(
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
        const string &metadata);

    void createpoll(
        const account_name creator,
        const poll_name slug,
        const string &title,
        const vector<string> &options,
        const uint16_t min_num_choices,
        const uint16_t max_num_choices,
        const vector<account_name> &whitelist,
        const vector<account_name> &blacklist,
        const time open_time,
        const time close_time,
        const string &metadata);

    void destroypoll(
        const account_name creator,
        const poll_name slug,
        const string &metadata);

    void createvote(
        const account_name creator,
        const poll_name slug,
        const account_name voter,
        const vector<uint16_t> &choices,
        const string &metadata);

    void destroyvote(
        const account_name creator,
        const poll_name slug,
        const account_name voter,
        const string &metadata);

    void transfer(
        const eosio::currency::transfer &t,
        const account_name code);

    void apply(
        const account_name contract,
        const account_name action);
};

} // namespace eosstrawpoll