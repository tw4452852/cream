use crate::connection::*;
use shared::*;
use std::borrow::Cow;
use zoon::*;

#[static_ref]
fn ip() -> &'static Mutable<String> {
    Mutable::new("127.0.0.1".to_owned())
}

#[static_ref]
fn port() -> &'static Mutable<u32> {
    Mutable::new(2019)
}

#[static_ref]
fn status() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

#[static_ref]
fn connecting() -> &'static Mutable<bool> {
    Mutable::new(false)
}

fn set_ip(new_ip: String) {
    ip().set_neq(new_ip);
}

fn set_port(p: u32) {
    port().set_neq(p);
}

pub fn set_status(reason: impl Into<Cow<'static, str>>) {
    status().set(Some(reason.into()));
}

fn set_connecting(is: bool) {
    connecting().set_neq(is);
}

pub fn get_url() -> String {
    format!(
        "http://{}:{}",
        *ip().read_only().lock_ref(),
        *port().read_only().lock_ref()
    )
}

fn connect() {
    let connecting = connecting().read_only();

    if !connecting.get() {
        set_connecting(true);

        let ip = ip().read_only().get_cloned();
        let port = port().read_only().get();
        Task::start(async move {
            let up_msg = UpMsg::Connect(ip, port);
            if let Ok(cor_id) = connection().send_up_msg(up_msg).await {
                wait_for_cor_id(cor_id).await;
                set_connecting(false);
            }
        });
    }
}

pub fn root() -> impl Element {
    Column::new()
        .s(Height::fill().max(550))
        .s(Padding::new().y(10))
        .s(Align::center())
        .s(Width::fill().max(550))
        .item(title())
        .item(ip_input())
        .item(port_input())
        .s(Spacing::new(2))
        .item(connect_button())
        .item(connect_status())
}

fn title() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .s(Font::new().size(40))
        .child("Cream")
}

fn ip_input() -> impl Element {
    Row::new()
        .s(Spacing::new(2))
        .item(Label::new().label("Hostname:").for_input("ip"))
        .item(
            crate::widget::textinput()
                .id("ip")
                .placeholder(Placeholder::new("Hostname or IP address"))
                .text(ip().get_cloned())
                .on_change(set_ip),
        )
}

fn port_input() -> impl Element {
    let (parse_err, parse_err_sig) = Mutable::new_and_signal_cloned(None);

    Row::new()
        .s(Spacing::new(2))
        .item(Label::new().label("Port:").for_input("port"))
        .item(
            Column::new()
                .item(
                    crate::widget::textinput()
                        .id("port")
                        .placeholder(Placeholder::new("Port"))
                        .text(port().get_cloned())
                        .on_change(move |s| {
                            parse_err.set(None);
                            match s.parse() {
                                Ok(p) => set_port(p),
                                Err(e) => parse_err.set(Some(e)),
                            }
                        }),
                )
                .item_signal(parse_err_sig.map_some(|s| Text::new(s.to_string()))),
        )
}

fn connect_button() -> impl Element {
    crate::widget::button()
        .label_signal(
            connecting()
                .signal()
                .map(|ongoing| if ongoing { "Connecting..." } else { "Connect" }),
        )
        .on_press(connect)
}

fn connect_status() -> impl Element {
    El::new().child_signal(status().signal_cloned())
}
