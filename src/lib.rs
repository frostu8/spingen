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
use crate::skin::{loaders::Pk3SkinLoader, Skin};
use crate::spray::{sprays, Spray};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use derive_more::{Deref, Display, Error, From};

use std::io;

use gloo::file::{futures::read_as_bytes, File};

use eyre::Report;

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
            let data = match read_as_bytes(&file).await.map(|bytes| Bytes::from(bytes)) {
                Ok(data) => data,
                Err(err) => {
                    // TODO show to user
                    leptos::logging::error!(
                        "{:?}",
                        Report::from(err).wrap_err("failed reading file")
                    );
                    return;
                }
            };

            let pk3 = match Pk3SkinLoader::new(data) {
                Ok(pk3) => pk3,
                Err(err) => {
                    leptos::logging::error!(
                        "{:?}",
                        Report::from(err).wrap_err("failed reading pk3")
                    );
                    return;
                }
            };
            let new_skins = pk3
                .filter_map(|skin| match skin {
                    Ok(skin) => Some(skin),
                    Err(err) => {
                        leptos::logging::error!(
                            "{:?}",
                            Report::from(err).wrap_err("failed reading skin")
                        );
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !new_skins.is_empty() {
                set_skins_raw.update(|skins| skins.extend(new_skins));
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
