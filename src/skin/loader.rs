//! Different loaders.

use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::doom::{skin::SkinDefine, soc};
use crate::lump::Lump;

use super::{spr2, Error, Skin};

use ahash::{HashMap, HashMapExt};

use bytes::Bytes;

use wad::Name;
use zip::ZipArchive;

/// Loads all the skins from a given PK3.
pub fn load_pk3(bytes: impl Into<Bytes>) -> Result<HashMap<String, Skin>, Error> {
    let bytes = bytes.into();
    let mut zip = ZipArchive::new(Cursor::new(bytes))?;

    // find all S_SKIN defines
    let s_skins = (0..zip.len())
        .filter_map(|i| -> Option<Result<_, Error>> {
            let entry = match zip.by_index_raw(i) {
                Ok(entry) => entry,
                Err(err) => return Some(Err(Error::from(err))),
            };

            match Path::new(entry.name()).file_stem().and_then(|s| s.to_str()) {
                Some(name) if name == "S_SKIN" => Some(Ok(i)),
                Some(_) => None,
                // ignore misplaced skin definitions at top level
                None => None,
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // now, create a skin for each define
    let mut skins = HashMap::with_capacity(s_skins.len());

    for file_index in s_skins {
        // get s_skin entry
        let mut entry = zip.by_index(file_index)?;

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
        let mut parser = soc::Parser::new(&s_skin);
        let skin_define = parser
            .deserialize::<SkinDefine>()
            .map_err(|err| Error::Deser(skin_path.display().to_string(), err))?;

        // read all related sprites
        let mut index = spr2::Index::default();
        let mut in_sounds = false;

        for i in 0..zip.len() {
            let entry = zip.by_index_raw(i)?;
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
                if let Err(err) = index.add(name, Lump::new_from_zip(zip.clone(), i)) {
                    leptos::logging::error!("{:?}", err);
                }
            }
        }

        skins.insert(
            skin_define.name.clone(),
            Skin {
                skin: Arc::new(skin_define),
                index: Arc::new(index),
            },
        );
    }

    Ok(skins)
}
