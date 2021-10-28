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
    ip().set(new_ip);
}

fn set_port(new_port: String) {
    port().set(new_port.parse().unwrap());
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
        .item(
            Row::new()
                .item(ip_input())
                .item(port_input())
                .item(connect_button()),
        )
        .item(connect_status())
}

fn ip_input() -> impl Element {
    TextInput::new()
        .label_hidden("IP Address")
        .placeholder(Placeholder::new("127.0.0.1"))
        .text(ip().get_cloned())
        .on_change(set_ip)
}

fn port_input() -> impl Element {
    TextInput::new()
        .label_hidden("Port")
        .placeholder(Placeholder::new(2019))
        .text(port().get_cloned())
        .on_change(set_port)
}

fn connect_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(
            hovered_signal.map_bool(|| named_color::GREEN_5, || named_color::GREEN_2),
        ))
        .s(Padding::all(7))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
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
