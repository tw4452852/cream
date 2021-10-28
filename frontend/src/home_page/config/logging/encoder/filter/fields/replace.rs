use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    value: Mutable<Option<String>>,
}

fn value_default() -> String {
    Default::default()
}
