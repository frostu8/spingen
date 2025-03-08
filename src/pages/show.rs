//! Shows a skin on the page.

use leptos::control_flow::Show as ControlShow;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::{
    frame_select::FrameSelect, skin_show::SkinShow, sprite_select::SpriteSelect,
};
use crate::spray::Spray;
use crate::SkinWithOptions;

use wad::Name;

/// Parameters for [`Show`].
#[derive(Params, PartialEq)]
pub struct ShowParams {
    pub name: String,
}

/// Shows a skin on the page.
#[component]
pub fn Show<S, SP>(skins: S, sprays: SP) -> impl IntoView
where
    S: Fn() -> im::HashMap<String, SkinWithOptions> + Send + Sync + 'static,
    SP: Fn() -> im::Vector<Spray> + Send + Sync + 'static,
{
    let params = use_params::<ShowParams>();

    let skin = Signal::derive(move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| skins().get(&params.name).cloned())
    });

    let (name, set_name) = signal(Name::from_bytes(b"STIN").expect("valid name"));
    let (frame, set_frame) = signal(b'A');

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
                    spray=move || spray.get().expect("valid spray")
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
