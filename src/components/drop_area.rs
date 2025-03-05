//! A html drop area.
//!
//! The [`DropArea`] components wraps the entire viewport, and accepts files
//! from the user.

use leptos::prelude::*;

/// A drop area.
#[component]
pub fn DropArea() -> impl IntoView {
    view! {
        <div id="app-drop-area"></div>
    }
}
