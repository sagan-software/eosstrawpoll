use std::str::FromStr;
use stdweb::web::error::SecurityError;
use stdweb::web::Location;
use types::*;

#[derive(Clone, PartialEq)]
pub enum Route {
    Home(Option<ChainIdPrefix>),
    Profile(ChainIdPrefix, AccountName),
    Poll(ChainIdPrefix, AccountName, PollName),
    PollResults(ChainIdPrefix, AccountName, PollName),
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
            [chain_id_prefix] => Ok(Route::Home(Some(chain_id_prefix.to_string().into()))),
            [chain_id_prefix, account] => Ok(Route::Profile(
                chain_id_prefix.to_string().into(),
                account.to_string(),
            )),
            [chain_id_prefix, creator, slug] => Ok(Route::Poll(
                chain_id_prefix.to_string().into(),
                creator.to_string(),
                slug.to_string(),
            )),
            [chain_id_prefix, creator, slug, "results"] => Ok(Route::PollResults(
                chain_id_prefix.to_string().into(),
                creator.to_string(),
                slug.to_string(),
            )),
            _ => Err(RouteError::NotFound(format!("/{}", pathnames.join("/")))),
        }
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
                format!("/{}/{}", chain_id_prefix.to_string(), account)
            }
            Route::Poll(chain_id_prefix, creator, slug) => {
                format!("/{}/{}/{}", chain_id_prefix.to_string(), creator, slug)
            }
            Route::PollResults(chain_id_prefix, creator, slug) => format!(
                "/{}/{}/{}/results",
                chain_id_prefix.to_string(),
                creator,
                slug
            ),
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
