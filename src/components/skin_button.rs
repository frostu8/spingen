//! Displays a skin as a thumbnail.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::doom::patch::Palette;
use crate::image::gen_thumbnail;
use crate::skin::SkinData;
use crate::spray::Spray;

#[component]
pub fn SkinButton(skin: SkinData, spray: impl Into<Signal<Spray>>) -> impl IntoView {
    let spray = spray.into();
    let skin_clone = skin.clone();

    // create display name
    let display_name = skin.realname.replace('_', " ");

    // create thumbnail
    let thumbnail = Signal::derive(move || {
        // get spray color
        let spray = spray.get();

        // remap spray
        let palette = spray.remap(&Palette::default(), skin_clone.startcolor as usize);

        match gen_thumbnail(&skin_clone, &palette) {
            Ok(url) => Some(url),
            Err(err) => {
                leptos::logging::error!("{:?}", err);
                None
            }
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
