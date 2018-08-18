use agents::scatter::{self, ScatterAgent, ScatterError, ScatterInput, ScatterOutput};
use context::Context;
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
    SetAmount(f32),
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
                if amount > 0.0001 {
                    self.amount = amount;
                    true
                } else {
                    false
                }
            }
            Msg::Submit => {
                info!("submitting form");
                self.submitting = true;
                true
            }
            Msg::Scatter(output) => match output {
                ScatterOutput::GotIdentity(result) => {
                    info!("got identity {:#?}", result);
                    self.scatter_identity = Some(result);
                    if self.submitting {
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
                    self.pushed = Some(result);
                    true
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
                <p>
                    { "Want to support EOS Straw Poll? Please consider donating some EOS to fund development for new features and enhancements. Thank you!" }
                </p>
                <div class="donation_input", >
                    <input placeholder="1.0000 EOS", />
                    <button type="submit", >
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
