use super::*;

impl<T> AsElement for Mutable<Option<Vec<T>>>
where
    T: Clone + 'static + Default + Serialize,
    Mutable<Option<T>>: AsElement,
{
    type EL = El<el::ChildFlagSet>;
    fn as_root_element(&self, id: impl ToString) -> Self::EL {
        let id = id.to_string();
        let placeholder = id.split("/").last().unwrap().to_owned();
        let self_mut = self.clone();

        El::new().child_signal(self.signal_cloned().map_some({
            let self_mut = self_mut.clone();
            let id = id.clone();

            move |v| {
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
                        let self_mut = self_mut.clone();
                        move || {
                            let v = v.clone();
                            Column::new()
                                .item(button!("add").on_press({
                                    let self_mut = self_mut.clone();
                                    let id = id.clone();

                                    move || {
                                        let v: T = Default::default();
                                        crate::sync_config!(
                                            set,
                                            id,
                                            serde_json::to_string(&v).unwrap(),
                                            {
                                                self_mut.update_mut(|m| {
                                                    m.as_mut().unwrap().push(v);
                                                });
                                            }
                                        );
                                    }
                                }))
                                .items(v.into_iter().enumerate().map({
                                    let self_mut = self_mut.clone();
                                    let id = id.clone();
                                    move |(i, item)| {
                                        let item = Mutable::new(Some(item));
                                        El::new().child_signal(item.signal_cloned().map({
                                            let self_mut = self_mut.clone();
                                            let id = format!("{}/{}", id, &i);
                                            move |v| {
                                                if v.is_none() {
                                                    self_mut.update_mut(|m| {
                                                        m.as_mut().unwrap().remove(i);
                                                    });
                                                }
                                                item.as_root_element(&id)
                                            }
                                        }))
                                    }
                                }))
                        }
                    }))
            }
        }))
    }
}
