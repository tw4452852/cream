use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    ipv4_cidr: Mutable<Option<String>>,
    ipv6_cidr: Mutable<Option<String>>,
}

fn ipv4_cidr_default() -> String {
    Default::default()
}

fn ipv6_cidr_default() -> String {
    Default::default()
}
