//! Shows a skin on the page.

use leptos::control_flow::Show as ControlShow;
use leptos::prelude::*;
use leptos_router::hooks::use_query;

use crate::components::{
    frame_select::FrameSelect, skin_show::SkinShow, sprite_select::SpriteSelect,
};
use crate::pages::home::PageQuery;
use crate::spray::Spray;
use crate::SkinWithOptions;

use wad::Name;

/// Shows a skin on the page.
#[component]
pub fn Show<S, SP>(skins: S, sprays: SP) -> impl IntoView
where
    S: Fn() -> im::HashMap<String, SkinWithOptions> + Send + Sync + 'static,
    SP: Fn() -> im::Vector<Spray> + Send + Sync + 'static,
{
    let params = use_query::<PageQuery>();

    let skin = Signal::derive(move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| skins().get(&params.name).cloned())
    });

    let (name, set_name) = signal(Name::from_bytes(b"STIN").expect("valid name"));
    let (frame, set_frame) = signal(b'A');

    // reset frame and name if out-of-bounds
    let (old_skin, set_old_skin) = signal(None);
    Effect::new(move || {
        let Some(skin) = skin.get() else {
            return;
        };
        let old_skin = old_skin.get();

        if old_skin.is_none() {
            // update frame stuff on the next tic to resynch the UI
            set_name(Name::from_bytes(b"STIN").expect("valid name"));
            set_frame(b'A');
        }

        if Some(&skin.name) != old_skin.as_ref() {
            set_old_skin.set(Some(skin.name.clone()));

            // update frame name
            let name = name.get_untracked();
            if !skin.iter().any(|frame| frame == name) {
                // reset
                set_name(Name::from_bytes(b"STIN").expect("valid name"));
            }

            // update frame
            let frame = frame.get_untracked();
            if !skin
                .iter_frames(&name)
                .any(|inner_frame| inner_frame == frame)
            {
                // reset
                set_frame(b'A');
            }
        }
    });

    let spray = Signal::derive(move || {
        let sprays = sprays();

        let Some(skin) = skin.get() else {
            return None;
        };

        if let Some(spray) = skin.spray.get() {
            Some(spray)
        } else {
            sprays
                .iter()
                .find(|spray| spray.name.eq_ignore_ascii_case(&skin.prefcolor))
                .cloned()
        }
    });

    view! {
        <section class="skin-show">
            <ControlShow
                when=move || skin.with(|skin| skin.is_some())
            >
                <SkinShow
                    skin=move || skin.get().expect("valid skin").skin
                    spray=move || spray.get().unwrap_or_default()
                    name=move || (name.get(), frame.get())
                />
                <div class="skin-show-controls">
                    <SpriteSelect
                        skin=move || skin.get().expect("valid skin").skin
                        on_change=move |new_name| {
                            let skin = skin.get_untracked().expect("valid skin");
                            let frame = skin
                                .iter_frames(&new_name)
                                .reduce(|a, b| std::cmp::min(a, b))
                                .unwrap_or(b'A');

                            set_name(new_name);
                            set_frame(frame);
                        }
                        value=move || name.get()
                    />
                    <FrameSelect
                        skin=move || skin.get().expect("valid skin").skin
                        sprite_name=move || name.get()
                        on_change=move |new_frame| set_frame(new_frame)
                        value=move || frame.get()
                    />
                    <button
                        on:click=move |_| {
                            skin.get().expect("valid skin").spray.set(None);
                        }
                    >
                        { "Use Preferred Spray" }
                    </button>
                </div>
                <p>
                    { "To save this: Right-click → Save Image As" }
                    <br/>
                    { "Showing " }
                    <strong>
                        { move || skin.with(|skin| skin.as_ref().unwrap().realname.replace('_', " ")) }
                    </strong>
                    { "." }
                </p>
            </ControlShow>
        </section>
    }
}
