use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    interval: Mutable<Option<u32>>,
    first: Mutable<Option<u32>>,
    thereafter: Mutable<Option<u32>>,
}

fn interval_default() -> u32 {
    0
}

fn first_default() -> u32 {
    0
}

fn thereafter_default() -> u32 {
    0
}
