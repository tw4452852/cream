use field_derive::*;
use zoon::{Deserialize, Serialize};

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub struct Val {
    repo: Mutable<Option<String>>,
    path: Mutable<Option<String>>,
    branch: Mutable<Option<String>>,
    r#type: Mutable<Option<String>>,
    secret: Mutable<Option<String>>,
    depth: Mutable<Option<String>>,
    submodule: Mutable<Option<bool>>,
    command: Mutable<Option<Vec<String>>>,
    key: Mutable<Option<String>>,
    key_password: Mutable<Option<String>>,
    username: Mutable<Option<String>>,
    password: Mutable<Option<String>>,
    token: Mutable<Option<String>>,
}

fn repo_default() -> String {
    Default::default()
}

fn path_default() -> String {
    Default::default()
}

fn branch_default() -> String {
    Default::default()
}

fn type_default() -> String {
    Default::default()
}

fn secret_default() -> String {
    Default::default()
}

fn depth_default() -> String {
    Default::default()
}

fn submodule_default() -> bool {
    false
}

fn command_default() -> String {
    Default::default()
}

fn key_default() -> String {
    Default::default()
}

fn key_password_default() -> String {
    Default::default()
}

fn username_default() -> String {
    Default::default()
}

fn password_default() -> String {
    Default::default()
}

fn token_default() -> String {
    Default::default()
}
