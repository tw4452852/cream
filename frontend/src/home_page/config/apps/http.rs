use field_derive::*;
use std::collections::BTreeMap;
use zoon::{Deserialize, Serialize};

mod server;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    http_port: Mutable<Option<u32>>,
    https_port: Mutable<Option<u32>>,
    grace_period: Mutable<Option<String>>,
    servers: Mutable<Option<BTreeMap<String, server::Val>>>,
}

fn http_port_default() -> u32 {
    80
}

fn https_port_default() -> u32 {
    443
}

fn grace_period_default() -> String {
    "0s".to_owned()
}
