use crate::{app, home_page, login_page};
use shared::*;
use zoon::*;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        zoon::println!("{}: {:#?}", cor_id, down_msg);

        match down_msg {
            DownMsg::ConnectResult(r) => match r {
                Ok(config) => {
                    home_page::set_config(config.as_str());
                    app::set_page_id(app::PageId::Home);
                }
                Err(reason) => login_page::set_status(reason),
            },
            DownMsg::ConfigResult(r) => match r {
                Ok(_) => {
                    home_page::set_config_err(None::<String>);
                }
                Err(e) => {
                    home_page::set_config_err(Some(e));
                }
            },
        }

        recent_cor_id().set_neq(Some(cor_id));
    })
}

#[static_ref]
fn recent_cor_id() -> &'static Mutable<Option<CorId>> {
    Mutable::new(None)
}

pub async fn wait_for_cor_id(cor_id: CorId) {
    recent_cor_id().signal().wait_for(Some(cor_id)).await;
}
