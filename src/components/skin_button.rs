//! Displays a skin as a thumbnail.

use leptos::prelude::*;
use leptos_router::components::A;
use web_sys::Url;

use crate::image::Encoder;
use crate::skin::Skin;
use crate::spray::Spray;

use gloo::file::Blob;

use std::io::Cursor;

use wad::Name;

use eyre::{Report, WrapErr};

#[component]
pub fn SkinButton(skin: Skin, spray: impl Into<Signal<Spray>>) -> impl IntoView {
    let spray = spray.into();
    let skin_clone = skin.clone();

    // create display name
    let display_name = skin.realname.replace('_', " ");

    // create thumbnail
    let thumbnail = Signal::derive(move || {
        // get spray color
        let spray = spray.get();

        // create encoder
        let gen_thumbnail = || -> Result<String, Report> {
            let mut encoder = Encoder::new(&skin_clone).with_spray(&spray);

            // try to find asymmetric sprite first
            let mut buf = Vec::new();
            encoder
                .sprite(
                    Cursor::new(&mut buf),
                    "STINA2".parse::<Name>().expect("valid name"),
                )
                .or_else(|err| {
                    if err.not_found() {
                        // try to get other sprite
                        encoder.sprite(
                            Cursor::new(&mut buf),
                            "STINA2A8".parse::<Name>().expect("valid name"),
                        )
                    } else {
                        Err(err)
                    }
                })
                .wrap_err("failed to encode thumbnail")?;

            let blob = Blob::new_with_options(&buf[..], Some("image/png"));

            Url::create_object_url_with_blob(blob.as_ref())
                .map_err(|_| Report::msg("failed to create object url"))
        };

        match gen_thumbnail() {
            Ok(url) => Some(url),
            Err(err) => {
                leptos::logging::error!("{:?}", err);
                None
            }
        }
    });

    on_cleanup(move || {
        if let Some(thumbnail_url) = thumbnail.get_untracked() {
            Url::revoke_object_url(&thumbnail_url).expect("revoke object url");
        }
    });

    view! {
        <A attr:class="skin-button btn" href=skin.name.clone()>
            {
                thumbnail
                    .get()
                    .map(|src| view! { <img src=src /> })
            }
            <p>{ display_name }</p>
        </A>
    }
}
