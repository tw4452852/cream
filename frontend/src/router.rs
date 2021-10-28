use crate::app::{self, PageId};
use zoon::*;

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::Login) => {
            app::set_page_id(PageId::Login);
        }
        _ => (),
    })
}

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
pub enum Route {
    #[route()]
    Login,
}
