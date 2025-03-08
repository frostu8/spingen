//! A html drop area.
//!
//! The [`DropArea`] components wraps the entire viewport, and accepts files
//! from the user.

use leptos::prelude::*;

use gloo::file::File;

use leptos_use::{
    use_drop_zone_with_options, UseDropZoneEvent, UseDropZoneOptions, UseDropZoneReturn,
};

/// A drop area.
#[component]
pub fn DropArea<F>(on_file: F) -> impl IntoView
where
    F: Fn(File) + Send + Sync + 'static,
{
    let drop_zone_ref = NodeRef::new();

    let (enabled, set_enabled) = signal(true);

    let on_drop = move |ev: UseDropZoneEvent| {
        set_enabled(false);

        for file in ev.files {
            on_file(File::from(file))
        }
    };

    let UseDropZoneReturn {
        is_over_drop_zone: _,
        ..
    } = use_drop_zone_with_options(
        drop_zone_ref,
        UseDropZoneOptions::default().on_drop(on_drop),
    );

    view! {
        <div
            id="app-drop-area"
            class={ move || if enabled.get() { "enabled" } else { "" } }
            node_ref=drop_zone_ref
        >
        </div>
    }
}
