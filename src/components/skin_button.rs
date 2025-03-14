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
pub fn SkinButton<SP>(skin: Skin, spray: SP) -> impl IntoView
where
    SP: Fn() -> Spray + Send + Sync + 'static,
{
    let skin_clone = skin.clone();

    // create display name
    let display_name = skin.realname.replace('_', " ");

    // create thumbnail
    let thumbnail_src = Memo::<String>::new(move |old_src| {
        // free old src
        if let Some(src) = old_src {
            Url::revoke_object_url(&src).expect("object revoke");
        }

        // get spray color
        let spray = spray();

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
            Ok(url) => url,
            Err(err) => {
                leptos::logging::error!("{:?}", err);
                String::from("")
            }
        }
    });

    on_cleanup(move || {
        let thumbnail_url = thumbnail_src.get_untracked();
        Url::revoke_object_url(&thumbnail_url).expect("revoke object url");
    });

    view! {
        <A
            attr:class="skin-button btn"
            href=format!("/?name={}", skin.name.clone())
        >
            <img src=thumbnail_src />
            <p>{ display_name }</p>
        </A>
    }
}
