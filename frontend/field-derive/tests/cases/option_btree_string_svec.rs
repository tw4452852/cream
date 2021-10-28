use field_derive::*;
use std::collections::BTreeMap;
use zoon::{Deserialize, Serialize};

fn mutable_is_none<T>(m: &Mutable<Option<T>>) -> bool {
    m.map(|v| v.is_none())
}

#[ignore_none]
#[derive(Serialize, Deserialize, Field)]
#[serde(crate = "serde")]
struct A {
    f: Mutable<Option<BTreeMap<String, MSVec>>>,
}

type MSVec = Mutable<Vec<String>>;

fn main() {}
