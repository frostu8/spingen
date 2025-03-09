//! Spray can selection menu.

use leptos::prelude::*;

use crate::doom::patch::{Palette, Patch};
use crate::image::patch_to_image;
use crate::spray::Spray;

use std::io::Cursor;

use gloo::file::Blob;

use web_sys::{MouseEvent, Url};

use eyre::{Report, WrapErr};

const SPRAYCAN_GRAPHIC: &[u8] = include_bytes!("../SPCNK0.lmp");

/// Spray can selection menu.
#[component]
pub fn SpraySelect<S, F, V>(sprays: S, value: V, on_change: F) -> impl IntoView
where
    S: Fn() -> im::Vector<Spray> + Clone + Send + Sync + 'static,
    F: Fn(Spray) + Clone + Send + Sync + 'static,
    V: Fn() -> Spray + Clone + Send + Sync + 'static,
{
    view! {
        <div class="spray-select">
            <For
                each=move || sprays()
                key=move |spray| spray.id.clone()
                children=move |spray| {
                    let on_change = on_change.clone();

                    let spray_clone = spray.clone();
                    let spray_clone2 = spray.clone();
                    let value = value.clone();

                    view! {
                        <SpraySelectOption
                            spray
                            on_click=move |_| {
                                on_change(spray_clone.clone());
                            }
                            selected=move || value() == spray_clone2
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn SpraySelectOption<F, Sel>(spray: Spray, on_click: F, selected: Sel) -> impl IntoView
where
    F: Fn(MouseEvent) + Send + Sync + 'static,
    Sel: Fn() -> bool + Send + Sync + 'static,
{
    // generate spray palette
    let palette = Palette::default();
    let palette = spray.remap(&palette, 96);

    let gen_url = move || -> Result<String, Report> {
        // load patch
        let patch = Patch::read(Cursor::new(SPRAYCAN_GRAPHIC))
            .wrap_err("failed to read spraycan graphic")?;

        // write to png
        let mut buf = Vec::new();
        patch_to_image(Cursor::new(&mut buf), &patch, &palette)
            .wrap_err("failed to generate spraycan graphic")?;

        let blob = Blob::new_with_options(&buf[..], Some("image/png"));

        Url::create_object_url_with_blob(blob.as_ref())
            .map_err(|_| Report::msg("failed to create object url"))
    };

    match gen_url() {
        Ok(url) => {
            let url_clone = url.clone();
            on_cleanup(move || {
                Url::revoke_object_url(&url_clone).expect("revoke object url");
            });

            view! {
                <button
                    on:click=on_click
                    class=move || {
                        if selected() {
                            "spray-button selected"
                        } else {
                            "spray-button"
                        }
                    }
                    value=spray.id.clone()
                >
                    <img src=url />
                    { spray.name.clone() }
                </button>
            }
            .into_any()
        }
        Err(err) => {
            leptos::logging::error!("{:?}", err);

            view! {
                <button
                    on:click=on_click
                    class="spray-button"
                    value=spray.id.clone()
                >
                    { spray.name.clone() }
                </button>
            }
            .into_any()
        }
    }
}
