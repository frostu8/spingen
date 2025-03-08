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
                        <option value=k.to_string()>{ as_human_readable(&k) }</option>
                    }
                }
            />
        </select>
    }
}

fn as_human_readable(name: &Name) -> String {
    match name.as_str() {
        "STIN" => String::from("Still"),
        "STIL" => String::from("Still Left"),
        "STIR" => String::from("Still Right"),
        "STGL" => String::from("Still Left (glance back)"),
        "STGR" => String::from("Still Right (glance back)"),
        "STLL" => String::from("Still Left (look back)"),
        "STLR" => String::from("Still Right (look back)"),
        "SLWN" => String::from("Slow Driving"),
        "SLWL" => String::from("Slow Driving Left"),
        "SLWR" => String::from("Slow Driving Right"),
        "SLGL" => String::from("Slow Driving Left (glance back)"),
        "SLGR" => String::from("Slow Driving Right (glance back)"),
        "SLLL" => String::from("Slow Driving Left (look back)"),
        "SLLR" => String::from("Slow Driving Right (look back)"),
        "FSTN" => String::from("Fast Driving"),
        "FSTL" => String::from("Fast Driving Left"),
        "FSTR" => String::from("Fast Driving Right"),
        "FSGL" => String::from("Fast Driving Left (glance back)"),
        "FSGR" => String::from("Fast Driving Right (glance back)"),
        "FSLL" => String::from("Fast Driving Left (look back)"),
        "FSLR" => String::from("Fast Driving Right (look back)"),
        "DRLN" => String::from("Drifting Left, Steering Neutral"),
        "DRLO" => String::from("Drifting Left, Steering Outwards"),
        "DRLI" => String::from("Drifting Left, Steering Inwards"),
        "DRRN" => String::from("Drifting Right, Steering Neutral"),
        "DRRO" => String::from("Drifting Right, Steering Outwards"),
        "DRRI" => String::from("Drifting Right, Steering Inwards"),
        "SPIN" => String::from("Spinout"),
        "DEAD" => String::from("Dead"),
        "SIGN" => String::from("Finish Signpost"),
        "SIGL" => String::from("Finish Signpost, Ironman Perfect"),
        "SSIG" => String::from("\"working designs\" Signpost"),
        "XTRA" => String::from("Wanted"),
        "TALK" => String::from("Dialogue Icon"),
        // wtf is this spr2
        name => String::from(name),
    }
}
