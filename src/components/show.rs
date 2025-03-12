//! Shows a skin on the page.

use leptos::control_flow::Show as ControlShow;
use leptos::prelude::*;
use leptos_router::hooks::use_query;

use std::str::FromStr;

use crate::components::{
    frame_select::FrameSelect, skin_show::SkinShow, sprite_select::SpriteSelect,
};
use crate::doom::skin::SkinDefine;
use crate::image::GifOptions;
use crate::pages::home::PageQuery;
use crate::spray::Spray;
use crate::SkinWithOptions;

use std::cmp::{max, min};

use derive_more::{Display, Error};

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
    let (scale, set_scale) = signal(Scale::Times1);

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
                    options=move || {
                        GifOptions {
                            scale: match scale.get() {
                                Scale::Times1 => 1.0,
                                Scale::Times2 => 2.0,
                                Scale::Times3 => 3.0,
                                Scale::Times4 => 4.0,
                                Scale::Times6 => 6.0,
                                Scale::Times8 => 8.0,
                            },
                            ..Default::default()
                        }
                    }
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
                    <select
                        on:change:target=move |ev| {
                            set_scale(ev.target().value().parse().expect("valid scale"));
                        }
                        prop:value=move || scale.get().to_string()
                    >
                        {
                            Scale::iter()
                                .map(|scale| view! {
                                    <option value=scale.to_string()>{ scale.to_string() }</option>
                                })
                                .collect::<Vec<_>>()
                        }
                    </select>
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
                    { ", a "}
                    <strong>
                        { move || skin.with(|skin| class_from_skin(skin.as_ref().unwrap())) }
                    </strong>
                    { " driver." }
                </p>
            </ControlShow>
        </section>
    }
}

fn class_from_skin(skin: &SkinDefine) -> &'static str {
    const CLASSES: &[&str] = &[
        "Class A", "Class B", "Class C", "Class D", "Class E", "Class F", "Class G", "Class H",
        "Class I",
    ];

    let x = min((max(skin.kartspeed, 1) as usize - 1) / 3, 2);
    let y = min((max(skin.kartweight, 1) as usize - 1) / 3, 2);

    CLASSES[y * 3 + x]
}

#[derive(Debug, Clone, Copy, Display)]
enum Scale {
    #[display("x1")]
    Times1,
    #[display("x2")]
    Times2,
    #[display("x3")]
    Times3,
    #[display("x4")]
    Times4,
    #[display("x6")]
    Times6,
    #[display("x8")]
    Times8,
}

impl Scale {
    /// Iterates over all possible values of [`Scale`].
    pub fn iter() -> impl Iterator<Item = Scale> + ExactSizeIterator {
        [
            Scale::Times1,
            Scale::Times2,
            Scale::Times3,
            Scale::Times4,
            Scale::Times6,
            Scale::Times8,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Display, Error)]
#[display("not a scale")]
struct NotAScaleError;

impl FromStr for Scale {
    type Err = NotAScaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x1" => Ok(Scale::Times1),
            "x2" => Ok(Scale::Times2),
            "x3" => Ok(Scale::Times3),
            "x4" => Ok(Scale::Times4),
            "x6" => Ok(Scale::Times6),
            "x8" => Ok(Scale::Times8),
            _ => Err(NotAScaleError),
        }
    }
}
