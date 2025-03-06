//! The home page.

use leptos::prelude::*;

use crate::skins::Skin;

/// Default Home Page
#[component]
pub fn Home(skin_list: ReadSignal<im::HashMap<String, Skin>>) -> impl IntoView {
    view! {
        <For
            each=move || skin_list.get()
            key=move |(_, skin)| skin.name.clone()
            children=move |(_, skin)| view! { <p>{ skin.name.clone() }</p> }
        />
        <p>{ "Todo" }</p>
    }
}
