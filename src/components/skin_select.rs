//! The skin select menu.

use leptos::prelude::*;

use crate::components::skin_button::SkinButton;
use crate::spray::Spray;
use crate::SkinWithOptions;

/// The skin select menu.
#[component]
pub fn SkinSelect(
    skins: impl Fn() -> im::HashMap<String, SkinWithOptions> + Send + Sync + 'static,
    sprays: impl Fn() -> im::Vector<Spray> + Clone + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <section class="skin-container">
            <For
                each=move || skins().into_iter().map(|(_, v)| v)
                key=move |skin| skin.name.clone()
                children=move |skin| {
                    let prefcolor = skin.prefcolor.clone();
                    let sprays = sprays.clone();

                    view! {
                        <SkinButton
                            skin=skin.skin.clone()
                            spray=move || {
                                if let Some(spray) = skin.spray.get() {
                                    spray
                                } else {
                                    sprays()
                                        .iter()
                                        .find(|spray| spray.name.eq_ignore_ascii_case(&prefcolor))
                                        .cloned()
                                        .unwrap_or_default()
                                }
                            }
                        />
                    }
                }
            />
        </section>
    }
}
