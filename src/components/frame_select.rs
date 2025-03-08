//! Sprite selection menu.

use leptos::prelude::*;

use crate::skin::Skin;

use wad::Name;

/// A sprite selection menu.
#[component]
pub fn FrameSelect<S, SP, C, V>(skin: S, sprite_name: SP, on_change: C, value: V) -> impl IntoView
where
    S: Fn() -> Skin + Send + Sync + 'static,
    SP: Fn() -> Name + Send + Sync + 'static,
    C: Fn(u8) + Send + Sync + 'static,
    V: Fn() -> u8 + Send + Sync + 'static,
{
    let frames = Memo::new(move |_| {
        let skin = skin();
        let name = sprite_name();

        let mut frames = skin.iter_frames(&name).collect::<Vec<_>>();
        frames.sort();
        frames
    });

    view! {
        <select
            on:change:target=move |ev| {
                on_change(ev.target().value().parse().unwrap());
            }
            prop:value=move || value().to_string()
        >
            <For
                each=move || frames.get()
                key=move |k| *k
                children=move |k| {
                    view! {
                        <option value=k.to_string()>{ (k as char).to_string() }</option>
                    }
                }
            />
        </select>
    }
}
