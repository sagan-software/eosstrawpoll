#pragma once

#include <eosstrawpoll/types.hpp>
#include <eosiolib/currency.hpp>

namespace eosstrawpoll
{

class contract
{

  private:
    account_name _self;
    global_config_t global_config;

    // tables
    global_config_table_t global_config_table;
    polls_table_t polls_table;
    votes_table_t votes_table;
    popular_polls_table_t popular_polls_table;
    new_polls_table_t new_polls_table;
    new_donations_table_t new_donations_table;
    donors_table_t donors_table;
    profiles_table_t profiles_table;

    // utils
    void prune_new_donations();
    void prune_new_polls();
    void prune_popular_polls();
    bool is_popular_polls_full();
    void ensure_user(const account_name account);
    uint32_t get_num_votes(const poll_id_t poll_id);
    double calculate_popularity(const uint32_t num_votes, const time_t start_time);

  public:
    contract(account_name self);

    inline account_name get_self() const { return _self; }

    void clearprofile(const account_name account);

    void closepoll(const poll_id_t poll_id);

    void createpoll(
        const poll_id_t id,
        const account_name account,
        const string &title,
        const vector<string> &prefilled_options,
        const uint16_t min_answers,
        const uint16_t max_answers,
        const uint16_t max_writein_answers,
        const bool use_allow_list,
        const vector<account_name> &account_list,
        const time_t open_time,
        const time_t close_time);

    void createvote(
        const poll_id_t poll_id,
        const account_name account,
        const vector<answer_t> &answers);

    void destroyold(const string &table, const account_name scope);

    void destroypoll(const poll_id_t poll_id);

    void destroytable(const string &table);

    void destroyvote(
        const poll_id_t poll_id,
        const account_name account);

    void destroyvotes(const poll_id_t poll_id);

    void openpoll(const poll_id_t poll_id);

    void setconfig(
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
        const uint64_t profile_unlock_threshold);

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
        const vector<account_list_preset_t> &account_list_presets);

    void transfer(
        const eosio::currency::transfer &t,
        const account_name code);

    void apply(
        const account_name contract,
        const account_name action);
};

} // namespace eosstrawpoll