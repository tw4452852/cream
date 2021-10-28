use field_derive::*;
use zoon::{Deserialize, Serialize};

mod fields;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    wrap: Mutable<Option<super::Enum>>,
    fields: Mutable<Option<fields::Enum>>,
}
