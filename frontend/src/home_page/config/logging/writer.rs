use field_derive::*;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
use zoon::{Deserialize, Serialize};

mod discard;
mod file;
mod net;
mod stderr;
mod stdout;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Field, AsRefStr, EnumVariantNames, EnumString)]
#[serde(crate = "serde", tag = "output")]
#[allow(non_camel_case_types)]
pub enum Enum {
    discard(discard::Val),
    stderr(stderr::Val),
    stdout(stdout::Val),
    file(file::Val),
    net(net::Val),
}
