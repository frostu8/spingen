//! Shows a skin as a gif.

use leptos::prelude::*;

use crate::image::Encoder;
use crate::skin::Skin;
use crate::spray::Spray;

use eyre::{Report, WrapErr};

use std::io::Cursor;

use wad::Name;

use gloo::file::Blob;

use web_sys::Url;

/// Shows a skin as a gif.
#[component]
pub fn SkinShow<S, SP, N>(skin: S, spray: SP, name: N) -> impl IntoView
where
    S: Fn() -> Skin + Send + Sync + 'static,
    SP: Fn() -> Spray + Send + Sync + 'static,
    N: Fn() -> (Name, u8) + Send + Sync + 'static,
{
    let memoized_spray = Memo::new(move |_| spray());

    // create object url
    let img_src = Memo::<String>::new(move |old_img_src| {
        let spray = memoized_spray.get();

        let skin = skin();
        let (name, frame) = name();

        // free old src
        if let Some(src) = old_img_src {
            Url::revoke_object_url(&src).expect("object revoke");
        }

        let gen_gif = move || -> Result<String, Report> {
            let mut encoder = Encoder::new(&skin).with_spray(&spray);

            // generate new gif
            let mut buf = Vec::new();
            encoder
                .sprite_gif(Cursor::new(&mut buf), name, frame)
                .wrap_err("failed to encode gif")?;

            let blob = Blob::new_with_options(&buf[..], Some("image/gif"));

            Url::create_object_url_with_blob(blob.as_ref())
                .map_err(|_| Report::msg("failed to create object url"))
        };

        match gen_gif() {
            Ok(url) => url,
            Err(err) => {
                leptos::logging::error!("{:?}", err);
                String::from("")
            }
        }
    });

    view! {
        <img src=img_src />
    }
}
