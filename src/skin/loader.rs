//! Different loaders.

use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::doom::patch::{self, Patch as DoomPatch};
use crate::doom::{skin::SkinDefine, soc};

use super::{Skin, Sprite, SpriteName};

use ahash::{HashMap, HashMapExt};

use derive_more::{Deref, Display, Error, From};

use bytes::Bytes;

use wad::Name;
use zip::ZipArchive;

/// A skin loader type.
pub trait SkinLoader: Send + Sync + 'static {
    /// Reads a patch from the pack.
    fn read_sprite(&self, name: Name) -> Result<Sprite, Error>;

    /// Reads a set of patches from the pack by sprite prefix.
    fn read_prefix(&self, prefix: &str) -> Result<Vec<Sprite>, Error>;

    /// Iters over all the patches of a sprite.
    fn list(&self) -> Result<Vec<Name>, Error>;
}

#[derive(Debug, Clone)]
struct Pk3Skin {
    skin: SkinDefine,
    path: PathBuf,
    sprites: HashMap<Name, usize>,
}

/// A pk3 loader.
///
/// Cheap-to-clone.
#[derive(Debug, Clone)]
pub struct Pk3Loader {
    inner: ZipArchive<Cursor<Bytes>>,
    skins: Arc<HashMap<String, Pk3Skin>>,
}

impl Pk3Loader {
    /// Loads all the skins from a given PK3.
    pub fn new(bytes: impl Into<Bytes>) -> Result<Pk3Loader, Error> {
        let bytes = bytes.into();
        let mut zip = ZipArchive::new(Cursor::new(bytes))?;

        // find all S_SKIN defines
        let s_skins = (0..zip.len())
            .filter_map(|i| -> Option<Result<_, Error>> {
                let entry = match zip.by_index_raw(i) {
                    Ok(entry) => entry,
                    Err(err) => return Some(Err(Error::from(err))),
                };

                match Path::new(entry.name()).file_name().and_then(|s| s.to_str()) {
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
            let path = Path::new(entry.name())
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
                .map_err(|err| Error::Deser(path.clone(), err))?;

            // create skin
            let mut skin = Pk3Skin {
                skin: skin_define,
                path,
                sprites: HashMap::new(),
            };

            // read all related sprites
            let mut in_sounds = false;

            for i in 0..zip.len() {
                let entry = zip.by_index_raw(i)?;
                let path = Path::new(entry.name());

                if let Ok(name) = path.strip_prefix(&skin.path) {
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
                        "DS_START" => in_sounds = true,
                        "DS_END" => in_sounds = false,
                        // skip skin lump
                        "S_SKIN" => (),
                        _ if in_sounds => (),
                        _ => {
                            skin.sprites.insert(name, i);
                        }
                    }
                }
            }

            skins.insert(skin.skin.name.clone(), skin);
        }

        Ok(Pk3Loader {
            inner: zip,
            skins: Arc::new(skins),
        })
    }

    /// An iterator over all the skins in a PK3.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Skin> + 'a {
        self.skins.iter().map(|(name, skin)| Skin {
            loader: Arc::new(Box::new(Pk3SkinLoader {
                inner: self.clone(),
                name: name.to_owned(),
            })),
            skin: skin.skin.clone(),
        })
    }
}

#[derive(Deref)]
struct Pk3SkinLoader {
    #[deref]
    inner: Pk3Loader,
    name: String,
}

impl SkinLoader for Pk3SkinLoader {
    fn read_sprite(&self, name: Name) -> Result<Sprite, Error> {
        let name = SpriteName::try_from(name)?;

        let mut zip = self.inner.inner.clone();
        let skin = self.skins.get(&self.name).expect("valid skin");

        let Some(entry_ix) = skin.sprites.get(&name.name).copied() else {
            return Err(Error::NotFound(name.to_string()));
        };

        let mut entry = zip.by_index(entry_ix).expect("valid entry");

        let mut buf = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut buf)?;

        Ok(Sprite {
            name,
            patch: DoomPatch::read(Cursor::new(buf))?,
        })
    }

    fn read_prefix(&self, prefix: &str) -> Result<Vec<Sprite>, Error> {
        let skin = self.skins.get(&self.name).expect("valid skin");

        // index skin
        let names = skin
            .sprites
            .keys()
            .filter(|name| name.as_str().starts_with(prefix))
            .copied()
            .collect::<Vec<_>>();

        names
            .into_iter()
            .map(|name| self.read_sprite(name))
            .collect()
    }

    fn list(&self) -> Result<Vec<Name>, Error> {
        let skin = self.skins.get(&self.name).expect("valid skin");

        Ok(skin.sprites.keys().copied().collect())
    }
}

/// Loader errors.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display("malformed pk3: {_0}")]
    Zip(zip::result::ZipError),
    Io(io::Error),
    Patch(patch::Error),
    Name(super::FromNameError),
    #[display("soc {_0:?}: {_1}")]
    #[from(ignore)]
    Deser(PathBuf, soc::Error),
    #[display("sprite \"{_0}\" not found")]
    NotFound(#[error(not(source))] String),
}

impl Error {
    /// Checks if the error is a not found error.
    pub fn not_found(&self) -> bool {
        matches!(self, Error::NotFound(..))
    }
}
