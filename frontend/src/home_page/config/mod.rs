use field_derive::*;
use zoon::{Deserialize, Serialize};

mod admin;
mod apps;
mod logging;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    apps: Mutable<Option<apps::Val>>,
    logging: Mutable<Option<logging::Val>>,
    admin: Mutable<Option<admin::Val>>,
}
