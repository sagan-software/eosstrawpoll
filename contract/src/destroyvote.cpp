#include <eosstrawpoll/contract.hpp>

namespace eosstrawpoll
{

void contract::destroyvote(
    const account_name creator,
    const poll_name slug,
    const account_name voter,
    const string &metadata){};

} // namespace eosstrawpoll
