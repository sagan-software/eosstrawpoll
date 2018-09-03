use eos::types::*;
use serde::{Deserialize, Serialize};
use serde_json;
// use services::scatter::{Authorization, ScatterAction};
use std::fmt::Debug;
use types::Chain;
use yew::prelude::{Component, Html};

pub trait ToAction
where
    Self: Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn to_action(&self, chain: &Chain) -> Action<Self>;
}

// impl<T> From<T> for ScatterAction
// where
//     T: Action,
// {
//     fn from(action: T) -> ScatterAction {
//         let data = serde_json::to_value(&action).unwrap();

//         let auth = Authorization {
//             actor: action.actor(),
//             permission: "active".into(),
//         };

//         ScatterAction {
//             account: action.code(),
//             name: action.name(),
//             authorization: vec![auth],
//             data,
//         }
//     }
// }

pub enum PageState {
    Loading,
    Error,
    Loaded,
}

pub trait Page: Component {
    fn title(&self) -> String;
    fn class(&self) -> String;
    fn content(&self) -> Html<Self>;
    fn get_state(&self) -> PageState;
}