use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    db_path: Mutable<Option<String>>,
    allow_countries: Mutable<Option<Vec<String>>>,
    deny_countries: Mutable<Option<Vec<String>>>,
    allow_subdivisions: Mutable<Option<Vec<String>>>,
    deny_subdivisions: Mutable<Option<Vec<String>>>,
    allow_metro_codes: Mutable<Option<Vec<String>>>,
    deny_metro_codes: Mutable<Option<Vec<String>>>,
}

fn db_path_default() -> String {
    Default::default()
}

fn allow_countries_default() -> String {
    Default::default()
}

fn deny_countries_default() -> String {
    Default::default()
}

fn allow_subdivisions_default() -> String {
    Default::default()
}

fn deny_subdivisions_default() -> String {
    Default::default()
}

fn allow_metro_codes_default() -> String {
    Default::default()
}

fn deny_metro_codes_default() -> String {
    Default::default()
}
