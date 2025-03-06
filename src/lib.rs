pub mod components;
pub mod doom;
pub mod image;
pub mod pages;
pub mod skin;
pub mod spray;

use crate::components::header::Header;
use crate::doom::{skin::SkinDefine, soc};
use crate::pages::{home::Home, show::Show};
use crate::skin::SkinData;
use crate::spray::sprays;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use gloo::file::{futures::read_as_bytes, File};

use std::io::{Cursor, Read as _};
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use eyre::{OptionExt, Report, WrapErr};

use bytes::Bytes;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // the list of loaded sprays
    let (sprays, set_sprays) = signal(sprays());

    // the raw skin datas, this will be used to create the normal skin datas
    let (skins, set_skins) = signal(im::Vector::<SkinData>::new());
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

async fn load_pk3(file: File) -> Result<Vec<SkinData>, Report> {
    // read all data
    let data = read_as_bytes(&file)
        .await
        .map(|bytes| Bytes::from(bytes))
        .wrap_err(format!("failed to read file {:?}", file.name()))?;

    // open zip file
    let cursor = Cursor::new(data.clone());
    let mut zip = ZipArchive::new(cursor).wrap_err("zip file has invalid header")?;

    // find all S_SKIN defines
    let s_skins = (0..zip.len())
        .filter_map(|i| {
            let entry = match zip.by_index(i) {
                Ok(entry) => entry,
                Err(err) => {
                    return Some(Err(Report::from(err).wrap_err("failed to read zip file")))
                }
            };

            match Path::new(entry.name()).file_name().and_then(|s| s.to_str()) {
                Some(name) if name == "S_SKIN" => Some(Ok(i)),
                Some(_) => None,
                None => Some(Err(Report::msg(format!(
                    "strange, path {:?} does not have basename",
                    entry.name()
                )))),
            }
        })
        .collect::<Result<Vec<_>, Report>>()?;

    // now, create a skin for each define
    let mut skins = Vec::with_capacity(s_skins.len());

    for file_index in s_skins {
        // get s_skin entry
        let mut entry = zip
            .by_index(file_index)
            .wrap_err("failed to read zip file")?;

        // get path prefix
        let path = Path::new(entry.name())
            .parent()
            .map(|prefix| PathBuf::from(prefix))
            .ok_or_eyre("failed to get path prefix")?;

        // read entry to file
        let mut s_skin = String::with_capacity(entry.size() as usize);
        entry.read_to_string(&mut s_skin)?;

        // parse entry
        let mut parser = soc::Parser::new(&s_skin);
        let skin_define = parser
            .deserialize::<SkinDefine>()
            .wrap_err(format!("invalid S_SKIN lump {:?}", entry.name()))?;

        skins.push(SkinData {
            data: data.clone(),
            skin: skin_define,
            path,
        });
    }

    Ok(skins)
}
