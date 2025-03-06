//! Shows a skin on the page.

use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::skin::SkinData;
use crate::spray::Spray;

#[derive(Params, PartialEq)]
struct ShowParams {
    name: String,
}

/// Shows a skin on the page.
#[component]
pub fn Show(
    skins: impl Into<Signal<im::Vector<SkinData>>>,
    sprays: impl Into<Signal<im::Vector<Spray>>>,
) -> impl IntoView {
    let skins = skins.into();
    let params = use_params::<ShowParams>();

    let skin = Signal::derive(move || {
        params.read().as_ref().ok().and_then(|params| {
            skins.with(|skins| skins.iter().find(|skin| skin.name == params.name).cloned())
        })
    });

    view! {
        <section class="skin-show">
            {move || skin.get().map(|skin| {
                let display_name = skin.realname.replace('_', " ");
                view! {
                    <p>
                        { "To save this: Right-click → Save" }
                        <br/>
                        { "Showing " }
                        <strong>{ display_name }</strong>
                        { "." }
                    </p>
                }
            })}
        </section>
    }
}
