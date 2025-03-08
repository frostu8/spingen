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
pub fn DropArea<F, T>(on_file: F, children: TypedChildrenFn<T>) -> impl IntoView
where
    F: Fn(File) + Send + Sync + 'static,
    T: IntoView + 'static,
{
    let drop_zone_ref = NodeRef::new();

    let on_drop = move |ev: UseDropZoneEvent| {
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

    let children = children.into_inner();

    view! {
        <div node_ref=drop_zone_ref>
            {children()}
        </div>
    }
}
