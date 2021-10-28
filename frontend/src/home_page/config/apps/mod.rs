use field_derive::*;
use zoon::{Deserialize, Serialize};

mod http;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    http: Mutable<Option<http::Val>>,
}
