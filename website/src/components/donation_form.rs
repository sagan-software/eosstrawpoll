use agents::api::*;
use agents::scatter::{
    self, ScatterAction, ScatterAgent, ScatterError, ScatterInput, ScatterOutput,
};
use components::*;
use context::Context;
use stdweb::traits::IEvent;
use types::Transfer;
use yew::prelude::*;

pub struct DonationForm {
    amount: f32,
    submitting: bool,
    context: Context,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<scatter::Identity, ScatterError>>,
    pushed: Option<Result<scatter::PushedTransaction, ScatterError>>,
    api: Box<Bridge<ApiAgent>>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
}

pub enum Msg {
    Submit,
    Scatter(ScatterOutput),
    SetAmount(String),
    Api(ApiOutput),
}

impl Component for DonationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let api_config = props.context.api_config();
        let api = ApiAgent::new(api_config, link.send_back(Msg::Api));
        let scatter_agent = ScatterAgent::new("eosstrawpoll", 10000, link.send_back(Msg::Scatter));
        DonationForm {
            amount: 0.,
            submitting: false,
            context: props.context,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            api,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Api(_output) => false,
            Msg::SetAmount(amount) => {
                let amount = amount.parse::<f32>();
                match amount {
                    Ok(amount) => {
                        if amount > 0.0001 {
                            self.amount = amount;
                            true
                        } else {
                            true
                        }
                    }
                    Err(_) => true,
                }
            }
            Msg::Submit => {
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

                let amount = if self.amount == 0. { 1. } else { self.amount };
                let network = self.context.network();
                let config = self.context.eos_config();
                let action: ScatterAction = Transfer {
                    from: donor.to_string(),
                    to: "eosstrawpoll".to_string(),
                    quantity: format!("{:.4} SYS", amount),
                    memo: "Funded EOS Straw Poll".to_string(),
                }.into();

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
                    self.scatter_connected = Some(result);
                    true
                }
                ScatterOutput::PushedActions(result) => {
                    if self.submitting {
                        self.pushed = Some(result);
                        self.submitting = false;
                        self.amount = 0.;
                        self.api.send(ApiInput::GetDonors);
                        self.api.send(ApiInput::GetNewDonations);
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
            <form class="donation_form",
                onsubmit=|e| {
                    e.prevent_default();
                    Msg::Submit
                },
            >
                <h2>{ "Support Development" }</h2>
                <p>
                    { "Donations help cover development costs and fund new features. Thank you!" }
                </p>
                <div class="donation_input", >
                    <input
                        placeholder="1.0000",
                        oninput=|e| Msg::SetAmount(e.value),
                        disabled=self.submitting,
                        value=(if self.amount == 0. { "".to_string() } else { format!("{}", self.amount) }),
                    />
                    <Button:
                        size=Size::Small,
                        type_="submit",
                        disabled=self.submitting,
                        text="Donate".to_string(),
                    />
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
