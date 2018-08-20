use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
use serde_json;
use stdweb::traits::IEvent;
use types::TransferAction;
use yew::prelude::*;

pub struct DonationForm {
    amount: f32,
    submitting: bool,
    context: Context,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pushed: Option<Result<scatter::PushedTransaction, ScatterError>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

pub enum Msg {
    Submit,
    Scatter(ScatterOutput),
    SetAmount(String),
}

impl Component for DonationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Scatter);
        let mut scatter_agent = ScatterAgent::bridge(callback);
        scatter_agent.send(ScatterInput::Connect("eosstrawpoll".into(), 10000));
        DonationForm {
            amount: 0.0,
            submitting: false,
            context: props.context,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetAmount(amount) => {
                let amount = amount.parse::<f32>();
                match amount {
                    Ok(amount) => {
                        if amount > 0.0001 {
                            self.amount = amount;
                            true
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                }
            }
            Msg::Submit => {
                info!("submitting form");
                self.submitting = true;

                let donor = match self.donor() {
                    Some(donor) => donor,
                    None => {
                        let required_fields = self.context.required_fields();
                        let scatter_input = ScatterInput::GetIdentity(required_fields);
                        self.scatter_agent.send(scatter_input);
                        return true;
                    }
                };

                let network = self.context.network();
                let config = self.context.eos_config();

                let action_data = TransferAction {
                    from: donor.to_string(),
                    to: "eosstrawpoll".to_string(),
                    quantity: "1.0000 SYS".to_string(),
                    memo: "Funded EOS Straw Poll".to_string(),
                };

                let data = serde_json::to_value(action_data).unwrap();

                let action = scatter::Action {
                    account: "eosio.token".into(),
                    name: "transfer".into(),
                    authorization: vec![scatter::Authorization {
                        actor: donor.to_string(),
                        permission: "active".into(),
                    }],
                    data,
                };

                let actions = vec![action];

                self.scatter_agent
                    .send(ScatterInput::PushActions(network, config, actions));

                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    let is_ok = result.is_ok();
                    self.scatter_identity = Some(result);
                    if !is_ok && self.submitting {
                        self.submitting = false;
                    }
                    if is_ok && self.submitting {
                        self.update(Msg::Submit)
                    } else {
                        true
                    }
                }
                ScatterOutput::ForgotIdentity(_result) => {
                    self.scatter_identity = None;
                    true
                }
                ScatterOutput::Connected(result) => {
                    if result.is_ok() {
                        self.scatter_agent.send(ScatterInput::CurrentIdentity);
                    }
                    self.scatter_connected = Some(result);
                    true
                }
                ScatterOutput::PushedActions(result) => {
                    if self.submitting {
                        self.pushed = Some(result);
                        self.submitting = false;
                        true
                    } else {
                        false
                    }
                }
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<DonationForm> for DonationForm {
    fn view(&self) -> Html<Self> {
        html! {
            <form class="donation_form", >
                <h2>{ "Support Development" }</h2>
                <p>
                    { "Please consider donating EOS to help cover development costs. Thank you!" }
                </p>
                <div class="donation_input", >
                    <input
                        placeholder="1.0000 EOS",
                        oninput=|e| Msg::SetAmount(e.value),
                    />
                    <button type="submit",
                        disabled=self.submitting,
                        onclick=|e| {
                            e.prevent_default();
                            Msg::Submit
                        },
                    >
                        { "Donate" }
                    </button>
                </div>
            </form>
        }
    }
}

impl DonationForm {
    fn donor(&self) -> Option<String> {
        let result = match &self.scatter_identity {
            Some(result) => result,
            None => return None,
        };

        let identity = match result {
            Ok(identity) => identity,
            Err(_error) => return None,
        };

        match identity.accounts.first() {
            Some(ref account) => Some(account.name.clone()),
            None => None,
        }
    }
}
