use super::*;

impl AsElement for Mutable<Option<String>> {
    type EL = El<el::ChildFlagSet>;

    fn as_root_element(&self, id: impl ToString) -> Self::EL {
        let id = id.to_string();
        let placeholder = id.split("/").last().unwrap().to_owned();
        let self_mut = self.clone();

        El::new().child_signal(self.signal_cloned().map_some(move |v| {
            let val_mut: Mutable<Option<String>> = Mutable::new(None);
            Row::new()
                .item(
                    Label::new()
                        .for_input(&id)
                        .label(placeholder.to_owned() + ": "),
                )
                .item(
                    text_input!(id, v, placeholder)
                        .on_change({
                            let val_mut = val_mut.clone();
                            move |s| {
                                if s.is_empty() {
                                    val_mut.set(None);
                                } else if s != v.to_string() {
                                    val_mut.set(Some(s));
                                }
                            }
                        })
                        .input_type(InputType::text()),
                )
                .item_signal(val_mut.signal_cloned().map_some({
                    let self_mut = self_mut.clone();
                    let id = id.clone();
                    move |s| {
                        button!("ok").on_press({
                            let self_mut = self_mut.clone();
                            let id = id.clone();
                            move || {
                                crate::sync_config!(set, id, serde_json::to_string(&s).unwrap(), {
                                    self_mut.set(Some(s));
                                });
                            }
                        })
                    }
                }))
                .item(button!("x").on_press({
                    let self_mut = self_mut.clone();
                    let id = id.clone();
                    move || {
                        crate::sync_config!(del, id, {
                            self_mut.set(None);
                        });
                    }
                }))
        }))
    }
}
