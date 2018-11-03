use contract::PollName;
use eosio::AccountName;

#[derive(Clone, PartialEq)]
pub enum Route {
    Home,
    Settings,
    Poll(PollName),
}

impl Default for Route {
    fn default() -> Route {
        Route::Home
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::Home => "/".into(),
            Route::Settings => "/settings".into(),
            Route::Poll(poll_name) => format!("/{}", poll_name.to_string()),
        }
    }
}

impl From<Route> for String {
    fn from(r: Route) -> String {
        r.to_string()
    }
}
