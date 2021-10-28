use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    error: Mutable<Option<String>>,
    status_code: Mutable<Option<String>>,
}

fn error_default() -> String {
    Default::default()
}

fn status_code_default() -> String {
    "500".to_owned()
}
