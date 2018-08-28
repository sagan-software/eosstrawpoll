use services::history::HistoryService;
use std::collections::HashSet;
use stdweb::web::Location;
use yew::prelude::worker::*;
use yew::prelude::Callback;

#[derive(Serialize, Deserialize, Debug)]
pub enum RouterInput {
    ChangeRoute(String),
    ChangeRouteNoBroadcast(String),
    GetCurrentRoute,
}

impl Transferable for RouterInput {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterOutput {
    pub pathname: String,
    pub search: String,
    pub hash: String,
}
impl Transferable for RouterOutput {}

impl RouterOutput {
    pub fn from_location(location: &Location) -> RouterOutput {
        RouterOutput {
            pathname: location.pathname().unwrap_or_else(|_| "".into()),
            search: location.search().unwrap_or_else(|_| "".into()),
            hash: location.hash().unwrap_or_else(|_| "".into()),
        }
    }
}

pub enum RouterMsg {
    BrowserNavigationRouteChanged(Location),
}

pub struct RouterAgent {
    link: AgentLink<RouterAgent>,
    history_service: HistoryService,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<HandlerId>,
}

impl RouterAgent {
    fn output(&self) -> RouterOutput {
        let location = self.history_service.location().unwrap();
        RouterOutput::from_location(&location)
    }
}

impl Agent for RouterAgent {
    type Reach = Context;
    type Message = RouterMsg;
    type Input = RouterInput;
    type Output = RouterOutput;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|route_changed: Location| {
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
            RouterMsg::BrowserNavigationRouteChanged(location) => {
                info!("Browser navigated");
                let output = RouterOutput::from_location(&location);
                for sub in &self.subscribers {
                    self.link.response(*sub, output.clone());
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            RouterInput::ChangeRoute(url) => {
                self.history_service.push_state(&url);
                let output = self.output();
                for sub in &self.subscribers {
                    self.link.response(*sub, output.clone());
                }
            }
            RouterInput::ChangeRouteNoBroadcast(url) => {
                self.history_service.push_state(&url);
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

impl RouterAgent {
    pub fn redirect(url: String) {
        let callback = Callback::from(|_| ());
        let mut router = RouterAgent::bridge(callback);
        router.send(RouterInput::ChangeRoute(url));
    }
}
