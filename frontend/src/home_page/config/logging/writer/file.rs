use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    filename: Mutable<Option<String>>,
    roll: Mutable<Option<bool>>,
    roll_size_mb: Mutable<Option<u32>>,
    roll_gzip: Mutable<Option<bool>>,
    roll_local_time: Mutable<Option<bool>>,
    roll_keep: Mutable<Option<u32>>,
    roll_keep_days: Mutable<Option<u32>>,
}

fn filename_default() -> String {
    Default::default()
}

fn roll_default() -> bool {
    true
}

fn roll_size_mb_default() -> u32 {
    0
}

fn roll_gzip_default() -> bool {
    true
}

fn roll_local_time_default() -> bool {
    false
}

fn roll_keep_default() -> u32 {
    10
}

fn roll_keep_days_default() -> u32 {
    90
}
