use json_patch::{diff, patch, Patch, PatchOperation};
use zoon::*;

mod config;

static mut SERVER_CONFIG: serde_json::Value = serde_json::Value::Null;

#[static_ref]
fn config() -> &'static Mutable<Option<config::Val>> {
    Mutable::new(None)
}

#[static_ref]
fn need_diff() -> &'static Mutable<()> {
    Mutable::new(())
}

#[static_ref]
fn config_err() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn is_syncing() -> &'static Mutable<bool> {
    Mutable::new(false)
}

pub fn trigger_diff() {
    need_diff().set(());
}

fn set_is_syncing(v: bool) {
    is_syncing().set_neq(v);
}

pub fn set_config_err(s: Option<impl std::string::ToString>) {
    config_err().set(s.map(|s| s.to_string()));
}

pub fn update_server_config() {
    unsafe {
        SERVER_CONFIG = serde_json::to_value(config().read_only().get_cloned()).unwrap();
    }
    trigger_diff();
}

pub fn set_config(s: &str) {
    let raw: Option<serde_json::Value> = serde_json::from_str(s).ok();
    zoon::println!("{:#?}", raw);

    if let Some(c) = raw.clone() {
        unsafe {
            SERVER_CONFIG = c;
        };
    }

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
    Row::new()
        .item(
            Column::new()
                .s(Align::new().top())
                .item(Text::new("Configuration"))
                .item_signal(config().signal_cloned().map_some(|config| config.root(0)))
                .item_signal(is_syncing().signal().map_true(|| {
                    El::new()
                        .s(Background::new().color(named_color::GREEN_1))
                        .s(Font::new().color(named_color::GRAY_9))
                        .s(Width::fill())
                        .child(Text::new("Syncing ..."))
                }))
                .item_signal(config_err().signal_ref(|e| {
                    e.as_ref().map(|e| {
                        El::new()
                            .s(Background::new().color(named_color::RED_1))
                            .s(Font::new().color(named_color::GRAY_9))
                            .s(Width::fill())
                            .child(Text::new(e.to_string()))
                    })
                })),
        )
        .item(diff_page())
}

fn diff_page() -> impl Element {
    let (show_commit, show_commit_sig) = Mutable::new_and_signal(false);
    Column::new()
        .s(Align::new().right())
        .s(Align::new().top())
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item("Pending changes:")
                .item_signal(show_commit_sig.map_true(|| {
                    crate::widget::button().label("commit").on_press(|| {
                        Task::start(async move {
                            let up_msg = shared::UpMsg::Config(
                                crate::login_page::get_url() + "/load",
                                shared::Config::SetOrReplace(
                                    serde_json::to_string(&config().lock_ref().as_ref().unwrap())
                                        .unwrap(),
                                ),
                            );
                            set_is_syncing(true);
                            if let Ok(cor_id) =
                                crate::connection::connection().send_up_msg(up_msg).await
                            {
                                crate::connection::wait_for_cor_id(cor_id).await;
                                set_is_syncing(false);
                            }
                        });
                    })
                })),
        )
        .item_signal(need_diff().signal().map(move |_| {
            let cur = serde_json::to_value(config().read_only().get_cloned()).unwrap();
            let mut srv = unsafe { SERVER_CONFIG.clone() };
            let patches = diff(&srv, &cur).0;

            show_commit.set(!patches.is_empty());

            Column::new()
                .s(Padding::new().top(2))
                .items(patches.into_iter().map(|op| {
                    let el = match op {
                        PatchOperation::Add(ref add) => El::new()
                            .s(Background::new().color(named_color::GREEN_1))
                            .child(format!(
                                "+{}: {}",
                                &add.path,
                                serde_json::to_string_pretty(&add.value).unwrap()
                            )),
                        PatchOperation::Remove(ref remove) => El::new()
                            .s(Background::new().color(named_color::RED_1))
                            .child(format!(
                                "- {}: {}",
                                &remove.path,
                                serde_json::to_string_pretty(srv.pointer(&remove.path).unwrap())
                                    .unwrap()
                            )),
                        PatchOperation::Replace(ref replace) => El::new()
                            .s(Background::new().color(named_color::GREEN_1))
                            .child(format!(
                                "{}: {} -> {}",
                                &replace.path,
                                serde_json::to_string_pretty(srv.pointer(&replace.path).unwrap())
                                    .unwrap(),
                                serde_json::to_string_pretty(&replace.value).unwrap()
                            )),
                        _ => todo!(),
                    };

                    patch(&mut srv, &Patch(vec![op])).unwrap();

                    el
                }))
        }))
}
