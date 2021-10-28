use field_derive::*;
use zoon::{Deserialize, Serialize};

mod handle;
mod r#match;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    group: Mutable<Option<String>>,
    handle: Mutable<Option<Vec<handle::Enum>>>,
    r#match: Mutable<Option<Vec<r#match::Val>>>,
    terminal: Mutable<Option<bool>>,
}

fn group_default() -> String {
    Default::default()
}

fn terminal_default() -> bool {
    false
}
