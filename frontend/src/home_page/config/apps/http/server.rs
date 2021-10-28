use field_derive::*;
use zoon::{Deserialize, Serialize};

mod route;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    listen: Mutable<Option<Vec<String>>>,
    read_timeout: Mutable<Option<u32>>,
    read_header_timeout: Mutable<Option<u32>>,
    write_timeout: Mutable<Option<u32>>,
    idle_timeout: Mutable<Option<u32>>,
    max_header_bytes: Mutable<Option<u32>>,
    routes: Mutable<Option<Vec<route::Val>>>,
}

fn listen_default() -> String {
    ":12345".to_owned()
}

fn read_timeout_default() -> u32 {
    0
}

fn read_header_timeout_default() -> u32 {
    0
}

fn write_timeout_default() -> u32 {
    0
}

fn idle_timeout_default() -> u32 {
    0
}

fn max_header_bytes_default() -> u32 {
    0
}
