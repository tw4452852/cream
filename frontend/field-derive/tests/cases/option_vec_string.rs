use field_derive::*;
use zoon::{Deserialize, Serialize};

fn mutable_is_none<T>(m: &Mutable<Option<T>>) -> bool {
    m.map(|v| v.is_none())
}

#[ignore_none]
#[derive(Serialize, Deserialize, Field)]
#[serde(crate = "serde")]
struct S {
    f: Mutable<Option<Vec<String>>>,
}

fn f_default() -> String {
    Default::default()
}
fn main() {}
