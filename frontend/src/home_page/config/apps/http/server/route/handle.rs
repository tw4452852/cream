use field_derive::*;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
use zoon::{Deserialize, Serialize};

mod error;
mod exec;
mod rate_limit;
mod static_response;
mod subroute;
mod webhook;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Field, AsRefStr, EnumVariantNames, EnumString)]
#[serde(crate = "serde", tag = "handler")]
#[allow(non_camel_case_types)]
pub enum Enum {
    rate_limit(rate_limit::Val),
    webhook(webhook::Val),
    exec(exec::Val),
    subroute(subroute::Val),
    error(error::Val),
    static_response(static_response::Val),
}
