use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    address: Mutable<Option<String>>,
    dial_timeout: Mutable<Option<String>>,
}

fn address_default() -> String {
    ":12345".to_owned()
}

fn dial_timeout_default() -> String {
    "0s".to_owned()
}

impl Default for Val {
    fn default() -> Self {
        Self {
            address: Mutable::new(Some(address_default())),
            dial_timeout: Mutable::new(None),
        }
    }
}
