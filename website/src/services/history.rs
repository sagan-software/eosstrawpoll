use std::marker::PhantomData;
use stdweb::unstable::TryFrom;
use stdweb::web::event::PopStateEvent;
use stdweb::web::window;
use stdweb::web::EventListenerHandle;
use stdweb::web::History;
use stdweb::web::IEventTarget;
use stdweb::web::Location;
use stdweb::JsSerialize;
use stdweb::Value;
use yew::callback::Callback;

pub struct HistoryService<State> {
    history: History,
    event_listeners: Vec<EventListenerHandle>,
    phantom_data: PhantomData<State>,
}

impl<State> HistoryService<State>
where
    State: JsSerialize + Clone + TryFrom<Value> + 'static,
{
    pub fn new() -> HistoryService<State> {
        HistoryService {
            history: window().history(),
            event_listeners: vec![],
            phantom_data: PhantomData,
        }
    }

    pub fn add_listener(&mut self, callback: Callback<(Location, State)>) {
        let event_listener = window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();

            if let Ok(state) = State::try_from(state_value) {
                match window().location() {
                    Some(location) => callback.emit((location.clone(), state.clone())),
                    None => error!("browser does not support location API"),
                }
            } else {
                error!("Nothing farther back in history, not calling routing callback.");
            }
        });
        self.event_listeners.push(event_listener);
    }

    pub fn push_state(&mut self, route: &str, state: State) {
        self.history.push_state(state, "", Some(route));
    }

    pub fn location(&self) -> Option<Location> {
        window().location()
    }
}
