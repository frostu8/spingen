#![feature(once_cell_try)]

pub mod components;
pub mod doom;
pub mod image;
pub mod pages;
pub mod skin;
pub mod spray;

use crate::components::header::Header;
use crate::pages::{home::Home, show::Show};
use crate::skin::{loader::Pk3Loader, Skin};
use crate::spray::sprays;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use gloo::file::{futures::read_as_bytes, File};

use eyre::{Report, WrapErr};

use bytes::Bytes;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // the list of loaded sprays
    let (sprays, set_sprays) = signal(sprays());

    // the raw skin datas, this will be used to create the normal skin datas
    let (skins, set_skins) = signal(im::Vector::<Skin>::new());
    let on_file = move |file: File| {
        // try to load file
        wasm_bindgen_futures::spawn_local(async move {
            match load_pk3(file).await {
                // merge with list
                Ok(skins) => set_skins.update(move |data| data.extend(skins)),
                Err(err) => {
                    // TODO: show error to user
                    leptos::logging::error!("{:?}", err);
                }
            }
        });
    };

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="spingen" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Header on_file/>

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <ParentRoute path=path!("/") view=move || view! { <Home skins sprays /> }>
                    <Route path=path!(":name") view=move || view! { <Show skins sprays /> } />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

async fn load_pk3(file: File) -> Result<Vec<Skin>, Report> {
    // read all data
    let data = read_as_bytes(&file)
        .await
        .map(|bytes| Bytes::from(bytes))
        .wrap_err_with(|| format!("failed to read file {:?}", file.name()))?;

    // open zip file
    let loader = Pk3Loader::new(data).wrap_err("failed to read pk3")?;

    Ok(loader.iter().collect())
}
