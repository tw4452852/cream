use field_derive::*;

use zoon::{Deserialize, Serialize};

mod sampling;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    writer: Mutable<Option<crate::home_page::config::logging::writer::Enum>>,
    encoder: Mutable<Option<crate::home_page::config::logging::encoder::Enum>>,
    level: Mutable<Option<String>>,
    sampling: Mutable<Option<sampling::Val>>,
    include: Mutable<Option<Vec<String>>>,
    exclude: Mutable<Option<Vec<String>>>,
}

fn level_default() -> String {
    "INFO".to_owned()
}

fn include_default() -> String {
    Default::default()
}

fn exclude_default() -> String {
    Default::default()
}
