use super::service::*;
use super::types::*;
use eos::types::*;
use std::collections::HashSet;
use yew::prelude::worker::*;
use yew::prelude::Callback;

#[derive(Serialize, Deserialize, Debug)]
pub enum ScatterInput {
    Connect(String, u32),
    GetIdentity(ScatterRequiredFields),
    CurrentIdentity,
    ForgetIdentity,
    PushTransaction(ScatterNetwork, EosJsConfig, ScatterTransaction),
}

impl Transferable for ScatterInput {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScatterOutput {
    Connected(Result<(), ScatterError>),
    GotIdentity(Result<ScatterIdentity, ScatterError>),
    ForgotIdentity(Result<(), ScatterError>),
    PushedTransaction(Result<PushedTransaction, ScatterError>),
}

impl Transferable for ScatterOutput {}

pub enum ScatterMsg {
    Connected(Result<ScatterService, ScatterError>),
    GotIdentity(Result<ScatterIdentity, ScatterError>),
    ForgotIdentity(Result<(), ScatterError>),
    PushedTransaction(Result<PushedTransaction, ScatterError>),
}

pub enum ScatterStatus {
    Uninitialized,
    Connecting,
    Connected(ScatterService),
    NotConnected(ScatterError),
}

impl ScatterStatus {
    fn is_uninitialized(&self) -> bool {
        match self {
            ScatterStatus::Uninitialized => true,
            _ => false,
        }
    }
}

pub struct ScatterAgent {
    link: AgentLink<ScatterAgent>,
    subscribers: HashSet<HandlerId>,
    current_identity: Option<Result<ScatterIdentity, ScatterError>>,
    status: ScatterStatus,
}

impl Agent for ScatterAgent {
    type Reach = Context;
    type Message = ScatterMsg;
    type Input = ScatterInput;
    type Output = ScatterOutput;

    fn create(link: AgentLink<Self>) -> Self {
        ScatterAgent {
            link,
            subscribers: HashSet::new(),
            current_identity: None,
            status: ScatterStatus::Uninitialized,
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            ScatterInput::Connect(appname, timeout) => {
                if self.status.is_uninitialized() {
                    self.status = ScatterStatus::Connecting;
                    let callback = self.link.send_back(ScatterMsg::Connected);
                    ScatterService::connect(appname, timeout, callback);
                } else {
                    match self.status {
                        ScatterStatus::Connected(_) => {
                            self.link.response(who, ScatterOutput::Connected(Ok(())));
                            self.handle(ScatterInput::CurrentIdentity, who);
                        }
                        ScatterStatus::NotConnected(ref error) => {
                            self.link
                                .response(who, ScatterOutput::Connected(Err(error.clone())));
                        }
                        _ => {}
                    }
                }
            }
            ScatterInput::GetIdentity(required_fields) => match self.scatter_service() {
                Some(scatter_service) => {
                    let callback = self.link.send_back(ScatterMsg::GotIdentity);
                    scatter_service.get_identity(required_fields, callback);
                }
                None => {
                    let output = ScatterOutput::GotIdentity(Err(ScatterError::NotConnected));
                    self.link.response(who, output);
                }
            },
            ScatterInput::CurrentIdentity => {
                if let Some(result) = &self.current_identity {
                    let output = ScatterOutput::GotIdentity(result.clone());
                    self.link.response(who, output);
                }
            }
            ScatterInput::ForgetIdentity => match self.scatter_service() {
                Some(scatter_service) => {
                    let callback = self.link.send_back(ScatterMsg::ForgotIdentity);
                    scatter_service.forget_identity(callback);
                }
                None => {
                    let output = ScatterOutput::ForgotIdentity(Err(ScatterError::NotConnected));
                    self.link.response(who, output);
                }
            },
            ScatterInput::PushTransaction(network, config, transaction) => match self
                .scatter_service()
            {
                Some(scatter_service) => {
                    let callback = self.link.send_back(ScatterMsg::PushedTransaction);
                    scatter_service.push_transaction(network, config, transaction, callback);
                }
                None => {
                    let output = ScatterOutput::PushedTransaction(Err(ScatterError::NotConnected));
                    self.link.response(who, output);
                }
            },
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let output = match msg {
            ScatterMsg::Connected(result) => match result {
                Ok(scatter_service) => {
                    self.status = ScatterStatus::Connected(scatter_service);
                    ScatterOutput::Connected(Ok(()))
                }
                Err(error) => {
                    self.status = ScatterStatus::NotConnected(error.clone());
                    ScatterOutput::Connected(Err(error))
                }
            },
            ScatterMsg::GotIdentity(result) => {
                self.current_identity = Some(result.clone());
                ScatterOutput::GotIdentity(result)
            }
            ScatterMsg::ForgotIdentity(result) => {
                self.current_identity = None;
                ScatterOutput::ForgotIdentity(result)
            }
            ScatterMsg::PushedTransaction(result) => ScatterOutput::PushedTransaction(result),
        };
        for sub in &self.subscribers {
            self.link.response(*sub, output.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl ScatterAgent {
    pub fn new(
        appname: &str,
        timeout: u32,
        callback: Callback<ScatterOutput>,
    ) -> Box<Bridge<ScatterAgent>> {
        let mut scatter = ScatterAgent::bridge(callback);
        scatter.send(ScatterInput::Connect(appname.to_string(), timeout));
        scatter
    }

    fn scatter_service(&self) -> Option<&ScatterService> {
        match &self.status {
            ScatterStatus::Connected(scatter_service) => Some(scatter_service),
            _ => None,
        }
    }
}
