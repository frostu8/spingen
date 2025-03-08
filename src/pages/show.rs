//! Shows a skin on the page.

use leptos::control_flow::Show as ControlShow;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::skin_show::SkinShow;
use crate::skin::Skin;
use crate::spray::Spray;

use wad::Name;

#[derive(Params, PartialEq)]
struct ShowParams {
    name: String,
}

/// Shows a skin on the page.
#[component]
pub fn Show(
    skins: impl Into<Signal<im::Vector<Skin>>>,
    sprays: impl Into<Signal<im::Vector<Spray>>>,
) -> impl IntoView {
    let skins = skins.into();
    let sprays = sprays.into();

    let params = use_params::<ShowParams>();

    let skin = Signal::derive(move || {
        params.read().as_ref().ok().and_then(|params| {
            skins.with(|skins| skins.iter().find(|skin| skin.name == params.name).cloned())
        })
    });

    let (spray, set_spray) = signal(Spray::default());
    let (name, _set_name) = signal(Name::from_bytes(b"STINA").expect("valid name"));

    // create an effect for initialization
    Effect::new(move |_| {
        let sprays = sprays.get();

        let spray_name = skin.with(|skin| skin.as_ref().map(|s| s.prefcolor.to_owned()));
        if let Some(spray) = spray_name
            .and_then(|name| {
                sprays
                    .iter()
                    .find(|spray| spray.name.eq_ignore_ascii_case(&name))
            })
            .cloned()
        {
            set_spray(spray);
        }
    });

    view! {
        <section class="skin-show">
            <ControlShow
                when=move || skin.with(|skin| skin.is_some())
            >
                <SkinShow
                    skin={ move || skin.get().expect("valid skin") }
                    spray={ move || spray.get() }
                    name
                />
                <p>
                    { "To save this: Right-click → Save Image As" }
                    <br/>
                    { "Showing " }
                    <strong>
                        { move || skin.with(|skin| skin.as_ref().unwrap().realname.replace('_', " ")) }
                    </strong>
                    { "." }
                </p>
            </ControlShow>
        </section>
    }
}
