#![feature(once_cell_try)]

#[macro_use]
extern crate log;

pub mod doom;
pub mod image;
pub mod lump;
pub mod skin;
pub mod spray;

use derive_more::{Display, Error, From};

use ahash::HashMap;

use gloo::file::{futures::read_as_bytes, Blob, File};

use doom::patch::{Palette, Patch};
use image::patch_to_image;
use skin::Skin;
use spray::{loaders::Pk3SprayLoader, Spray};

use std::io::{self, Cursor};

use web_sys::Url;

use bytes::Bytes;

use wasm_bindgen::prelude::*;

use log::Level;

use eyre::{Report, WrapErr};

const SPRAYCAN_GRAPHIC: &[u8] = include_bytes!("./SPCNK0.lmp");

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
}

/// Spingen entry point.
#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct Spingen {
    sprays: HashMap<String, Spray>,
    skins: HashMap<String, Skin>,
}

#[wasm_bindgen]
impl Spingen {
    /// Creates a new `Spingen`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Spingen {
        Spingen::default()
    }

    /// Loads sprays from a file.
    ///
    /// If the file is a wad, this does nothing.
    #[wasm_bindgen(js_name = fetchSprays)]
    pub async fn fetch_sprays(&mut self, blob: &web_sys::File, resolve: &js_sys::Function) {
        let file = File::from(blob.clone());

        if file.name().ends_with(".pk3") {
            let file = match read_as_bytes(&file).await {
                Ok(file) => Bytes::from(file),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            // read into loader
            let loader = match Pk3SprayLoader::new(file) {
                Ok(loader) => loader,
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            let loader = loader.filter_map(|spray| match spray {
                Ok(spray) => Some(spray),
                Err(err) => {
                    error!("{:?}", Report::from(err).wrap_err("failed reading spray"));
                    None
                }
            });

            for spray in loader {
                self.sprays.insert(spray.id.clone(), spray.clone());
                let _ = resolve.call1(&JsValue::null(), &JsValue::from(spray));
            }
        }
    }

    /// Generates a spraycan image.
    #[wasm_bindgen(js_name = generateSprayImage)]
    pub fn generate_spray_image(&self, spray_id: String) -> Result<String, JsValue> {
        // get spray
        let Some(spray) = self.sprays.get(&spray_id) else {
            return Err(format!("spray \"{}\" not found", spray_id).into());
        };

        let palette = Palette::default();
        let palette = spray.remap(&palette, 96);

        // load patch
        let patch = Patch::read(Cursor::new(SPRAYCAN_GRAPHIC))
            .wrap_err("failed to read spraycan graphic")
            .map_err(|err| JsValue::from(format!("{:?}", err)))?;

        // write to png
        let mut buf = Vec::new();
        patch_to_image(Cursor::new(&mut buf), &patch, &palette)
            .wrap_err("failed to generate spraycan graphic")
            .map_err(|err| JsValue::from(format!("{:?}", err)))?;

        let blob = Blob::new_with_options(&buf[..], Some("image/png"));

        Url::create_object_url_with_blob(blob.as_ref())
    }
}

/// Loader errors.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display("malformed pk3: {_0}")]
    Zip(zip::result::ZipError),
    #[display("malformed wad: {_0}")]
    Wad(wad::Error),
    Io(io::Error),
    Patch(crate::doom::patch::Error),
    Name(skin::FromNameError),
    #[display("invalid skin {_0:?}: {_1}")]
    #[from(ignore)]
    Skin(String, crate::doom::skin::Error),
    #[display("sprite \"{_0}\" not found")]
    NotFound(#[error(not(source))] String),
    #[display("sprite \"{_0}\" is malformed")]
    Image(String, eyre::Report),
}

impl Error {
    /// Checks if the error is a not found error.
    pub fn not_found(&self) -> bool {
        matches!(self, Error::NotFound(..))
    }
}
