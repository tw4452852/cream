use zoon::*;

pub fn button() -> Button<button::LabelFlagNotSet, button::OnPressFlagNotSet> {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| named_color::GREEN_1, || named_color::GRAY_0)))
        .s(Padding::all(2))
        .s(RoundedCorners::all(7))
        .s(Borders::all(Border::new()))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
}

pub fn textinput() -> TextInput<
    text_input::IdFlagNotSet,
    text_input::OnChangeFlagNotSet,
    text_input::PlaceholderFlagNotSet,
    text_input::TextFlagNotSet,
    text_input::LabelFlagNotSet,
    text_input::InputTypeFlagNotSet,
    text_input::ReadOnlyFlagNotSet,
> {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    TextInput::new()
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .s(Borders::new().bottom_signal(
            hovered_signal.map_bool(|| Border::new(), || Border::new().color(hsluv!(0, 0, 0, 0))),
        ))
        .s(Scrollbars::both())
}
