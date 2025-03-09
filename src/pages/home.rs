//! The home page.

use leptos::prelude::*;
use leptos_router::{hooks::use_query, params::Params};

use crate::components::{show::Show, skin_select::SkinSelect, spray_select::SpraySelect};
use crate::spray::Spray;
use crate::SkinWithOptions;

/// Parameters for the URL.
#[derive(Params, PartialEq)]
pub struct PageQuery {
    pub name: String,
}

/// Default Home Page
#[component]
pub fn Home<S, SP>(skins: S, sprays: SP) -> impl IntoView
where
    S: Fn() -> im::HashMap<String, SkinWithOptions> + Clone + Send + Sync + 'static,
    SP: Fn() -> im::Vector<Spray> + Clone + Send + Sync + 'static,
{
    let params = use_query::<PageQuery>();

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

    view! {
        <main>
            <section class="select-menu">
                {
                    let skins = skins.clone();
                    let sprays = sprays.clone();
                    view! { <SkinSelect skins sprays /> }
                }
                {
                    let sprays = sprays.clone();
                    view! {
                        <SpraySelect
                            sprays
                            value
                            on_change
                        />
                    }
                }
            </section>
            {
                let skins = skins.clone();
                let sprays = sprays.clone();
                view! { <Show skins sprays /> }
            }
        </main>
    }
}
