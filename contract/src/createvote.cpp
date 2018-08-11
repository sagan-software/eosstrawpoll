#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::createvote(
    const account_name creator,
    const poll_name slug,
    const account_name voter,
    const vector<uint16_t> &choices,
    const string &metadata)
{
    require_auth(voter);
    eosio_assert(metadata.size() <= _config.max_metadata_size, "metadata is too long");

    // check if poll exists
    polls_table _creator_polls(_self, creator);
    auto p = _creator_polls.find(slug);
    eosio_assert(p != _creator_polls.end(), "poll doesn't exist");

    // check if poll has opened yet
    eosio_assert(p->has_opened(), "poll has not opened yet");

    // check if poll is closed
    eosio_assert(!p->is_closed(), "poll is closed");

    // check number choices can be selected
    const auto num_choices = choices.size();
    eosio_assert(num_choices >= p->min_num_choices, "too few choices");
    eosio_assert(num_choices <= p->max_num_choices, "too many choices");

    // check if choices are valid
    const auto max_choice = p->options.size() - 1;
    std::map<uint8_t, bool> seen_choices;
    for (auto &choice : choices)
    {
        eosio_assert(choice >= 0 && choice <= max_choice, "received invalid choice");
        eosio_assert(seen_choices.count(choice) == 0, "duplicate choices are not allowed");
        seen_choices[choice] = true;
    }

    // check if poll has whitelist, and if voter is on whitelist
    const auto wl = p->whitelist;
    eosio_assert(
        wl.empty() || std::find(wl.begin(), wl.end(), voter) != wl.end(),
        "voter is not on whitelist");

    // check if voter is on blacklist
    const auto bl = p->blacklist;
    eosio_assert(
        bl.empty() || std::find(bl.begin(), bl.end(), voter) == bl.end(),
        "voter is blacklisted");

    // create vote
    vote v;
    v.voter = voter;
    v.choices = choices;
    v.created = now();
    v.staked = 0;

    vector<vote> votes = p->votes;

    // cast vote
    _creator_polls.modify(p, voter, [&](auto &p) {
        // check if voter has voted already
        for (size_t i = 0; i < p.votes.size(); i++)
        {
            const vote current_vote = p.votes[i];
            if (current_vote.voter == voter)
            {
                p.votes[i] = v;
                votes = p.votes;
                return;
            }
        }

        // new voter, add to end of vector
        p.votes.push_back(v);
        votes = p.votes;
    });

    // check popular poll's table
    double lowest_popularity = 999999999;
    bool updated_poll = false;
    auto num_left = _config.max_popular_polls;
    for (auto poll_ref = _popular_polls.begin(); poll_ref != _popular_polls.end();)
    {
        updated_poll =
            updated_poll ||
            (poll_ref->slug == slug &&
             poll_ref->creator == creator);

        // get the poll object from the creator's polls table
        polls_table creator_polls(_self, poll_ref->creator);
        auto creator_poll = creator_polls.find(poll_ref->slug);

        // check if the poll exists
        if (creator_poll != creator_polls.end())
        {
            // update the poll reference
            _popular_polls.modify(poll_ref, voter, [&](auto &p) {
                p.popularity = creator_poll->calculate_popularity(_config.popularity_gravity);
                p.votes = votes;
            });

            // save the lowest popularity for later
            if (poll_ref->popularity < lowest_popularity)
            {
                lowest_popularity = poll_ref->popularity;
            }
            num_left -= 1;
            ++poll_ref;
        }
        else
        {
            // no poll found in the creator's polls table so erase the reference
            poll_ref = _popular_polls.erase(poll_ref);
        }
    }

    // check if we should put this poll in the popular poll's table
    const double poll_popularity = p->calculate_popularity(_config.popularity_gravity);
    const bool should_emplace =
        !updated_poll &&
        (num_left > 0 || poll_popularity > lowest_popularity);
    if (should_emplace)
    {
        _popular_polls.emplace(voter, [&](auto &pp) {
            pp.id = _popular_polls.available_primary_key();
            pp.creator = creator;
            pp.slug = slug;
            pp.title = p->title;
            pp.options = p->options;
            pp.min_num_choices = p->min_num_choices;
            pp.max_num_choices = p->max_num_choices;
            pp.whitelist = p->whitelist;
            pp.blacklist = p->blacklist;
            pp.create_time = p->create_time;
            pp.open_time = p->open_time;
            pp.close_time = p->close_time;
            pp.metadata = p->metadata;
            pp.votes = votes;
        });
    }

    prune_popular_polls();
}

void contract::prune_popular_polls()
{
    auto popularity_index = _popular_polls.get_index<N(popularity)>();
    auto num_left = _config.max_popular_polls;
    for (auto it = popularity_index.rbegin(); it != popularity_index.rend();)
    {
        if (num_left <= 0)
        {
            it = decltype(it){popularity_index.erase(std::next(it).base())};
        }
        else
        {
            num_left -= 1;
            ++it;
        }
    }
}

} // namespace eosstrawpoll
