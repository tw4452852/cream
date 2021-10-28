use field_derive::*;
use zoon::{Deserialize, Serialize};

fn mutable_is_none<T>(m: &Mutable<Option<T>>) -> bool {
    m.map(|v| v.is_none())
}

#[ignore_none]
#[derive(Serialize, Deserialize, Field)]
#[serde(crate = "serde")]
struct S {
    r#match: Mutable<Option<String>>,
}

fn match_default() -> String {
    Default::default()
}

fn main() {}
