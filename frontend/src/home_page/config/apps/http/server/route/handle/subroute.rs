use field_derive::*;
use zoon::{Deserialize, Serialize};

mod errors;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    routes: Mutable<Option<Vec<super::super::Val>>>,
    errors: Mutable<Option<errors::Val>>,
}
