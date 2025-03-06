//! The home page.

use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

use crate::components::skin_select::SkinSelect;
use crate::skin::SkinData;
use crate::spray::Spray;

/// Default Home Page
#[component]
pub fn Home(
    skins: impl Fn() -> im::Vector<SkinData> + Send + Sync + 'static,
    sprays: impl Into<Signal<im::Vector<Spray>>>,
) -> impl IntoView {
    let sprays = sprays.into();
    view! {
        <main>
            <SkinSelect skins sprays />
            <Outlet/>
        </main>
    }
}
