//! The home page.

use leptos::prelude::*;
use leptos_router::{hooks::use_params, nested_router::Outlet};

use crate::components::{skin_select::SkinSelect, spray_select::SpraySelect};
use crate::pages::show::ShowParams;
use crate::spray::Spray;
use crate::SkinWithOptions;

/// Default Home Page
#[component]
pub fn Home<S, SP>(skins: S, sprays: SP) -> impl IntoView
where
    S: Fn() -> im::HashMap<String, SkinWithOptions> + Clone + Send + Sync + 'static,
    SP: Fn() -> im::Vector<Spray> + Clone + Send + Sync + 'static,
{
    let params = use_params::<ShowParams>();

    let sprays_clone = sprays.clone();
    let skins_clone = skins.clone();
    let value = move || {
        if let Some(skin) = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| skins_clone().get(&params.name).cloned())
        {
            skin.spray
                .get()
                .or_else(|| {
                    sprays_clone()
                        .iter()
                        .find(|spray| spray.name.eq_ignore_ascii_case(&skin.prefcolor))
                        .cloned()
                })
                .unwrap_or_default()
        } else {
            Spray::default()
        }
    };

    let skins_clone = skins.clone();
    let on_change = move |spray| {
        if let Some(skin) = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| skins_clone().get(&params.name).cloned())
        {
            skin.spray.set(Some(spray));
        }
    };

    let sprays_clone = sprays.clone();

    view! {
        <main>
            <section class="select-menu">
                <SkinSelect skins sprays />
                <SpraySelect
                    sprays=sprays_clone
                    value
                    on_change
                />
            </section>
            <Outlet/>
        </main>
    }
}
