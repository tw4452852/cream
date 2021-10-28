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
    f: Mutable<Option<BTreeMap<String, Val>>>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(crate = "serde")]
struct Val {}

impl Val {
    fn root(self, _id: String) -> impl Element {
        Text::new("b")
    }
}

fn main() {}
