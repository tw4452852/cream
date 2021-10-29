use zoon::*;

mod bool;
mod obj;
mod string;
mod u32;
mod vec;

pub(crate) trait AsElement {
    type EL: Element;
    fn as_root_element(&self, id: impl ToString) -> Self::EL;
}

pub(crate) trait Obj {}
pub(crate) trait Enu {}

macro_rules! button {
    ($label:expr) => {{
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .label($label)
            .s(Borders::all(Border::new()))
            .s(RoundedCorners::all(7))
            .s(Align::new().left())
            .s(Padding::all(2))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| named_color::GREEN_5, || named_color::GRAY_0),
            ))
    }};
}
pub(crate) use button;

macro_rules! text_input {
    ($id:expr, $text:expr, $placeholder:expr) => {{
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        TextInput::new()
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .s(Borders::new().bottom_signal(
                hovered_signal
                    .map_bool(|| Border::new(), || Border::new().color(hsluv!(0, 0, 0, 0))),
            ))
            .s(Scrollbars::both())
            .id(&$id)
            .text($text.clone())
            .placeholder(Placeholder::new("<".to_string() + &$placeholder + ">"))
    }};
}
pub(crate) use text_input;

macro_rules! checkbox {
    () => {{
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Checkbox::new()
            .s(Borders::all(Border::new()))
            .s(RoundedCorners::all(7))
            .s(Align::new().left())
            .s(Padding::all(2))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| named_color::GREEN_5, || named_color::GRAY_0),
            ))
    }};
}
pub(crate) use checkbox;
