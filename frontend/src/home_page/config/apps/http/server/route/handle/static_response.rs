use field_derive::*;
use std::collections::BTreeMap;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    status_code: Mutable<Option<String>>,
    headers: Mutable<Option<BTreeMap<String, crate::MSVec>>>,
    body: Mutable<Option<String>>,
    close: Mutable<Option<bool>>,
    abort: Mutable<Option<bool>>,
}

fn status_code_default() -> String {
    Default::default()
}

fn body_default() -> String {
    Default::default()
}

fn close_default() -> bool {
    false
}

fn abort_default() -> bool {
    false
}
