use crate::components::*;
use crate::eos::*;
use crate::prelude::*;
use crate::scatter::*;
use eosio::AccountName;
use stdweb::traits::IEvent;

pub struct DonationForm {
    amount: f32,
    submitting: bool,
    context: Context,
    scatter_agent: Box<Bridge<ScatterAgent>>,
    scatter_connected: Option<Result<(), ScatterError>>,
    scatter_identity: Option<Result<ScatterIdentity, ScatterError>>,
    pushed: Option<Result<PushedTransaction, ScatterError>>,
    eos_agent: Box<Bridge<EosAgent>>,
    chain: Chain,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub context: Context,
    pub chain: Chain,
}

pub enum Msg {
    Submit,
    Scatter(ScatterOutput),
    SetAmount(String),
    Chain(EosOutput),
}

impl Component for DonationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let eos_agent = EosAgent::bridge(link.send_back(Msg::Chain));
        let scatter_agent = ScatterAgent::new("eosstrawpoll", 10000, link.send_back(Msg::Scatter));
        DonationForm {
            amount: 0.,
            submitting: false,
            context: props.context,
            scatter_agent,
            scatter_connected: None,
            scatter_identity: None,
            pushed: None,
            eos_agent,
            chain: props.chain,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Chain(_output) => false,
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
                        let required_fields = self.chain.to_scatter_required_fields();
                        let scatter_input = ScatterInput::GetIdentity(required_fields);
                        self.scatter_agent.send(scatter_input);
                        return true;
                    }
                };

                let amount = if self.amount == 0. { 1. } else { self.amount };
                let network = self.chain.to_scatter_network();
                let config = self.chain.to_eos_config();
                let action = Transfer {
                    from: donor,
                    to: self.chain.code_account.clone(),
                    quantity: format!(
                        "{:.4} {}",
                        amount,
                        self.chain.core_symbol.name().to_string()
                    ),
                    memo: "Funded EOS Straw Poll".to_string(),
                }
                .to_action(&self.chain);

                let transaction: ScatterTransaction = action.into();
                self.scatter_agent.send(ScatterInput::PushTransaction(
                    network,
                    config,
                    transaction,
                ));

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
                ScatterOutput::PushedTransaction(result) => {
                    if self.submitting {
                        self.pushed = Some(result);
                        self.submitting = false;
                        self.amount = 0.;
                        self.eos_agent.send(EosInput::GetDonors);
                        self.eos_agent.send(EosInput::GetNewDonations);
                        true
                    } else {
                        false
                    }
                }
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        self.chain = props.chain;
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
    fn donor(&self) -> Option<AccountName> {
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
