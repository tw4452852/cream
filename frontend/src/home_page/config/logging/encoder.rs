use field_derive::*;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
use zoon::{Deserialize, Serialize};

mod console;
mod filter;
mod formatted;
mod json;
mod logfmt;
mod single_field;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Field, AsRefStr, EnumVariantNames, EnumString)]
#[serde(crate = "serde", tag = "format")]
#[allow(non_camel_case_types)]
pub enum Enum {
    console(console::Val),
    filter(filter::Val),
    json(json::Val),
    logfmt(logfmt::Val),
    single_field(single_field::Val),
    formatted(formatted::Val),
}

impl Default for Enum {
    fn default() -> Self {
        Self::console(Default::default())
    }
}
