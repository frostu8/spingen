//! Site-wide header.
//!
//! Also handles file drags.

use leptos::prelude::*;

use web_sys::HtmlInputElement;

//use crate::components::drop_area::DropArea;

use gloo::file::File;

#[component]
pub fn Header<F>(on_file: F) -> impl IntoView
where
    F: Fn(File) + Clone + Send + Sync + 'static,
{
    let file_dialog = NodeRef::new();

    let on_file_clone = on_file.clone();
    let on_input_file = move |_| {
        // read from file
        let dialog: HtmlInputElement = file_dialog.get().expect("input node should exist");
        let Some(files) = dialog.files() else {
            return;
        };

        for i in 0..files.length() {
            let file = files.get(i).expect("in bounds");
            let file = File::from(file);
            on_file_clone(file);
        }
    };

    view! {
        <header>
            <h3>spingen</h3>
            <p>
                { "Show off your racer!" }
                <br/>
                <button
                    class="link-button"
                    on:click=move |_ev| { file_dialog.get().expect("input node should exist").click() }
                >
                    { "Click here" }
                </button>
                { " to load a pk3."}
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
