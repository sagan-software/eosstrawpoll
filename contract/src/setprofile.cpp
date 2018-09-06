#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::setprofile(
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
    const vector<account_list_preset_t> &account_list_presets)
{
    require_auth(account);

    // validate profile
    eosio_assert(url.size() <= 144, "url is too big");
    eosio_assert(bio.size() <= 288, "bio is too big");
    eosio_assert(avatar_hash.size() <= 1000, "avatar_hash is too big");
    // TODO check if avatar_hash is valid IPFS hash
    eosio_assert(location.size() <= 50, "location is too big");

    // validate social media
    // TODO check that these IDs are not URLs, potentially use regex for stronger validation
    eosio_assert(github_id.size() <= 30, "github_id is too big");
    eosio_assert(twitter_id.size() <= 30, "twitter_id is too long");
    eosio_assert(steem_id.size() <= 30, "steem_id is too long");
    eosio_assert(medium_id.size() <= 30, "medium_id is too long");
    eosio_assert(twitch_id.size() <= 30, "twitter_id is too long");
    eosio_assert(youtube_id.size() <= 100, "youtube_id is too long");
    eosio_assert(theme.size() <= 1000, "theme is too long");

    // validate presets
    eosio_assert(account_list_presets.size() <= 10, "up to 10 account lists are allowed");
    for (auto &account_list_preset : account_list_presets)
    {
        eosio_assert(
            account_list_preset.description.size() <= 100,
            "account list preset description is too long");
        const auto account_list = account_list_preset.account_list;
        const auto account_list_len = account_list.size();
        eosio_assert(
            account_list_len <= global_config.max_account_list_len,
            "invalid preset: account_list is too long");
        for (auto &account : account_list)
        {
            eosio_assert(
                is_account(account),
                "invalid preset: account_list contains an account that doesn't exist");
        }
    }

    // check if the user has donated
    auto donor = donors_table.find(account);
    eosio_assert(donor != donors_table.end(), "user has never donated");

    // check if the user has donated enough to enable profiles
    eosio_assert(
        donor->donated >= global_config.profile_unlock_threshold,
        "user has not donated enough to unlock profiles");

    auto profile = profiles_table.find(account);
    if (profile != profiles_table.end())
    {
        profiles_table.modify(profile, account, [&](auto &p) {
            p.url = url;
            p.bio = bio;
            p.avatar_hash = avatar_hash;
            p.location = location;
            p.github_id = github_id;
            p.twitter_id = twitter_id;
            p.steem_id = steem_id;
            p.medium_id = medium_id;
            p.twitch_id = twitch_id;
            p.youtube_id = youtube_id;
            p.facebook_id = facebook_id;
            p.theme = theme;
            p.account_list_presets = account_list_presets;
        });
    }
    else
    {
        profiles_table.emplace(account, [&](auto &p) {
            p.account = account;
            p.url = url;
            p.bio = bio;
            p.avatar_hash = avatar_hash;
            p.location = location;
            p.github_id = github_id;
            p.twitter_id = twitter_id;
            p.steem_id = steem_id;
            p.medium_id = medium_id;
            p.twitch_id = twitch_id;
            p.youtube_id = youtube_id;
            p.facebook_id = facebook_id;
            p.theme = theme;
            p.account_list_presets = account_list_presets;
        });
    }
};

} // namespace eosstrawpoll
