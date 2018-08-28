use stdweb::web::event::PopStateEvent;
use stdweb::web::window;
use stdweb::web::EventListenerHandle;
use stdweb::web::History;
use stdweb::web::IEventTarget;
use stdweb::web::Location;
use yew::callback::Callback;

pub struct HistoryService {
    history: History,
    event_listeners: Vec<EventListenerHandle>,
}

impl HistoryService {
    pub fn new() -> HistoryService {
        HistoryService {
            history: window().history(),
            event_listeners: vec![],
        }
    }

    pub fn add_listener(&mut self, callback: Callback<Location>) {
        let event_listener =
            window().add_event_listener(move |_e: PopStateEvent| match window().location() {
                Some(location) => callback.emit(location.clone()),
                None => error!("browser does not support location API"),
            });
        self.event_listeners.push(event_listener);
    }

    pub fn push_state(&mut self, route: &str) {
        self.history.push_state((), "", Some(route));
    }

    pub fn location(&self) -> Option<Location> {
        window().location()
    }
}
