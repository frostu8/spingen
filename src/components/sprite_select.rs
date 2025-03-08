//! Sprite selection menu.

use ahash::HashSet;
use eyre::WrapErr;
use leptos::prelude::*;

use crate::skin::Skin;

use std::cmp::min;

use wad::Name;

/// A sprite selection menu.
#[component]
pub fn SpriteSelect<S, C, V>(skin: S, on_change: C, value: V) -> impl IntoView
where
    S: Fn() -> Skin + Send + Sync + 'static,
    C: Fn(Name) + Send + Sync + 'static,
    V: Fn() -> Name + Send + Sync + 'static,
{
    let sprites = Memo::new(move |_| {
        let skin = skin();

        let sprites = match skin.list().wrap_err("failed to get sprite list") {
            Ok(sprites) => sprites,
            Err(err) => {
                leptos::logging::error!("{:?}", err);
                vec![]
            }
        };

        let mut sprites = sprites
            .into_iter()
            .map(|name| {
                let len = min(4, name.as_str().len());
                Name::from_bytes(&name[..len]).expect("valid subname")
            })
            // deduplicate
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        sprites.sort();
        sprites
    });

    view! {
        <select
            on:change:target=move |ev| {
                on_change(ev.target().value().parse().unwrap());
            }
            prop:value=move || value().to_string()
        >
            <For
                each=move || sprites.get()
                key=move |k| *k
                children=move |k| {
                    view! {
                        <option value=k.to_string()>{ k.as_str()[..4].to_string() }</option>
                    }
                }
            />
        </select>
    }
}
