use std::str::FromStr;
use stdweb::web::error::SecurityError;
use stdweb::web::Location;

#[derive(Clone)]
pub enum Route {
    Home,
    PopularPolls,
    NewPolls,
    Donors,
    Profile(String),
    Poll(String, String),
    PollResults(String, String),
}

pub enum RouteError {
    SecurityError(SecurityError),
    NotFound(String),
}

impl Route {
    pub fn to_string(&self) -> String {
        match self {
            Route::Home => "/".into(),
            Route::PopularPolls => "/-/popular".into(),
            Route::NewPolls => "/-/new".into(),
            Route::Donors => "/-/donors".into(),
            Route::Profile(account) => format!("/{}", account),
            Route::Poll(creator, slug) => format!("/{}/{}", creator, slug),
            Route::PollResults(creator, slug) => format!("/{}/{}/results", creator, slug),
        }
    }

    pub fn from_location(location: &Location) -> Result<Route, RouteError> {
        match location.pathname() {
            Ok(pathname) => Route::from_str(pathname.as_str()),
            Err(error) => Err(RouteError::SecurityError(error)),
        }
    }

    fn from_strings(pathnames: &[String]) -> Result<Route, RouteError> {
        let strs: Vec<&str> = pathnames.iter().map(|s| s.as_str()).collect();
        match &strs[..] {
            [""] => Ok(Route::Home),
            ["-", path] => match *path {
                "popular" => Ok(Route::PopularPolls),
                "new" => Ok(Route::NewPolls),
                "donors" => Ok(Route::Donors),
                _ => Err(RouteError::NotFound(format!("/{}", pathnames.join("/")))),
            },
            [account] => Ok(Route::Profile(account.to_string())),
            [creator, slug] => Ok(Route::Poll(creator.to_string(), slug.to_string())),
            [creator, slug, "results"] => {
                Ok(Route::PollResults(creator.to_string(), slug.to_string()))
            }
            _ => Err(RouteError::NotFound(format!("/{}", pathnames.join("/")))),
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
