//! Discovers and compiles all skins.

use crate::doom::{patch::Patch, skin::SkinDefine, soc};

use ahash::RandomState;

use std::collections::HashMap;
use std::io::{Read, Seek};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use wad::Name;

use zip::read::ZipArchive;

use eyre::{OptionExt, Report, WrapErr};

/// The main `Skin` type.
///
/// Stores some useful, cached data.
#[derive(Clone, Debug)]
pub struct Skin {
    data: Arc<SkinData>,
}

impl Deref for Skin {
    type Target = SkinData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// The actual internal skin data.
///
/// Contains information about the skin, and all the patches associated with
/// it. This data never changes, so it is exchanged around in an [`Arc`].
#[derive(Clone, Debug)]
pub struct SkinData {
    /// The skin description.
    pub skin: SkinDefine,
    /// The patches associated with this skin.
    pub patches: HashMap<Name, Patch, RandomState>,
}

/// Loads a PK3, returning all the skins in the PK3.
pub fn load_pk3<R>(reader: R) -> Result<Vec<SkinData>, Report>
where
    R: Read + Seek,
{
    // open zip file
    let mut zip = ZipArchive::new(reader).wrap_err("zip file has invalid header")?;

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
    let mut skins: Vec<SkinData> = Vec::with_capacity(s_skins.len());

    for file_index in s_skins {
        // get s_skin entry
        let mut entry = zip
            .by_index(file_index)
            .wrap_err("failed to read zip file")?;

        // get path prefix
        let prefix = Path::new(entry.name())
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

        drop(entry); // rust moment

        // parse associated lumps
        let mut sprites = HashMap::<Name, Patch, RandomState>::default();
        let mut in_sounds = false;

        for i in 0..zip.len() {
            // we need to get the file first
            let entry = zip.by_index(i).wrap_err("failed to read zip file")?;
            let path = PathBuf::from(entry.name());
            drop(entry); // more rust moment

            let mut entry = zip.by_index_seek(i).wrap_err("failed to read zip file")?;

            if let Some(lumpname) = path.strip_prefix(&prefix).ok().and_then(|s| s.to_str()) {
                let Ok(lumpname) = Name::from_bytes(lumpname.as_bytes()) else {
                    // skip malformed names
                    continue;
                };

                // check for sound headers in zskins
                // our patch parser will be sad if we get any ogg files in here
                match lumpname.as_str() {
                    "DS_START" => {
                        in_sounds = true;
                        continue;
                    }
                    "DS_END" => {
                        in_sounds = false;
                        continue;
                    }
                    // always skip skindef
                    "S_SKIN" => continue,
                    _ => (),
                }

                if in_sounds {
                    // skip if reading sounds
                    continue;
                }

                // read patch
                let patch = Patch::read(&mut entry).wrap_err("invalid patch")?;
                sprites.insert(lumpname, patch);
            }
        }

        skins.push(SkinData {
            skin: skin_define,
            patches: sprites,
        })
    }

    Ok(skins)
}
