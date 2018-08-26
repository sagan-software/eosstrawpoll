#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::createvote(
    const account_name creator,
    const poll_name slug,
    const account_name voter,
    const vector<choice> &choices)
{
    require_auth(voter);

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
    eosio_assert(num_choices >= p->min_choices, "too few choices");
    eosio_assert(num_choices <= p->max_choices, "too many choices");

    // check if choices are valid
    const auto max_option_index = p->options.size() - 1;
    std::map<uint8_t, bool> seen_option_indexes;
    std::map<string, bool> seen_writeins;
    for (auto &c : choices)
    {
        const auto option_index = c.option_index;
        const auto writein = c.writein;
        const bool has_option_index = option_index >= 0;
        const bool has_writein = !writein.empty();
        eosio_assert(
            (has_option_index && !has_writein) || (!has_option_index && has_writein),
            "invalid choice: must have either an option index OR a writein answer, not neither or both. set option index to -1 or writein to an empty string");
        eosio_assert(option_index >= -1, "invalid choice: option index cannot be less than -1");

        if (has_option_index)
        {
            eosio_assert(option_index <= max_option_index, "invalid choice: option index cannot be greater than the total number of options");
            eosio_assert(seen_option_indexes.count(option_index) == 0, "invalid choices: duplicate option indexes are not allowed");
            seen_option_indexes[option_index] = true;
        }

        if (has_writein)
        {
            eosio_assert(writein.size() <= _config.max_writein_len, "invalid choice: writein length is too long");
            eosio_assert(seen_writeins.count(writein) == 0, "invalid choices: duplicate writeins are not allowed");
            // TODO: check that writein is not all whitespace characters
            seen_writeins[writein] = true;
            eosio_assert(seen_writeins.size() <= p->max_writeins, "invalid choices: too many writeins");
        }
    }

    // check account list
    const auto al = p->account_list;
    if (!al.empty())
    {
        const bool voter_is_on_list = std::find(al.begin(), al.end(), voter) != al.end();
        if (p->use_allow_list)
        {
            eosio_assert(voter_is_on_list, "voter is not on the allow list");
        }
        else
        {
            eosio_assert(!voter_is_on_list, "voter is on the deny list");
        }
    }

    if (p->min_staked > 0)
    {
        // TODO check staked amount
    }

    if (p->min_value > 0)
    {
        // TODO check account value
    }

    // create vote
    vote v;
    v.voter = voter;
    v.choices = choices;
    v.created = now();

    vector<vote> votes = p->votes;

    // TODO make sure votes have changed
    // eosio_assert(current_vote.choices != choices, "choices have not changed");

    // cast vote
    _creator_polls.modify(p, voter, [&](auto &p) {
        // check if voter has voted already
        for (size_t i = 0; i < p.votes.size(); i++)
        {
            const vote current_vote = p.votes[i];
            if (current_vote.voter == voter)
            {
                p.votes[i] = v;
                p.popularity = p.calculate_popularity(_config.popularity_gravity);
                votes = p.votes;
                return;
            }
        }

        // new voter, add to end of vector
        p.votes.push_back(v);
        p.popularity = p.calculate_popularity(_config.popularity_gravity);
        votes = p.votes;
    });

    ensure_user(creator);

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
                p.votes = creator_poll->votes;
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
            pp.min_choices = p->min_choices;
            pp.max_choices = p->max_choices;
            pp.max_writeins = p->max_writeins;
            pp.use_allow_list = p->use_allow_list;
            pp.account_list = p->account_list;
            pp.min_staked = p->min_staked;
            pp.min_value = p->min_value;
            pp.open_time = p->open_time;
            pp.close_time = p->close_time;
            pp.create_time = p->create_time;
            pp.votes = votes;
            pp.popularity = poll_popularity;
        });
    }

    prune_popular_polls();

    // TODO: update new polls table
}

} // namespace eosstrawpoll
