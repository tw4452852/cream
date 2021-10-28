use field_derive::*;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
use zoon::{Deserialize, Serialize};

fn mutable_is_none<T>(m: &Mutable<Option<T>>) -> bool {
    m.map(|v| v.is_none())
}

#[ignore_none]
#[derive(Serialize, Deserialize, Field)]
#[serde(crate = "serde")]
struct S {
    f: Mutable<Option<crate::Enum>>,
}

fn f_default() -> Enum {
    Default::default()
}

#[derive(Clone, Field, AsRefStr, EnumVariantNames, EnumString, Serialize, Deserialize)]
#[serde(crate = "serde")]
enum Enum {
    A(A),
    B(B),
}

impl Default for Enum {
    fn default() -> Self {
        Self::A(Default::default())
    }
}

#[ignore_none]
#[derive(Field, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
struct A {
    af: Mutable<Option<u32>>,
}

fn af_default() -> u32 {
    1
}

#[ignore_none]
#[derive(Field, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
struct B {
    bf: Mutable<Option<u32>>,
}

fn bf_default() -> u32 {
    2
}

fn main() {}
