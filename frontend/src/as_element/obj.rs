use super::*;

impl<S: AsElement + Clone + 'static> AsElement for Mutable<Option<S>> {
    type EL = El<el::ChildFlagSet>;

    fn as_root_element(&self, id: impl ToString) -> Self::EL {
        let id = id.to_string();
        let placeholder = id.split("/").last().unwrap().to_owned();
        let self_mut = self.clone();

        El::new().child_signal(self.signal_cloned().map_some(move |v| {
            let cl = Mutable::new(true);
            Column::new()
                .item(
                    Row::new()
                        .s(Spacing::new(2))
                        .item(
                            Button::new()
                                .label_signal(cl.signal().map_bool(|| "▶", || "▼"))
                                .on_press({
                                    let cl = cl.clone();
                                    move || cl.update(|v| !v)
                                }),
                        )
                        .item(Text::new(&placeholder))
                        .item(button!("x").on_press({
                            let self_mut = self_mut.clone();
                            let id = id.clone();
                            move || {
                                crate::sync_config!(del, id, {
                                    self_mut.set(None);
                                });
                            }
                        })),
                )
                .item_signal(cl.signal().map_false({
                    let id = id.clone();
                    move || v.as_root_element(&id)
                }))
        }))
    }
}
