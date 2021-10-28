use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    command: Mutable<Option<String>>,
    args: Mutable<Option<Vec<String>>>,
    directory: Mutable<Option<String>>,
    foreground: Mutable<Option<bool>>,
    timeout: Mutable<Option<String>>,
    at: Mutable<Option<Vec<String>>>,
}

fn command_default() -> String {
    Default::default()
}

fn args_default() -> String {
    Default::default()
}

fn directory_default() -> String {
    Default::default()
}

fn foreground_default() -> bool {
    false
}

fn timeout_default() -> String {
    Default::default()
}

fn at_default() -> String {
    Default::default()
}
