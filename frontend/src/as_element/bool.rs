use super::*;

impl AsElement for Mutable<Option<bool>> {
    type EL = El<el::ChildFlagSet>;

    fn as_root_element(&self, id: impl ToString) -> Self::EL {
        let id = id.to_string();
        let placeholder = id.split("/").last().unwrap().to_owned();
        let self_mut = self.clone();

        El::new().child_signal(self.signal_cloned().map_some(move |selected| {
            Row::new()
                .item(
                    Label::new()
                        .for_input(&id)
                        .label(placeholder.to_owned() + ": "),
                )
                .item(
                    checkbox!()
                        .id(&id)
                        .checked(selected)
                        .icon(|selected| {
                            Text::with_signal(selected.signal().map_bool(|| "true", || "false"))
                        })
                        .on_change({
                            let self_mut = self_mut.clone();
                            let id = id.clone();
                            move |selected| {
                                crate::sync_config!(
                                    set,
                                    id,
                                    serde_json::to_string(&selected).unwrap(),
                                    {
                                        self_mut.set(Some(selected));
                                    }
                                );
                            }
                        }),
                )
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
