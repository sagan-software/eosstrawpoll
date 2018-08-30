pub type ChainId = String;

pub type Name = String;

pub type AccountName = Name;

pub type TableName = Name;

pub type ActionName = Name;

pub type Symbol = Name;

pub type PermissionName = Name;

pub type ScopeName = Name;

pub type TypeName = Name;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

pub type PublicKey = String;

pub type PrivateKey = String;

pub type Signature = String;

pub type TransactionId = String;

pub type Asset = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    pub actor: AccountName,
    pub permission: PermissionName,
}

impl Authorization {
    pub fn active(actor: AccountName) -> Authorization {
        Authorization {
            actor,
            permission: "active".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action<Data> {
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<Authorization>,
    pub data: Data,
}
