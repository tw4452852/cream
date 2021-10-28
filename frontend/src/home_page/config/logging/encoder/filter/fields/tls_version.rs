use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    prefix: Mutable<Option<String>>,
}

fn prefix_default() -> String {
    Default::default()
}
