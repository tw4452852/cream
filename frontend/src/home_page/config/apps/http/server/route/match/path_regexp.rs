use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    name: Mutable<Option<String>>,
    pattern: Mutable<Option<String>>,
}

fn name_default() -> String {
    Default::default()
}

fn pattern_default() -> String {
    Default::default()
}
