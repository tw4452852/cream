use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    key: Mutable<Option<String>>,
    rate: Mutable<Option<String>>,
    zone_site: Mutable<Option<u32>>,
    reject_status: Mutable<Option<u32>>,
}

fn key_default() -> String {
    Default::default()
}

fn rate_default() -> String {
    Default::default()
}

fn zone_site_default() -> u32 {
    0
}

fn reject_status_default() -> u32 {
    0
}
