#pragma once

#include "config.hpp"
#include "donations.hpp"
#include "polls.hpp"

namespace eosstrawpoll
{

class contract
{

  private:
    account_name _self;
    config_index _config;
    donations_index _donations;
    top_donors_index _top_donors;
    popular_polls_index _popular_polls;
    new_polls_index _new_polls;
    closed_polls_index _closed_polls;

  public:
    contract(account_name self)
        : _self(self),
          _config(self, self),
          _donations(self, self),
          _top_donors(self, self),
          _popular_polls(self, self),
          _new_polls(self, self),
          _closed_polls(self, self) {}

    inline account_name get_self() const { return _self; }

    void setconfig(
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
        const vector<account_name> &blacklist,
        const vector<account_name> &graylist,
        const double popularity_gravity,
        const string &metadata);

    void createpoll(
        const account_name creator,
        const poll_name name,
        const string &title,
        const vector<string> &options,
        const vector<account_name> &whitelist,
        const vector<account_name> &blacklist,
        const uint16_t min_num_choices,
        const uint16_t max_num_choices,
        const timestamp open_time,
        const timestamp close_time,
        const string &metadata);

    void closepoll(
        const account_name creator,
        const poll_name name,
        const string &metadata);

    void destroypoll(
        const account_name creator,
        const poll_name name,
        const string &metadata);

    void createvote(
        const account_name creator,
        const poll_name name,
        const account_name voter,
        const vector<uint16_t> &choices,
        const string &metadata);

    void destroyvote(
        const account_name creator,
        const poll_name name,
        const account_name voter,
        const string &metadata);
};

} // namespace eosstrawpoll