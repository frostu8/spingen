//! Site-wide header.
//!
//! Also handles file drags.

use leptos::prelude::*;

//use crate::components::drop_area::DropArea;

use web_sys::HtmlInputElement;

use gloo::file::File;

#[component]
pub fn Header(on_file: impl Fn(File) + Send + Sync + 'static) -> impl IntoView {
    let file_dialog = NodeRef::new();

    let on_input_file = move |_| {
        // read from file
        let dialog: HtmlInputElement = file_dialog.get().expect("input node should exist");
        let Some(files) = dialog.files() else {
            return;
        };

        for i in 0..files.length() {
            let file = files.get(i).expect("in bounds");
            let file = File::from(file);
            on_file(file);
        }
    };

    view! {
        //<DropArea/>
        <header>
            <h3>spingen</h3>
            <p>
                { "Show off your racer!" }
                <br/>
                { "Drag and drop a pk3, or " }
                <button
                    class="link-button"
                    on:click=move |_ev| { file_dialog.get().expect("input node should exist").click() }
                >
                    { "click me" }
                </button>
                { "." }
                <br/>
                <strong>{ "Note:" }</strong> { " Only supports vanilla colors (for now)" }
            </p>
            <input
                type="file"
                accept=".pk3"
                multiple="true"
                class="hidden"
                on:change=on_input_file
                node_ref=file_dialog
            />
        </header>
    }
}
