#![recursion_limit = "256"]

use zoon::*;

mod app;
mod connection;
mod home_page;
mod login_page;
mod router;
mod widget;

type MSVec = Mutable<Vec<String>>;
type MString = Mutable<String>;

pub fn mutable_is_none<T>(m: &Mutable<Option<T>>) -> bool {
    m.map(|v| v.is_none())
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", app::root);
}
