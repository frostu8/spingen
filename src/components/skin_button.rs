//! Displays a skin as a thumbnail.

use leptos::prelude::*;

use crate::skin::Skin;

#[component]
pub fn SkinButton(skin: Skin) -> impl IntoView {
    view! {
        <button class="skin-button">
            {
                skin.thumbnail_url()
                    .map(|src| view! { <img src=src /> })
            }
            <p>{ skin.display_name() }</p>
        </button>
    }
}
