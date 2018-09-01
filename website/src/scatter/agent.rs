use scatter::service::*;
use scatter::types::*;
use std::collections::HashSet;
use yew::prelude::worker::*;
use yew::prelude::Callback;

#[derive(Serialize, Deserialize, Debug)]
pub enum ScatterInput {
    Connect(String, u32),
    GetIdentity(ScatterRequiredFields),
    CurrentIdentity,
    ForgetIdentity,
    PushTransaction(ScatterNetwork, EosConfig, ScatterTransaction),
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

pub struct ScatterAgent {
    link: AgentLink<ScatterAgent>,
    scatter_service: Option<ScatterService>,
    subscribers: HashSet<HandlerId>,
    current_identity: Option<Result<ScatterIdentity, ScatterError>>,
}

impl Agent for ScatterAgent {
    type Reach = Context;
    type Message = ScatterMsg;
    type Input = ScatterInput;
    type Output = ScatterOutput;

    fn create(link: AgentLink<Self>) -> Self {
        ScatterAgent {
            link,
            scatter_service: None,
            subscribers: HashSet::new(),
            current_identity: None,
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            ScatterInput::Connect(appname, timeout) => {
                if self.scatter_service.is_some() {
                    self.link.response(who, ScatterOutput::Connected(Ok(())));
                    self.handle(ScatterInput::CurrentIdentity, who);
                } else {
                    let callback = self.link.send_back(ScatterMsg::Connected);
                    ScatterService::connect(appname, timeout, callback);
                }
            }
            ScatterInput::GetIdentity(required_fields) => match &self.scatter_service {
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
            ScatterInput::ForgetIdentity => match &self.scatter_service {
                Some(scatter_service) => {
                    let callback = self.link.send_back(ScatterMsg::ForgotIdentity);
                    scatter_service.forget_identity(callback);
                }
                None => {
                    let output = ScatterOutput::ForgotIdentity(Err(ScatterError::NotConnected));
                    self.link.response(who, output);
                }
            },
            ScatterInput::PushTransaction(network, config, transaction) => match &self
                .scatter_service
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
                    self.scatter_service = Some(scatter_service);
                    ScatterOutput::Connected(Ok(()))
                }
                Err(error) => {
                    self.scatter_service = None;
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
}
