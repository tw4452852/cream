use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    root: Mutable<Option<String>>,
    try_files: Mutable<Option<Vec<String>>>,
    try_policy: Mutable<Option<String>>,
    split_path: Mutable<Option<Vec<String>>>,
}

fn root_default() -> String {
    Default::default()
}

fn try_files_default() -> String {
    Default::default()
}

fn try_policy_default() -> String {
    Default::default()
}

fn split_path_default() -> String {
    Default::default()
}
