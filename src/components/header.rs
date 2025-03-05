//! Site-wide header.
//!
//! Also handles file drags.

use leptos::prelude::*;

use crate::components::drop_area::DropArea;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <DropArea/>
        <header>
            <h3>spingen</h3>
            <p>
                { "Show off your racer!" }
                <br/>
                { "Drag and drop a pk3, wad, or zip." }
                <br/>
                <strong>{ "Note:" }</strong> { " Only supports vanilla colors (for now)" }
            </p>
        </header>
    }
}
