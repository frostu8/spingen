//! Spray can selection menu.

use leptos::prelude::*;

use crate::doom::patch::{Palette, Patch};
use crate::image::patch_to_image;
use crate::spray::Spray;

use std::io::Cursor;

use gloo::file::Blob;

use web_sys::Url;

use eyre::{Report, WrapErr};

const SPRAYCAN_GRAPHIC: &[u8] = include_bytes!("../SPCNK0.lmp");

/// Spray can selection menu.
#[component]
pub fn SpraySelect<S, F, V>(sprays: S, value: V, on_change: F) -> impl IntoView
where
    S: Fn() -> im::Vector<Spray> + Clone + Send + Sync + 'static,
    F: Fn(Spray) + Send + Sync + 'static,
    V: Fn() -> Spray + Send + Sync + 'static,
{
    let sprays_clone = sprays.clone();
    view! {
        <select
            on:change:target=move |ev| {
                let sprays = sprays_clone();
                let spray_id = ev.target().value();

                if let Some(spray) = sprays
                    .iter()
                    .find(|s| s.id == spray_id)
                    .cloned()
                {
                    on_change(spray);
                }
            }
            prop:value=move || value().name.clone()
            size="6"
        >
            <For
                each=move || sprays()
                key=move |spray| spray.id.clone()
                children=move |spray| {
                    view! { <SpraySelectOption spray /> }
                }
            />
        </select>
    }
}

#[component]
fn SpraySelectOption(spray: Spray) -> impl IntoView {
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
                <option class="spray-button" value=spray.id.clone()>
                    <img src=url />
                    <p>{ spray.name.clone() }</p>
                </option>
            }
            .into_any()
        }
        Err(err) => {
            leptos::logging::error!("{:?}", err);

            view! {
                <option value=spray.id.clone()>{ spray.name.clone() }</option>
            }
            .into_any()
        }
    }
}
