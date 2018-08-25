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
    users_table _users;
    popular_polls_table _popular_polls;
    new_polls_table _new_polls;

    void prune_new_donations();
    void prune_new_polls();
    void prune_popular_polls();
    bool is_popular_polls_full();
    void ensure_user(const account_name account);
    void assert_not_banned(const account_name account);
    void assert_is_moderator(const account_name account);
    void assert_metadata_len(const string &metadata);

  public:
    contract(account_name self);

    inline account_name get_self() const { return _self; }

    void banuser(
        const account_name moderator,
        const account_name account,
        const time expiration,
        const string &reason,
        const string &metadata);

    void clearprofile(
        const account_name account,
        const string &metadata);

    void closepoll(
        const account_name creator,
        const poll_name slug,
        const string &metadata);

    void createpoll(
        const account_name creator,
        const poll_name slug,
        const string &title,
        const vector<string> &options,
        const uint16_t min_choices,
        const uint16_t max_choices,
        const uint16_t max_writeins,
        const bool use_allow_list,
        const vector<account_name> &account_list,
        const uint64_t min_staked,
        const uint64_t min_value,
        const time open_time,
        const time close_time,
        const string &metadata);

    void createvote(
        const account_name creator,
        const poll_name slug,
        const account_name voter,
        const vector<choice> &choices,
        const string &metadata);

    void destroypoll(
        const account_name creator,
        const poll_name slug,
        const string &metadata);

    void destroyvote(
        const account_name creator,
        const poll_name slug,
        const account_name voter,
        const string &metadata);

    void destroyvotes(
        const account_name creator,
        const poll_name slug,
        const string &metadata);

    void moduser(
        const account_name account,
        const time expiration,
        const string &metadata);

    void openpoll(
        const account_name creator,
        const poll_name slug,
        const string &metadata);

    void setconfig(
        const uint16_t max_new_polls,
        const uint16_t max_popular_polls,
        const uint16_t max_new_donations,
        const uint16_t max_title_len,
        const uint16_t max_options_len,
        const uint16_t max_option_len,
        const uint16_t max_account_list_len,
        const uint16_t max_writein_len,
        const double popularity_gravity,
        const uint16_t max_metadata_len,
        const uint64_t profile_unlock_threshold,
        const string &metadata);

    void setprofile(
        const account_name account,
        const string &url,
        const string &bio,
        const string &avatar_hash,
        const string &location,
        const string &github_id,
        const string &twitter_id,
        const string &steem_id,
        const string &medium_id,
        const string &twitch_id,
        const string &youtube_id,
        const string &facebook_id,
        const string &theme,
        const vector<preset> &presets,
        const string &metadata);

    void transfer(
        const eosio::currency::transfer &t,
        const account_name code);

    void unbanuser(
        const account_name moderator,
        const account_name account,
        const string &metadata);

    void unmoduser(
        const account_name account,
        const string &metadata);

    void apply(
        const account_name contract,
        const account_name action);
};

} // namespace eosstrawpoll