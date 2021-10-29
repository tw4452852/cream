#![recursion_limit = "256"]

use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, parse_str, spanned::Spanned, DataEnum, DataStruct, DeriveInput, Type,
};

fn extract_type_levels(ty: &syn::Type) -> Vec<String> {
    ty.clone()
        .into_token_stream()
        .to_string()
        .replace('>', "")
        .replace(' ', "")
        .replace('\n', "")
        .split('<')
        .map(|s| format!("{}", s.split(',').last().unwrap()))
        .collect()
}

fn struct_field_derive(s_ident: syn::Ident, s: DataStruct) -> proc_macro2::TokenStream {
    let mut fields_output = quote! {};
    let mut primitive_fields = Vec::new();
    let mut other_fields = Vec::new();
    let mut enum_fields = Vec::new();
    let mut enum_fields_types = Vec::new();

    let button_style = quote! {
        .s(Borders::all(Border::new()))
        .s(RoundedCorners::all(7))
        .s(Align::new().left())
        .s(Padding::all(2))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || named_color::GREEN_5,
            || named_color::GRAY_0,
        )))
    };
    let button = quote! {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()#button_style
    };

    let current_level = quote! {
        { module_path!().matches("::").count() as u32 }
    };
    let textinput = quote! {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        TextInput::new()
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .s(Borders::new().bottom_signal(hovered_signal.map_bool(|| Border::new(), || Border::new().color(hsluv!(0, 0, 0, 0)))))
        .s(Scrollbars::both())
    };

    if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = s.fields {
        for f in named {
            let ident = format_ident!("{}", f.ident.as_ref().unwrap());
            let ident_s = ident.to_string();
            let ident_raw = format_ident!("r#{}", ident);
            let ident_mut = format_ident!("{}_mut", ident);
            let ident_page = format_ident!("{}_page", ident);

            if ident == "id" {
                continue;
            }

            let ty = f.ty;
            let tys = extract_type_levels(&ty);

            if let [inner, outer] = tys.iter().rev().take(2).collect::<Vec<_>>().as_slice() {
                match (outer.as_str(), inner.as_str()) {
                    ("Option", "u32" | "String" | "bool") => {
                        primitive_fields.push(ident_raw.clone());
                    }
                    ("Vec", "u32" | "String" | "bool") => other_fields.push(ident_raw.clone()),
                    ("Option" | "BTreeMap" | "Vec", inner) if inner.ends_with("Val") => {
                        other_fields.push(ident_raw.clone());
                    }
                    ("BTreeMap" | "Vec", inner) if inner.ends_with("Enum") => {
                        other_fields.push(ident_raw.clone());
                    }
                    ("BTreeMap", inner) if inner.ends_with("MSVec") => {
                        other_fields.push(ident_raw.clone());
                    }
                    ("BTreeMap", inner) if inner.ends_with("MString") => {
                        other_fields.push(ident_raw.clone());
                    }
                    ("Option", inner) if inner.ends_with("Enum") => {
                        enum_fields.push(ident_raw.clone());
                        enum_fields_types.push(parse_str::<Type>(&inner).unwrap());
                    }
                    (x, y) => {
                        fields_output.extend(
                            syn::Error::new(
                                ty.span(),
                                format!("unsupported inner type: {}<{}>", x, y),
                            )
                            .to_compile_error(),
                        );
                        continue;
                    }
                }

                fields_output.extend(quote! {
                 fn #ident_mut(&self) -> #ty {
                     self.#ident_raw.clone()
                 }
                });

                match (outer.as_str(), inner.as_str()) {
                    ("Option" | "Vec", "u32" | "String" | "bool") => fields_output.extend({
                    quote_spanned! {ty.span()=>
                        fn #ident_page(&self, id: String) -> impl Element {
                        	self.#ident_raw.as_root_element(id + "/" + #ident_s)
                        }
                    }}),
                    ("Option" , x) if x.ends_with("Val") || x.ends_with("Enum") => fields_output.extend({
                    quote_spanned! {ty.span()=>
                        fn #ident_page(&self, id: String) -> impl Element {
                        	self.#ident_raw.as_root_element(id + "/" + #ident_s)
                        }
                    }}),
                    ("Vec" , x) if x.ends_with("Val") => fields_output.extend({
                    quote_spanned! {ty.span()=>
                        fn #ident_page(&self, id: String) -> impl Element {
                        	self.#ident_raw.as_root_element(id + "/" + #ident_s)
                        }
                    }}),
                    (outer @ ("BTreeMap"|"Vec"), inner) => fields_output.extend({
                    let inner_field_name = ident.to_string();
                    let inner_field_name = inner_field_name.trim_end_matches('s');
                    let inner_type = parse_str::<Type>(inner).unwrap();
                    let child = match outer {
                    	"BTreeMap" => {
                    		let child = if inner.ends_with("MSVec") {
                    			// child of Mutable<Option<BTreeMap<string, Mutable<Vec<String>>>>>
                    			quote_spanned! {ty.span()=>
                    				Column::new()
                    				.item({#button.label(format!("add {}", &k)).on_press({
                    			  		let id = id.clone();
                    			  		let mv = mv.clone();
                    			  		move || {
                    			  			let s: String = Default::default();
                    						#[cfg(not(trybuild))]
											crate::sync_config!(set, id, serde_json::to_string(&s).unwrap(),
											{
												mv.update_mut(|v| {
													v.push(s);
												});
											});
											#[cfg(trybuild)]
											{
												mv.update_mut(|v| {
													v.push(s);
												});
												let _ = id;
											}
										}
									})})
                    				.item_signal(mv.signal_cloned().map({
                    					let id = id.clone();
                    					let k = k.clone();
                    					let mv = mv.clone();
                    					move |v| {
                    						Column::new()
                    						.items(v.into_iter().enumerate().map(|(i, s)| {
                    							let val_mut: Mutable<Option<String>> = Mutable::new(None);
												Row::new()
													.item({#textinput.id(i).text(&s).on_change({
														let val_mut = val_mut.clone();
														let init_s = s;
														move |s| {
															if s.is_empty() {
                                         						val_mut.set(None);
                                         					} else if  s != init_s {
                                         						val_mut.set(Some(s));
                                         					}
														}
													}).input_type(InputType::text()).placeholder(Placeholder::new(format!("<{}>", &k)))
													})
													.item_signal(val_mut.signal_cloned().map_some({
														let id = id.clone();
														let mv = mv.clone();
                                    					move |s| {
                                    						#button.label("ok").on_press({
                                    						let id = id.clone();
                                    						let mv = mv.clone();
                                     						move || {
                                     							#[cfg(not(trybuild))]
                                     							crate::sync_config!(set, format!("{}/{}", id, i), serde_json::to_string(&s).unwrap(),
																{
																	mv.update_mut(|v| {
																		v[i] = s;
																	});
																});
                                     							#[cfg(trybuild)]
                                     							{
                                     								mv.update_mut(|v| {
																		v[i] = s;
																	});
																	let _ = id;
																}
                                     						}})
                                     					}
                                     				}))
                                     				.item({#button.label("x").on_press({
														let id = id.clone();
														let mv = mv.clone();
														move || {
															#[cfg(not(trybuild))]
															crate::sync_config!(del, format!("{}/{}", id, i),
																mv.update_mut(|v| {
																	v.remove(i);
																})
															);
															#[cfg(trybuild)]
															{
																mv.update_mut(|v| {
																	v.remove(i);
																});
																let _ = id;
															}
														}
													})})
                    						}))
                    					}
                    				}))

                    			}
                    		} else if inner.ends_with("MString") {
                    			// child of Mutable<Option<BTreeMap<string, Mutable<String>>>>
                    			quote_spanned! {ty.span()=>
                    				let val_mut: Mutable<Option<String>> = Mutable::new(None);
                    				let ms = mv.clone();
                    				let s = ms.read_only().get_cloned();
									Row::new()
									.item({#textinput.id(&k).text(&s).on_change({
											let val_mut = val_mut.clone();
											let init_s = s;
											move |s| {
												if s.is_empty() {
                                        			val_mut.set(None);
                                         		} else if  s != init_s {
                                         			val_mut.set(Some(s));
                                         		}
											}
										}).input_type(InputType::text()).placeholder(Placeholder::new(format!("<{}>", &k)))
									})
									.item_signal(val_mut.signal_cloned().map_some({
											let id = id.clone();
											let mv = mv.clone();
                                    		move |s| {
                                    			#button.label("ok").on_press({
                                   				let id = id.clone();
                                    			let mv = mv.clone();
                                     			move || {
                                   					#[cfg(not(trybuild))]
                                     				crate::sync_config!(set, id, serde_json::to_string(&s).unwrap(),
													{
														mv.set(s);
													});
                                     				#[cfg(trybuild)]
                                     				{
                                     					mv.set(s);
														let _ = id;
													}
                                     			}})
                                     		}
                                   	}))
                              	}
                    		} else {
                    			// child of Mutable<Option<BTreeMap<string, struct>>>
                    			quote_spanned! {ty.span()=>
                    				mv.clone().root(id)
                    			}
                    		};
                    		quote_spanned! {ty.span()=>{
							let f_mut = ident_mut.clone();
							let id = id.clone();
                    		move |m| {
                    			Column::new()
                    			.item({#button.label(format!("add {}", #inner_field_name)).on_press({
                    			  let m = m.clone();
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  move || {
                    				let mut k = String::new();
                    				for i in 0.. {
                    					k = format!("{}{}", #inner_field_name, i);
                    					if !m.contains_key(&k) {
                    						break;
                    					}
                    				}
                    				let v: #inner_type = Default::default();
                    				#[cfg(not(trybuild))]
									crate::sync_config!(new, format!("{}/{}/{}", id, stringify!(#ident), k), serde_json::to_string(&v).unwrap(),
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().insert(k, v);
										});
									});
									#[cfg(trybuild)]
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().insert(k, v);
										});
										let _ = id;
									}
                    			}})})
                    			.items(m.clone().into_iter().map({
                    				let f_mut = f_mut.clone();
                    				let id = id.clone();
                    				move |(k, mv)| {
                    					let cl = Mutable::new(true);
										Column::new()
										.item(Row::new()
											.s(Spacing::new(2))
											.item(Button::new().label_signal(cl.signal().map_bool(
                    							|| "▶",
                    							|| "▼",
                    							)).on_press({
                    								let cl = cl.clone();
                    								move || cl.update(|v| !v)
                    							})
                    					    )
											.item({
												let val_mut: Mutable<Option<String>> = Mutable::new(None);
												Row::new()
													.item({#textinput.id(&k).text(&k).on_change({
														let val_mut = val_mut.clone();
														let init_s = k.to_string();
														move |s| {
															if s.is_empty() {
                                         						val_mut.set(None);
                                         					} else if  s != init_s {
                                         						val_mut.set(Some(s));
                                         					}
														}
													}).input_type(InputType::text())})
													.item_signal(val_mut.signal_cloned().map_some({
                                     					let f_mut = f_mut.clone();
                                     					let id = id.clone();
                                     					let k = k.clone();
                                    					move |nk| {
                                    						#button.label("ok").on_press({
                                    						let f_mut = f_mut.clone();
                                    						let id = id.clone();
                                    						let k = k.clone();
                                     						move || {
                                     							// rename map's key
																f_mut.update_mut(|m| {
																	let m = m.as_mut().unwrap();

																	let v = m.remove(&k).unwrap();
																	#[cfg(not(trybuild))]
																	{
																		let old_id = format!("{}/{}/{}", &id, stringify!(#ident), &k);
																		let new_id = format!("{}/{}/{}", &id, stringify!(#ident), &nk);
																		let v = v.clone();
																		crate::sync_config!(del, old_id, {
																			crate::sync_config!(new, new_id, serde_json::to_string(&v).unwrap(), {});
																		});

																	}
																	m.insert(nk, v);
																	let _ = id;
																});
															}})
														}
                                     				}))
											})
											.item({#button.label("x").on_press({
												let f_mut = f_mut.clone();
												let id = id.clone();
												let k = k.clone();
												move || {
													#[cfg(not(trybuild))]
													crate::sync_config!(del, format!("{}/{}/{}", id, stringify!(#ident), k),
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(&k);
														})
													);
													#[cfg(trybuild)]
													{
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(&k);
														});
														let _ = id;
													}
												}
											})})
										)
										.item_signal(cl.signal().map_false({
											let id = id.clone();
											let k = k.clone();
                    						move || {
												let id = format!("{}/{}/{}", id.clone(), stringify!(#ident), k);
                    							#child
                    						}
                    					}))
                    				}}))
                    		}}}},
                    	"Vec" if inner.ends_with("Val") => {
                    		// child of Mutable<Option<Vec<struct>>>
                    		quote_spanned! {ty.span()=>{
							   let f_mut = ident_mut.clone();
							   let id = id.clone();
                    		   move |m| {
                    			Column::new()
                    			.item({#button.label(format!("add {}", #inner_field_name)).on_press({
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  let v: #inner_type = Default::default();
                    			  move || {
                    				#[cfg(not(trybuild))]
									crate::sync_config!(set, format!("{}/{}", id, stringify!(#ident)), serde_json::to_string(&v).unwrap(),
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().push(v);
										});
									});
									#[cfg(trybuild)]
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().push(v);
										});
										let _ = id;
									}
                    			}})})
                    			.items(m.clone().into_iter().enumerate().map({
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  move |(i, v)| {
                    				let cl = Mutable::new(true);
                    				Column::new()
                    				.item(Row::new()
                    						.s(Spacing::new(2))
                    						.item(Button::new().label_signal(cl.signal().map_bool(
                    							|| "▶",
                    							|| "▼",
                    							)).on_press({
                    								let cl = cl.clone();
                    								move || cl.update(|v| !v)
                    							})
                    					    )
                    					    .item(Text::new(#inner_field_name))
                    					    .item({#button.label("x").on_press({
                    					    	let f_mut = f_mut.clone();
												let id = id.clone();
												move || {
													#[cfg(not(trybuild))]
													crate::sync_config!(del, format!("{}/{}/{}", id, stringify!(#ident), i),
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(i);
														})
													);
													#[cfg(trybuild)]
													{
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(i);
														});
														let _ = id;
													}
												}
                    					    })})
                    				)
                    				.item_signal(cl.signal().map_false({
                    					let id = id.clone();
                    					move || {
                    						let id = format!("{}/{}/{}", id.clone(), stringify!(#ident), i);
                    						v.clone().root(id)
                    					}
                    				}))
                    			}}))
                    		}}
                    	}},
                    	"Vec" if inner.ends_with("Enum") => {
                    	 quote_spanned! {ty.span()=>{
                    		// child of Mutable<Option<Vec<enum>>>
							   let f_mut = ident_mut.clone();
							   let id = id.clone();
                    		   move |m| {
                    		   	use strum::VariantNames;
                    		   	use std::str::FromStr;

                    			Column::new()
                    			.item(
                    				Row::new()
                    				.s(Spacing::new(2))
                    				.item(Text::new("add: "))
                    				.items(#inner_type::VARIANTS.into_iter().map(|name| {
                    			  		#button.label(*name).on_press({
                    			  			let f_mut = f_mut.clone();
                    			  			let id = id.clone();
                    			  			move || {
                    			  				let v = #inner_type::from_str(name).unwrap();
                    							#[cfg(not(trybuild))]
												crate::sync_config!(set, format!("{}/{}", id, stringify!(#ident)), serde_json::to_string(&v).unwrap(),
													{
														f_mut.update_mut(|m| {
														m.as_mut().unwrap().push(v);
													});
												});
												#[cfg(trybuild)]
												{
													f_mut.update_mut(|m| {
														m.as_mut().unwrap().push(v);
													});
													let _ = id;
												}
											}
										})
									}))
                    			)
                    			.items(m.clone().into_iter().enumerate().map({
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  move |(i, v)| {
                    				let cl = Mutable::new(true);
                    				Column::new()
                    				.item(Row::new()
                    						.s(Spacing::new(2))
                    						.item(Button::new().label_signal(cl.signal().map_bool(
                    							|| "▶",
                    							|| "▼",
                    							)).on_press({
                    								let cl = cl.clone();
                    								move || cl.update(|v| !v)
                    							})
                    					    )
                    					    .item(Text::new(v.as_ref()))
                    					    .item({#button.label("x").on_press({
                    					    	let f_mut = f_mut.clone();
												let id = id.clone();
												move || {
													#[cfg(not(trybuild))]
													crate::sync_config!(del, format!("{}/{}/{}", id, stringify!(#ident), i),
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(i);
														})
													);
													#[cfg(trybuild)]
													{
														f_mut.update_mut(|m| {
															m.as_mut().unwrap().remove(i);
														});
														let _ = id;
													}
												}
                    					    })})
                    				)
                    				.item_signal(cl.signal().map_false({
                    					let id = id.clone();
                    					move || {
                    						let id = format!("{}/{}/{}", id.clone(), stringify!(#ident), i);
                    						v.clone().root(id)
                    					}
                    				}))
                    			}}))
                    		}
                    	}}},
                    	"Vec" => {
                    		// child of Mutable<Option<Vec<primitive>>>
                    		let def = format_ident!("{}_default", ident);
                    		 quote_spanned! {ty.span()=>{
							   let f_mut = ident_mut.clone();
							   let id = id.clone();
                    		   move |m| {
                    			Column::new()
                    			.item({#button.label(format!("add {}", #inner_field_name)).on_press({
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  move || {
                    				#[cfg(not(trybuild))]
									crate::sync_config!(set, format!("{}/{}", id, stringify!(#ident)), serde_json::to_string(&#def()).unwrap(),
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().push(#def());
										});
									});
									#[cfg(trybuild)]
									{
										f_mut.update_mut(|m| {
											m.as_mut().unwrap().push(#def());
										});
										let _ = id;
									}
                    			}})})
                    			.items(m.clone().into_iter().enumerate().map({
                    			  let f_mut = f_mut.clone();
                    			  let id = id.clone();
                    			  move |(i, v)| {
                    			  	let val_mut: Mutable<Option<String>> = Mutable::new(None);
                    				Row::new()
									.item({#textinput.id(i.to_string() + stringify!(#ident)).text(v.clone())
											.placeholder(Placeholder::new(format!("<{}>", stringify!(#ident))))
											.on_change({
                                         let init_s = v.to_string();
                                         let val_mut = val_mut.clone();
                                         move |s| {
                                         	if s.is_empty() {
                                         		val_mut.set(None);
                                         	} else if  s != init_s {
                                         		val_mut.set(Some(s));
                                         	}
                                         }
                                     }).input_type(InputType::text())})
                                     .item_signal(val_mut.signal_cloned().map_some({
                                     	let f_mut = f_mut.clone();
                                     	let id = id.clone();
                                    	move |s| {
                                    		#button.label("ok").on_press({
                                    		let f_mut = f_mut.clone();
                                    		let id = id.clone();
                                     		move || {
                                     			let v = s.parse().unwrap();
                                     			#[cfg(not(trybuild))]
                                     			{
													crate::sync_config!(set, format!("{}/{}/{}", id, stringify!(#ident), i), serde_json::to_string(&v).unwrap(), {
														f_mut.update_mut(|m| {
															m.as_mut().unwrap()[i] = v;
														});
													});
												}
												#[cfg(trybuild)]
												{
													f_mut.update_mut(|m| {
														m.as_mut().unwrap()[i] = v;
													});
													let _ = id;
												}
                                     		}})
                                     	}
                                     }))
									.item({#button.label("x").on_press({
										let f_mut = f_mut.clone();
										let id = id.clone();
										move || {
											#[cfg(not(trybuild))]
											crate::sync_config!(del, format!("{}/{}", id, i), {
												f_mut.update_mut(|m| {
													m.as_mut().unwrap().remove(i);
												});
											});
											#[cfg(trybuild)]
											{
												f_mut.update_mut(|m| {
													m.as_mut().unwrap().remove(i);
												});
												let _ = id;
											}

										}
									})})
								}}))}}}
                    	},
                    	_ => unreachable!(),
                    };


                    quote_spanned! {ty.span()=>
                    	// Mutable<Option<[Vec|BTreeMap|struct]>>
                    	fn #ident_page(&self, id: String) -> impl Element {
                    		let ident_mut = self.#ident_mut();
							let cl = Mutable::new(true);
                    		El::new().child_signal(ident_mut.signal_cloned().map_some(move |_| {
                    			Column::new()
                    			.item(Row::new()
                    				.s(Spacing::new(2))
                    				.item(Button::new().label_signal(cl.signal().map_bool(
                    						|| "▶",
                    						|| "▼",
                    					)).on_press({
                    						let cl = cl.clone();
                    						move || cl.update(|v| !v)
                    					})
                    				)
                    				.item(Text::new(stringify!(#ident)))
                    				.item({#button.label("x").on_press({
                    					let ident_mut = ident_mut.clone();
                    					let id = id.clone();
                    					move || {
                    						#[cfg(not(trybuild))]
                    						crate::sync_config!(del, format!("{}/{}", id, stringify!(#ident)), { ident_mut.set(None); });
                    						#[cfg(trybuild)]
                    						{ ident_mut.set(None); let _ = id; }
                    					}
                    				})})
                    			)
                    			.item_signal(cl.signal().map_false({
                    				let ident_mut = ident_mut.clone();
                    				let id = id.clone();
                    				move || {
                    					El::new().child_signal(ident_mut.signal_cloned().map_some(#child))
                    				}
                    			}))
                    		}))
                    	}
                    }}),
					_ => unreachable!(),
                }
            }
        }
    }

    let add_fields_page = {
        let primitive_fields_defs = primitive_fields
            .iter()
            .map(|x| format_ident!("{}_default", x));
        quote_spanned! {proc_macro2::Span::call_site()=>
            fn add_fields_page(&self, id: String) -> impl Element {
                El::new().child_signal({
                    let show_add = map_ref! {
                        let false_sig = Mutable::new(false).signal()
                        #(, let #primitive_fields = self.#primitive_fields.signal_ref(Option::is_some))*
                        #(, let #other_fields = self.#other_fields.signal_ref(Option::is_some))*
                        #(, let #enum_fields = self.#enum_fields.signal_ref(Option::is_some))*
                        =>
                        *false_sig #(|| !*#primitive_fields)* #(|| !*#other_fields)* #(|| !*#enum_fields)*
                    };

                    show_add.map_true({
                        #( let #primitive_fields = self.#primitive_fields.clone();)*
                        #( let #other_fields = self.#other_fields.clone();)*
                        #( let #enum_fields = self.#enum_fields.clone();)*
                        move || {
                            Row::new()
                                .multiline()
                                .s(Spacing::new(2))
                                .item(Text::new("add: "))
                                #( .item_signal(#primitive_fields.signal_cloned().map_none({
                                    let #primitive_fields = #primitive_fields.clone();
                                    let id = id.clone();
                                    move || {
                                        #button.label(stringify!(#primitive_fields).trim_start_matches("r#")).on_press({
                                            let #primitive_fields = #primitive_fields.clone();
                                            let id = id.clone();
                                            move || {
                                                let mut v = #primitive_fields.take();
                                                v = Some(#primitive_fields_defs());
                                                #[cfg(not(trybuild))]
                                                {
                                                    crate::sync_config!(new, format!("{}/{}", id, stringify!(#primitive_fields)), serde_json::to_string(&v).unwrap(), { #primitive_fields.set(v); });
                                                }
                                                #[cfg(trybuild)]
                                                {
                                                    #primitive_fields.set(v);
                                                }
                                            }
                                        })
                                    }
                                })))*
                                #( .item_signal(#other_fields.signal_cloned().map_none({
                                    let #other_fields = #other_fields.clone();
                                    let id = id.clone();
                                    move || {
                                        #button.label(stringify!(#other_fields).trim_start_matches("r#")).on_press({
                                            let #other_fields = #other_fields.clone();
                                            let id = id.clone();
                                            move || {
                                                let mut v = #other_fields.take();
                                                v = Some(Default::default());
                                                #[cfg(not(trybuild))]
                                                {
                                                    crate::sync_config!(new, format!("{}/{}", id, stringify!(#other_fields)), serde_json::to_string(&v).unwrap(), { #other_fields.set(v); });
                                                }
                                                #[cfg(trybuild)]
                                                {
                                                    #other_fields.set(v);
                                                }
                                            }
                                        })
                                    }
                                })))*
                                #( .item_signal(#enum_fields.signal_cloned().map_none({
                                    let #enum_fields = #enum_fields.clone();
                                    let id = id.clone();
                                    move || {
                                        let show_variants = Mutable::new(false);
                                        Column::new()
                                        .item(
                                          {#button.label(stringify!(#enum_fields).trim_start_matches("r#")).on_press({
                                              let show_variants = show_variants.clone();
                                              move || show_variants.update(|v| !v)
                                          })}
                                        )
                                        .item_signal(show_variants.signal().map_true({
                                            let #enum_fields = #enum_fields.clone();
                                            let id = id.clone();
                                            move || {
                                                use strum::VariantNames;
                                                use std::str::FromStr;

                                                Column::new().items(#enum_fields_types::VARIANTS.iter().map({
                                                    let #enum_fields = #enum_fields.clone();
                                                    let id = id.clone();
                                                    move |&x| {
                                                        #button.label(x).on_press({
                                                            let #enum_fields = #enum_fields.clone();
                                                            let id = id.clone();
                                                            let name = x.to_owned();
                                                            move || {
                                                                let v = Some(#enum_fields_types::from_str(&name).unwrap());
                                                                #[cfg(not(trybuild))]
                                                                {
                                                                    crate::sync_config!(new, format!("{}/{}", id, stringify!(#enum_fields)), serde_json::to_string(&v).unwrap(), { #enum_fields.set(v); });
                                                                }
                                                                #[cfg(trybuild)]
                                                                {
                                                                    #enum_fields.set(v);
                                                                }
                                                            }
                                                        })
                                                    }
                                                }))
                                            }
                                        }))
                                    }
                                })))*
                        }
                    })
                })
            }
        }
    };

    let root_page = {
        let field_pages = primitive_fields
            .iter()
            .chain(&other_fields)
            .chain(&enum_fields)
            .map(|x| format_ident!("{}_page", x));
        quote_spanned! {proc_macro2::Span::call_site()=>
            pub fn root(&self, id: String) -> impl Element {
                Column::new()
                .s(Width::fill())
                .s(Padding::new().left(#current_level))
                .item(El::new().s(Padding::new().left(#current_level + 1)).child(self.add_fields_page(id.clone())))
                #(.item(El::new().s(Padding::new().left(#current_level + 1)).child(self.#field_pages(id.clone()))))*
            }
        }
    };

    quote! {
        use crate::as_element::{Obj, AsElement};

        impl Obj for #s_ident {}
        impl AsElement for #s_ident {
            type EL = El<el::ChildFlagSet>;

            fn as_root_element(&self, id: impl ToString) -> Self::EL {
                El::new().child(self.root(id.to_string()))
            }
        }

        impl #s_ident {
            #fields_output

            #add_fields_page

            #root_page
        }
    }
}

fn enum_variants_derive(e_ident: syn::Ident, e: DataEnum) -> proc_macro2::TokenStream {
    let variants = e.variants.iter().map(|x| x.ident.clone());

    quote! {
        use crate::as_element::{Enu, AsElement};

        impl Enu for #e_ident {}
        impl AsElement for #e_ident {
            type EL = El<el::ChildFlagSet>;

            fn as_root_element(&self, id: impl ToString) -> Self::EL {
                El::new().child(Column::new()
                                .item(Text::new(self.as_ref()))
                                .item(self.root(id.to_string())))
            }
        }

        impl #e_ident {
        pub fn root(&self, id: String) -> impl Element {
            match self {
                #( Self::#variants(ref x) => El::new().child(x.root(id)),)*
            }
        }
        }
    }
}

#[proc_macro_derive(Field)]
pub fn field_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let contents = match data {
        syn::Data::Struct(s) => struct_field_derive(ident, s),
        syn::Data::Enum(e) => enum_variants_derive(ident, e),
        _ => todo!(),
    };

    let output = quote! {
        use zoon::*;
        #contents
    };
    //eprintln!("TOKENS: {}", output);
    output.into()
}

#[proc_macro_attribute]
pub fn ignore_none(_args: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(tokens);

    if let syn::Data::Struct(ref mut s) = input.data {
        if let syn::Fields::Named(syn::FieldsNamed { ref mut named, .. }) = s.fields {
            named.iter_mut().for_each(|f| {
                f.attrs.push(
                    syn::parse_quote!(#[serde(skip_serializing_if = "crate::mutable_is_none")]),
                );
            })
        }
    }

    let output = quote! {
        #input
    };
    //eprintln!("TOKENS: {}", output);
    output.into()
}
