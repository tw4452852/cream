use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    message_key: Mutable<Option<String>>,
    level_key: Mutable<Option<String>>,
    time_key: Mutable<Option<String>>,
    name_key: Mutable<Option<String>>,
    caller_key: Mutable<Option<String>>,
    stacktrace_key: Mutable<Option<String>>,
    line_ending: Mutable<Option<String>>,
    time_format: Mutable<Option<String>>,
    duration_format: Mutable<Option<String>>,
    level_format: Mutable<Option<String>>,
}

fn message_key_default() -> String {
    Default::default()
}

fn level_key_default() -> String {
    Default::default()
}

fn time_key_default() -> String {
    Default::default()
}

fn name_key_default() -> String {
    Default::default()
}

fn caller_key_default() -> String {
    Default::default()
}

fn stacktrace_key_default() -> String {
    Default::default()
}

fn line_ending_default() -> String {
    Default::default()
}

fn time_format_default() -> String {
    Default::default()
}

fn duration_format_default() -> String {
    Default::default()
}

fn level_format_default() -> String {
    Default::default()
}
