use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    field: Mutable<Option<String>>,
    fallback: Mutable<Option<super::Enum>>,
}

fn field_default() -> String {
    Default::default()
}
