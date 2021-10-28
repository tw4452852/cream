use crate::{home_page, login_page};
use zoon::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Login,
    Home,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Login)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

// View
pub fn root() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Login => login_page::root().into_raw_element(),
        PageId::Home => home_page::root().into_raw_element(),
    }))
}
