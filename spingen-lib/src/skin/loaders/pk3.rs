//! PK3 loaders.

use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::doom::skin::SkinDefine;
use crate::lump::Lump;
use crate::skin::{spr2, Error, Skin};

use bytes::Bytes;

use wad::Name;
use zip::ZipArchive;

/// A PK3 skin loader.
#[derive(Clone, Debug)]
pub struct Pk3SkinLoader {
    zip: ZipArchive<Cursor<Bytes>>,
    file_index: usize,
}

impl Pk3SkinLoader {
    /// Creates a new PK3 loader.
    pub fn new(bytes: impl Into<Bytes>) -> Result<Pk3SkinLoader, Error> {
        let bytes = bytes.into();
        ZipArchive::new(Cursor::new(bytes))
            .map(|zip| Pk3SkinLoader { zip, file_index: 0 })
            .map_err(From::from)
    }

    fn read_skin(&mut self, ix: usize) -> Result<Skin, Error> {
        // get s_skin entry
        let mut entry = self.zip.by_index(ix)?;

        // get path prefix
        let skin_path = Path::new(entry.name())
            .parent()
            .map(|prefix| PathBuf::from(prefix))
            .expect("path should have prefix");

        // read entry to file
        let mut s_skin = String::with_capacity(entry.size() as usize);
        entry.read_to_string(&mut s_skin)?;
        drop(entry);

        // parse entry
        let skin_define = SkinDefine::read(&s_skin)
            .map_err(|err| Error::Skin(skin_path.display().to_string(), err))?;

        // read all related sprites
        let mut index = spr2::Index::default();
        let mut in_sounds = false;

        for i in 0..self.zip.len() {
            let entry = self.zip.by_index_raw(i)?;
            let path = Path::new(entry.name());

            if let Ok(name) = path.strip_prefix(&skin_path) {
                let Some(name) = name
                    .to_str()
                    .map(|name| {
                        if let Some(ix) = name.rfind('.') {
                            // strip ext
                            &name[..ix]
                        } else {
                            name
                        }
                    })
                    .and_then(|s| s.parse::<Name>().ok())
                else {
                    continue;
                };

                if name.as_str().len() == 0 {
                    // skip the folder containing the skin
                    continue;
                }

                match name.as_str() {
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
                    // skip if within sounds bounds
                    _ if in_sounds => continue,
                    _ => (),
                }

                drop(entry);

                // read patch data
                if let Err(err) = index.add(name, Lump::new_from_zip(self.zip.clone(), i)) {
                    warn!("{:?}", err);
                }
            }
        }

        Ok(Skin {
            skin: Arc::new(skin_define),
            index: Arc::new(index),
        })
    }
}

impl Iterator for Pk3SkinLoader {
    type Item = Result<Skin, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.file_index < self.zip.len() {
            let file_index = self.file_index;
            self.file_index += 1;

            // find next S_SKIN define
            let entry = match self.zip.by_index_raw(file_index) {
                Ok(entry) => entry,
                Err(err) => return Some(Err(err.into())),
            };

            let Some(name) = Path::new(entry.name()).file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            if name.eq_ignore_ascii_case("S_SKIN") {
                drop(entry);
                return Some(self.read_skin(file_index));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.zip.len()))
    }
}
