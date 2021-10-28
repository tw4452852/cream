use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    ranges: Mutable<Option<Vec<String>>>,
    forwarded: Mutable<Option<bool>>,
}

fn ranges_default() -> String {
    Default::default()
}

fn forwarded_default() -> bool {
    false
}
