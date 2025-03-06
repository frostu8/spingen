//! The skin select menu.

use leptos::prelude::*;

use crate::components::skin_button::SkinButton;
use crate::skin::SkinData;
use crate::spray::Spray;

/// The skin select menu.
#[component]
pub fn SkinSelect(
    skins: impl Fn() -> im::Vector<SkinData> + Send + Sync + 'static,
    sprays: impl Into<Signal<im::Vector<Spray>>>,
) -> impl IntoView {
    let sprays = sprays.into();
    view! {
        <section class="skin-container">
            <For
                each=move || skins().into_iter()
                key=move |skin| skin.name.clone()
                children=move |skin| {
                    let prefcolor = skin.prefcolor.clone();
                    let spray = Signal::derive(move || {
                        sprays
                            .with(|sprays| sprays
                            .iter()
                            .find(|spray| spray.name.eq_ignore_ascii_case(&prefcolor))
                            .cloned()
                            .unwrap_or_default())
                    });
                    view! { <SkinButton skin spray/> }
                }
            />
        </section>
    }
}
