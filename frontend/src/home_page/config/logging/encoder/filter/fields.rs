use field_derive::*;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
use zoon::{Deserialize, Serialize};

mod delete;
mod ip_mask;
mod replace;
mod tls_cipher;
mod tls_version;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Field, AsRefStr, EnumVariantNames, EnumString)]
#[serde(crate = "serde", tag = "filter")]
#[allow(non_camel_case_types)]
pub enum Enum {
    delete(delete::Val),
    ip_mask(ip_mask::Val),
    replace(replace::Val),
    tls_cipher(tls_cipher::Val),
    tls_version(tls_version::Val),
}
