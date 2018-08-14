//! Agent that exposes a usable routing interface to components.

use serde::Deserialize;
use serde::Serialize;
use services::history::HistoryService;
use std::collections::HashSet;
use std::fmt::Debug;
use stdweb::unstable::TryFrom;
use stdweb::web::Location;
use stdweb::JsSerialize;
use stdweb::Value;
use yew::prelude::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum RouterInput<State> {
    ChangeRoute(String, State),
    ChangeRouteNoBroadcast(String, State),
    GetCurrentRoute,
}

impl<State> Transferable for RouterInput<State> where for<'de> State: Serialize + Deserialize<'de> {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterOutput<State> {
    pub pathname: String,
    pub search: String,
    pub hash: String,
    pub state: State,
}
impl<State> Transferable for RouterOutput<State> where for<'de> State: Serialize + Deserialize<'de> {}

impl<State> RouterOutput<State>
where
    State: Default,
{
    pub fn from_location(location: &Location) -> RouterOutput<State> {
        RouterOutput {
            pathname: location.pathname().unwrap_or_else(|_| "".into()),
            search: location.search().unwrap_or_else(|_| "".into()),
            hash: location.hash().unwrap_or_else(|_| "".into()),
            state: State::default(),
        }
    }
}

pub enum RouterMsg<State>
where
    State: JsSerialize + Clone + Debug + TryFrom<Value> + 'static,
{
    BrowserNavigationRouteChanged((Location, State)),
}

pub struct RouterAgent<State>
where
    for<'de> State: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    link: AgentLink<RouterAgent<State>>,
    history_service: HistoryService<State>,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<HandlerId>,
}

impl<State> RouterAgent<State>
where
    for<'de> State: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    fn output(&self) -> RouterOutput<State> {
        let location = self.history_service.location().unwrap();
        RouterOutput::from_location(&location)
    }
}

impl<State> Agent for RouterAgent<State>
where
    for<'de> State: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    type Reach = Context;
    type Message = RouterMsg<State>;
    type Input = RouterInput<State>;
    type Output = RouterOutput<State>;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|route_changed: (Location, State)| {
            RouterMsg::BrowserNavigationRouteChanged(route_changed)
        });
        let mut history_service = HistoryService::new();
        history_service.add_listener(callback);

        RouterAgent {
            link,
            history_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            RouterMsg::BrowserNavigationRouteChanged((location, state)) => {
                info!("Browser navigated");
                let mut output = RouterOutput::from_location(&location);
                output.state = state;
                for sub in &self.subscribers {
                    self.link.response(*sub, output.clone());
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            RouterInput::ChangeRoute(url, state) => {
                self.history_service.push_state(&url, state.clone());
                let mut output = self.output();
                output.state = state;
                for sub in &self.subscribers {
                    self.link.response(*sub, output.clone());
                }
            }
            RouterInput::ChangeRouteNoBroadcast(url, state) => {
                self.history_service.push_state(&url, state);
            }
            RouterInput::GetCurrentRoute => {
                self.link.response(who, self.output());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
