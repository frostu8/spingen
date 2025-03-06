//! Discovers and compiles all skins.

use crate::doom::patch::Palette;
use crate::doom::spray::Spray;
use crate::doom::{patch::Patch as DoomPatch, skin::SkinDefine};

use std::io::{Cursor, Read};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use derive_more::Deref;

use eyre::{Report, WrapErr};

use gloo::file::Blob;
use leptos::prelude::*;

use bytes::Bytes;
use web_sys::Url;
use zip::ZipArchive;

/// The main `Skin` type.
///
/// Stores some useful, cached data.
#[derive(Clone, Debug)]
pub struct Skin {
    data: Arc<SkinData>,
    spray: RwSignal<Spray>,
    thumbnail_url: Signal<Option<String>>,
}

impl Skin {
    /// Creates a new `Skin` from [`SkinData`].
    ///
    /// This sets up  some
    pub fn new(value: SkinData) -> Skin {
        let data = Arc::new(value);
        let data_clone = data.clone();

        let spray = RwSignal::default();
        let thumbnail_url = Signal::derive(move || {
            // get spray color
            let _spray = spray.get();

            match create_thumbnail(data_clone.clone(), &Palette::default()) {
                Ok(url) => Some(url),
                Err(err) => {
                    leptos::logging::error!("{:?}", err);
                    None
                }
            }
        });

        Skin {
            data,
            spray,
            thumbnail_url,
        }
    }

    /// The thumbnail url of the skin.
    pub fn thumbnail_url(&self) -> Option<String> {
        self.thumbnail_url.get()
    }

    /// The realname of the skin ready for display.
    pub fn display_name(&self) -> String {
        self.data.realname.replace('_', " ")
    }
}

impl From<SkinData> for Skin {
    fn from(value: SkinData) -> Self {
        Skin::new(value)
    }
}

impl Deref for Skin {
    type Target = SkinData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Drop for Skin {
    fn drop(&mut self) {
        // revoke url
        if let Some(url) = self.thumbnail_url.get_untracked() {
            let _ = Url::revoke_object_url(&url);
        }
    }
}

fn create_thumbnail(data: Arc<SkinData>, palette: &Palette) -> Result<String, Report> {
    let mut zip = ZipArchive::new(Cursor::new(data.data.clone())).wrap_err("invalid zip file")?;

    let path = data.path.join("STINA2");
    let asym_path = data.path.join("STINA2A8");

    let Some(entry_ix) = path
        .to_str()
        .and_then(|e| zip.index_for_name(e))
        .or(asym_path.to_str().and_then(|e| zip.index_for_name(e)))
    else {
        // no entry found
        return Err(Report::msg(format!(
            "no valid thumbnail found foor {:?}",
            data.name
        )));
    };

    let mut entry = zip.by_index(entry_ix).wrap_err("invalid zip file")?;
    let mut buf = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut buf).wrap_err("invalid patch")?;

    // load patch
    let patch = DoomPatch::read_with(Cursor::new(buf), palette).wrap_err("invalid patch")?;
    // encode image from patch
    let mut buf = Vec::new();
    patch
        .image
        .encode(Cursor::new(&mut buf))
        .wrap_err("failed to encode patch")?;

    // create blob
    let blob = Blob::new_with_options(&buf[..], Some("image/png"));

    Ok(Url::create_object_url_with_blob(blob.as_ref()).expect("valid blob url"))
}

/// The actual internal skin data.
///
/// Contains information about the skin, and all the patches associated with
/// it. This data never changes, so it is exchanged around in an [`Arc`].
#[derive(Clone, Debug, Deref)]
pub struct SkinData {
    /// The PK3 or WAD data this skin data is from.
    pub data: Bytes,
    /// The path of the skin.
    pub path: PathBuf,
    /// The skin description.
    #[deref]
    pub skin: SkinDefine,
}

/// A patch.
///
/// Exposes methods for encoding the patch data into more usable formats.
#[derive(Clone, Debug)]
pub struct Patch {
    data: Vec<u8>,
}

impl Patch {
    /// Creates a new patch from its raw bytes.
    pub fn new(data: impl Into<Vec<u8>>) -> Patch {
        Patch { data: data.into() }
    }
}
