#![feature(once_cell_try)]

pub mod components;
pub mod doom;
pub mod image;
pub mod lump;
pub mod pages;
pub mod skin;
pub mod spray;

use crate::components::header::Header;
use crate::pages::home::Home;
use crate::skin::Skin;
use crate::spray::{sprays, Spray};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use derive_more::{Deref, Display, Error, From};

use std::io;

use gloo::file::{futures::read_as_bytes, File};

use eyre::{Report, WrapErr};

use bytes::Bytes;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // the list of loaded sprays
    let (sprays, _set_sprays) = signal(sprays());

    // the raw skin datas, this will be used to create the normal skin datas
    let (skins_raw, set_skins_raw) = signal(im::Vector::<Skin>::new());
    let on_file = move |file: File| {
        // try to load file
        wasm_bindgen_futures::spawn_local(async move {
            match load_pk3(file).await {
                // merge with list
                Ok(skins) => set_skins_raw.update(move |data| data.extend(skins)),
                Err(err) => {
                    // TODO: show error to user
                    leptos::logging::error!("{:?}", err);
                }
            }
        });
    };

    let skins = Memo::<im::HashMap<String, SkinWithOptions>>::new_owning(move |old_skins| {
        let mut skins = old_skins.unwrap_or_default();

        for skin in skins_raw.get() {
            if !skins.contains_key(&skin.name) {
                skins.insert(
                    skin.name.clone(),
                    SkinWithOptions {
                        skin,
                        spray: ArcRwSignal::new(None),
                    },
                );
            }
        }

        (skins, true)
    });

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="spin.ringrace.rs" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Header on_file/>

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=move || view! { <Home skins sprays /> } />
            </Routes>
        </Router>
    }
}

/// A skin with reactive options.
#[derive(Clone, Debug, Deref)]
pub struct SkinWithOptions {
    #[deref]
    skin: Skin,
    pub spray: ArcRwSignal<Option<Spray>>,
}

async fn load_pk3(file: File) -> Result<Vec<Skin>, Report> {
    // read all data
    let data = read_as_bytes(&file)
        .await
        .map(|bytes| Bytes::from(bytes))
        .wrap_err_with(|| format!("failed to read file {:?}", file.name()))?;

    // open zip file
    let loader = crate::skin::loader::load_pk3(data).wrap_err("failed to read pk3")?;

    Ok(loader.into_iter().map(|(_, v)| v).collect())
}

/// Loader errors.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display("malformed pk3: {_0}")]
    Zip(zip::result::ZipError),
    Io(io::Error),
    Patch(crate::doom::patch::Error),
    Name(skin::FromNameError),
    #[display("soc {_0:?}: {_1}")]
    #[from(ignore)]
    Deser(String, crate::doom::soc::Error),
    #[display("sprite \"{_0}\" not found")]
    NotFound(#[error(not(source))] String),
}

impl Error {
    /// Checks if the error is a not found error.
    pub fn not_found(&self) -> bool {
        matches!(self, Error::NotFound(..))
    }
}
