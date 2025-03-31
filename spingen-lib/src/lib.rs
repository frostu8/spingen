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
use image::Encoder;
use skin::{
    loaders::{Pk3SkinLoader, WadSkinLoader},
    Skin,
};
use spray::{loaders::Pk3SprayLoader, sprays, Spray};

use std::io::{self, Cursor};

use wad::Name;

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

    /// Loads the default sprays.
    #[wasm_bindgen(js_name = fetchDefaultSprays)]
    pub fn fetch_default_sprays(&mut self) -> Vec<Spray> {
        let sprays = sprays();

        self.sprays
            .extend(sprays.iter().map(|spray| (spray.id.clone(), spray.clone())));
        sprays
    }

    /// Loads sprays and skins from a file.
    #[wasm_bindgen(js_name = fetchAll)]
    pub async fn fetch_all(
        &mut self,
        blob: &web_sys::File,
        resolve_spray: &js_sys::Function,
        resolve_skin: &js_sys::Function,
    ) {
        let file = File::from(blob.clone());
        let name = file.name();

        if name.ends_with(".pk3") {
            let file = match read_as_bytes(&file).await {
                Ok(file) => Bytes::from(file),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            // read into loader
            let loader = match Pk3SprayLoader::new(file.clone()) {
                Ok(loader) => loader.filter_map(|spray| match spray {
                    Ok(spray) => Some(spray),
                    Err(err) => {
                        error!("{:?}", Report::from(err).wrap_err("failed reading spray"));
                        None
                    }
                }),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            for spray in loader {
                self.sprays.insert(spray.id.clone(), spray.clone());
                let _ = resolve_spray.call1(&JsValue::null(), &JsValue::from(spray));
            }

            // read into loader
            let loader = match Pk3SkinLoader::new(file) {
                Ok(loader) => loader.filter_map(|spray| match spray {
                    Ok(spray) => Some(spray),
                    Err(err) => {
                        error!("{:?}", Report::from(err).wrap_err("failed reading spray"));
                        None
                    }
                }),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            for skin in loader {
                self.skins.insert(skin.name.clone(), skin.clone());
                let _ = resolve_skin.call1(&JsValue::null(), &JsValue::from(skin));
            }
        } else if name.ends_with(".wad") {
            let file = match read_as_bytes(&file).await {
                Ok(file) => Bytes::from(file),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            // read into loader
            let loader = match WadSkinLoader::new(file) {
                Ok(loader) => loader.filter_map(|spray| match spray {
                    Ok(spray) => Some(spray),
                    Err(err) => {
                        error!("{:?}", Report::from(err).wrap_err("failed reading spray"));
                        None
                    }
                }),
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };

            for skin in loader {
                self.skins.insert(skin.name.clone(), skin.clone());
                let _ = resolve_skin.call1(&JsValue::null(), &JsValue::from(skin));
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

    /// Generates a skin animation.
    #[wasm_bindgen(js_name = generateSkinAnimation)]
    pub fn generate_skin_animation(
        &self,
        skin_id: String,
        spray_id: Option<String>,
        sprite: String,
        frame: String,
        options: image::GifOptions,
    ) -> Result<String, JsValue> {
        // try to parse input
        let name = Name::from_bytes(sprite.as_bytes())
            .wrap_err("invalid `sprite` parameter")
            .map_err(|err| JsValue::from(format!("{:?}", err)))?;
        let frame = if frame.len() == 1 {
            frame.as_bytes()[0]
        } else {
            return Err(format!("invalid `frame` parameter: \"{}\"", frame).into());
        };

        let (skin, spray) = self.get_skin_and_spray(skin_id, spray_id)?;
        let mut encoder = Encoder::new(&skin).with_spray(&spray);

        // generate new gif
        let mut buf = Vec::new();
        encoder
            .sprite_gif_with_options(Cursor::new(&mut buf), name, frame, options)
            .wrap_err("failed to encode gif")
            .map_err(|err| JsValue::from(format!("{:?}", err)))?;

        // ignore MIME for now
        let blob = Blob::new_with_options(&buf[..], None);

        Url::create_object_url_with_blob(blob.as_ref())
    }

    /// Generates a skin thumbnail.
    #[wasm_bindgen(js_name = generateSkinThumbnail)]
    pub fn generate_skin_thumbnail(
        &self,
        skin_id: String,
        spray_id: Option<String>,
    ) -> Result<String, JsValue> {
        let (skin, spray) = self.get_skin_and_spray(skin_id, spray_id)?;
        let mut encoder = Encoder::new(&skin).with_spray(&spray);

        // try to find asymmetric sprite first
        let mut buf = Vec::new();
        encoder
            .sprite(
                Cursor::new(&mut buf),
                "STINA2".parse::<Name>().expect("valid name"),
            )
            .or_else(|err| {
                if err.not_found() {
                    // try to get other sprite
                    encoder.sprite(
                        Cursor::new(&mut buf),
                        "STINA2A8".parse::<Name>().expect("valid name"),
                    )
                } else {
                    Err(err)
                }
            })
            .wrap_err("failed to encode thumbnail")
            .map_err(|err| JsValue::from(format!("{:?}", err)))?;

        let blob = Blob::new_with_options(&buf[..], Some("image/png"));

        Url::create_object_url_with_blob(blob.as_ref())
    }

    fn get_skin_and_spray(
        &self,
        skin_id: String,
        spray_id: Option<String>,
    ) -> Result<(&Skin, &Spray), JsValue> {
        // get skin
        let Some(skin) = self.skins.get(&skin_id) else {
            return Err(format!("skin \"{}\" not found", skin_id).into());
        };

        // get spray if it exists
        let spray = if let Some(spray_id) = spray_id {
            match self.sprays.get(&spray_id) {
                Some(spray) => spray,
                None => return Err(format!("spray \"{}\" not found", spray_id).into()),
            }
        } else {
            let spray = self
                .sprays
                .values()
                .find(|spray| spray.name.eq_ignore_ascii_case(&skin.prefcolor));
            if let Some(spray) = spray {
                spray
            } else {
                warn!("invalid prefcolor {:?}, using default", skin.prefcolor);
                self.sprays.values().next().expect("at least 1 spray")
            }
        };

        Ok((skin, spray))
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
