use zoon::*;

mod config;

#[static_ref]
fn config() -> &'static Mutable<Option<config::Val>> {
    Mutable::new(None)
}

#[static_ref]
fn config_err() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn is_syncing() -> &'static Mutable<bool> {
    Mutable::new(false)
}

pub fn set_is_syncing(v: bool) {
    is_syncing().set_neq(v);
}

pub fn set_config_err(s: Option<impl std::string::ToString>) {
    config_err().set(s.map(|s| s.to_string()))
}

pub fn set_config(s: &str) {
    let raw: Option<serde_json::Value> = serde_json::from_str(s).ok();
    zoon::println!("{:#?}", raw);

    config().set(serde_json::from_str(s).map_or_else(
        |e| {
            if let Some(v) = raw {
                if v == serde_json::Value::Null {
                    return Some(Default::default());
                }
            }
            zoon::println!("{:#?}", e);
            set_config_err(Some(format!("{}", e)));
            None
        },
        |c| {
            set_config_err(None::<String>);
            Some(c)
        },
    ));
}

pub fn root() -> impl Element {
    Column::new()
        .item(Text::new("Configuration"))
        .item_signal(
            config()
                .signal_cloned()
                .map_some(|config| config.root("config".to_owned())),
        )
        .item_signal(is_syncing().signal().map_true(|| {
            El::new()
                .s(Background::new().color(named_color::GREEN_5))
                .s(Font::new().color(named_color::GRAY_9))
                .s(Width::fill())
                .child(Text::new("Syncing ..."))
        }))
        .item_signal(config_err().signal_ref(|e| {
            e.as_ref().map(|e| {
                El::new()
                    .s(Background::new().color(named_color::RED_5))
                    .s(Font::new().color(named_color::GRAY_9))
                    .s(Width::fill())
                    .child(Text::new(e.to_string()))
            })
        }))
}

#[macro_export]
macro_rules! sync_config {
    (get, $id:expr, $cb:expr) => {
        Task::start(async move {
            let up_msg = shared::UpMsg::Config(
                format!("{}/{}", crate::login_page::get_url(), $id.replace("r#", "")),
                shared::Config::Get,
            );
            crate::home_page::set_is_syncing(true);
            if let Ok(cor_id) = crate::connection::connection().send_up_msg(up_msg).await {
                crate::connection::wait_for_cor_id(cor_id).await;
                crate::home_page::set_is_syncing(false);
                $cb;
            }
        });
    };
    (del, $id:expr, $cb:expr) => {
        Task::start(async move {
            let up_msg = shared::UpMsg::Config(
                format!("{}/{}", crate::login_page::get_url(), $id.replace("r#", "")),
                shared::Config::Del,
            );
            crate::home_page::set_is_syncing(true);
            if let Ok(cor_id) = crate::connection::connection().send_up_msg(up_msg).await {
                crate::connection::wait_for_cor_id(cor_id).await;
                crate::home_page::set_is_syncing(false);
                $cb;
            }
        });
    };
    (set, $id:expr, $payload:expr, $cb:expr) => {
        Task::start(async move {
            let up_msg = shared::UpMsg::Config(
                format!("{}/{}", crate::login_page::get_url(), $id.replace("r#", "")),
                shared::Config::SetOrReplace($payload),
            );
            zoon::eprintln!("{:#?}", up_msg);
            crate::home_page::set_is_syncing(true);
            if let Ok(cor_id) = crate::connection::connection().send_up_msg(up_msg).await {
                crate::connection::wait_for_cor_id(cor_id).await;
                crate::home_page::set_is_syncing(false);
                $cb;
            }
        });
    };
    (new, $id:expr, $payload:expr, $cb:expr) => {
        Task::start(async move {
            let up_msg = shared::UpMsg::Config(
                format!("{}/{}", crate::login_page::get_url(), $id.replace("r#", "")),
                shared::Config::New($payload),
            );
            zoon::eprintln!("{:#?}", up_msg);
            crate::home_page::set_is_syncing(true);
            if let Ok(cor_id) = crate::connection::connection().send_up_msg(up_msg).await {
                crate::connection::wait_for_cor_id(cor_id).await;
                crate::home_page::set_is_syncing(false);
                $cb;
            }
        });
    };
}
