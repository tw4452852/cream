use field_derive::*;
use std::collections::BTreeMap;
use zoon::{Deserialize, Serialize};

pub mod encoder;
mod logs;
mod sink;
pub mod writer;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    sink: Mutable<Option<sink::Val>>,
    logs: Mutable<Option<BTreeMap<String, logs::Val>>>,
}
