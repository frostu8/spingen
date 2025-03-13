//! WAD loaders.

use std::io::{Cursor, Read};
use std::sync::Arc;

use crate::doom::skin::SkinDefine;
use crate::lump::Lump;
use crate::skin::{spr2, Skin};
use crate::Error;

use bytes::Bytes;

use wad::Archive;

/// A WAD skin loader.
#[derive(Clone, Debug)]
pub struct WadSkinLoader {
    wad: Archive<Cursor<Bytes>>,
    consumed: bool,
}

impl WadSkinLoader {
    /// Creates a new WAD loader.
    pub fn new(bytes: impl Into<Bytes>) -> Result<WadSkinLoader, Error> {
        let bytes = bytes.into();
        Archive::new(Cursor::new(bytes))
            .map(|wad| WadSkinLoader {
                wad,
                consumed: false,
            })
            .map_err(From::from)
    }
}

impl Iterator for WadSkinLoader {
    type Item = Result<Skin, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }

        self.consumed = true;

        // find S_SKIN lump
        let mut s_skin = None::<String>;
        for i in 0..self.wad.len() {
            let mut entry = self.wad.get(i).expect("valid entry");

            if entry.name().as_str().eq_ignore_ascii_case("S_SKIN") {
                let mut text = String::with_capacity(entry.bytes_len());
                match entry.read_to_string(&mut text) {
                    Ok(_) => (),
                    Err(err) => return Some(Err(err.into())),
                }
                s_skin = Some(text);
            }
        }

        let Some(s_skin) = s_skin else {
            return None;
        };

        let skin_define = match SkinDefine::read(&s_skin) {
            Ok(skin) => skin,
            Err(err) => return Some(Err(Error::Skin("S_SKIN".into(), err))),
        };

        // read all related sprites
        let mut index = spr2::Index::default();
        let mut in_sounds = false;

        for i in 0..self.wad.len() {
            let mut entry = self.wad.get(i).expect("valid entry");

            match entry.name().as_str() {
                "DS_START" => {
                    in_sounds = true;
                    continue;
                }
                "DS_END" => {
                    in_sounds = false;
                    continue;
                }
                // skip skin lump
                "S_SKIN" => continue,
                // skip if within sound bounds
                _ if in_sounds => continue,
                _ => (),
            }

            // read patch data
            let mut buf = Vec::with_capacity(entry.bytes_len());
            match entry.read_to_end(&mut buf) {
                Ok(_) => (),
                Err(err) => return Some(Err(err.into())),
            }
            if let Err(err) = index.add(*entry.name(), Lump::new(buf)) {
                leptos::logging::warn!("{:?}", err);
            }
        }

        Some(Ok(Skin {
            skin: Arc::new(skin_define),
            index: Arc::new(index),
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for WadSkinLoader {
    fn len(&self) -> usize {
        // wad files can only contain one char
        1
    }
}
