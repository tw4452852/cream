use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    disabled: Mutable<Option<bool>>,
    listen: Mutable<Option<String>>,
    enforce_origin: Mutable<Option<bool>>,
    origins: Mutable<Option<Vec<String>>>,
}

fn disabled_default() -> bool {
    false
}

fn listen_default() -> String {
    "localhost:2019".to_owned()
}

fn enforce_origin_default() -> bool {
    false
}

fn origins_default() -> String {
    Default::default()
}
