//! Different loaders.

use std::io::{self, Cursor, Read, Seek};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

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
}

#[derive(Debug, Clone)]
struct Pk3Skin {
    skin: SkinDefine,
    path: PathBuf,
    sprites: OnceLock<HashMap<Name, usize>>,
}

impl Pk3Skin {
    fn sprites<R>(&self, inner: &mut ZipArchive<R>) -> Result<&HashMap<Name, usize>, Error>
    where
        R: Read + Seek,
    {
        self.sprites.get_or_try_init(|| -> Result<_, Error> {
            let mut sprites = HashMap::<Name, usize>::new();

            for i in 0..inner.len() {
                let entry = inner.by_index(i)?;
                let path = Path::new(entry.name());

                if let Ok(name) = path.strip_prefix(&self.path) {
                    if let Some(name) = name.to_str().and_then(|s| s.parse::<Name>().ok()) {
                        sprites.insert(name, i);
                    }
                }
            }

            Ok(sprites)
        })
    }
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
                let entry = match zip.by_index(i) {
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

            // parse entry
            let mut parser = soc::Parser::new(&s_skin);
            let skin_define = parser
                .deserialize::<SkinDefine>()
                .map_err(|err| Error::Deser(path.clone(), err))?;

            skins.insert(
                skin_define.name.clone(),
                Pk3Skin {
                    skin: skin_define,
                    path,
                    sprites: OnceLock::default(),
                },
            );
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

        let path = skin.path.join(name.as_str());
        let path = path.to_str().expect("path should always be valid str");
        let mut entry = zip.by_name(path).map_err(|err| match err {
            zip::result::ZipError::FileNotFound => Error::NotFound(name.to_string()),
            err => Error::Zip(err),
        })?;

        let mut buf = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut buf)?;

        Ok(Sprite {
            name,
            patch: DoomPatch::read(Cursor::new(buf))?,
        })
    }

    fn read_prefix(&self, prefix: &str) -> Result<Vec<Sprite>, Error> {
        let mut zip = self.inner.inner.clone();
        let skin = self.skins.get(&self.name).expect("valid skin");

        // index skin
        let names = skin
            .sprites(&mut zip)?
            .keys()
            .filter(|name| name.as_str().starts_with(prefix))
            .copied()
            .collect::<Vec<_>>();

        names
            .into_iter()
            .map(|name| self.read_sprite(name))
            .collect()
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
