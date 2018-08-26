use serde::{Deserialize, Deserializer, Serializer};

pub fn bool_from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u8 = Deserialize::deserialize(deserializer)?;
    Ok(s == 1)
}

pub fn bool_to_u8<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let num = if *x { 1 } else { 0 };
    s.serialize_u8(num)
}
