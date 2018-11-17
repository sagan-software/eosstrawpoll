use contract::PollId;
use crate::eos::types::*;
use crate::types::*;
use eosio::AccountName;
use std::str::FromStr;
use stdweb::web::error::SecurityError;
use stdweb::web::Location;

#[derive(Clone, PartialEq)]
pub enum Route {
    Home(Option<ChainIdPrefix>),
    Profile(ChainIdPrefix, AccountName),
    PollVoting(ChainIdPrefix, PollId),
    PollResults(ChainIdPrefix, PollId),
}

impl Default for Route {
    fn default() -> Route {
        Route::Home(None)
    }
}

pub enum RouteError {
    SecurityError(SecurityError),
    NotFound(String),
}

impl Route {
    pub fn from_location(location: &Location) -> Result<Route, RouteError> {
        match location.pathname() {
            Ok(pathname) => Route::from_str(pathname.as_str()),
            Err(error) => Err(RouteError::SecurityError(error)),
        }
    }

    fn from_strings(pathnames: &[String]) -> Result<Route, RouteError> {
        let strs: Vec<&str> = pathnames.iter().map(|s| s.as_str()).collect();
        match &strs[..] {
            [""] => Ok(Route::Home(None)),
            [chain_id_prefix, ""] => Ok(Route::Home(Some(chain_id_prefix.to_string().into()))),
            [chain_id_prefix, "u", account] => AccountName::from_string(account.to_string())
                .map(|account| Route::Profile(chain_id_prefix.to_string().into(), account))
                .map_err(|_| RouteError::NotFound("invalid account name".to_string())),
            [chain_id_prefix, "v", poll_id] => PollId::from_string(poll_id.to_string())
                .map(|poll_id| Route::PollVoting(chain_id_prefix.to_string().into(), poll_id))
                .map_err(|_| RouteError::NotFound("invalid poll id".to_string())),
            [chain_id_prefix, "r", poll_id] => PollId::from_string(poll_id.to_string())
                .map(|poll_id| Route::PollResults(chain_id_prefix.to_string().into(), poll_id))
                .map_err(|_| RouteError::NotFound("invalid poll id".to_string())),
            _ => Err(RouteError::NotFound(format!("/{}", pathnames.join("/")))),
        }
    }

    pub fn to_absolute(&self) -> String {
        // TODO: use localhost in development environment
        format!("https://www.eosstrawpoll.com{}", self.to_string())
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::Home(chain_id_prefix) => match chain_id_prefix {
                Some(chain_id_prefix) => format!("/{}/", chain_id_prefix.to_string()),
                None => "/".into(),
            },
            Route::Profile(chain_id_prefix, account) => {
                format!("/{}/u/{}", chain_id_prefix.to_string(), account)
            }
            Route::PollVoting(chain_id_prefix, poll_id) => {
                format!("/{}/v/{}", chain_id_prefix.to_string(), poll_id)
            }
            Route::PollResults(chain_id_prefix, poll_id) => {
                format!("/{}/r/{}", chain_id_prefix.to_string(), poll_id)
            }
        }
    }
}

impl FromStr for Route {
    type Err = RouteError;
    fn from_str(s: &str) -> Result<Route, Self::Err> {
        let mut pathnames: Vec<String> = s.split('/').map(String::from).collect();
        pathnames.remove(0); // remove empty string that is split from the first '/'
        Route::from_strings(&pathnames)
    }
}
