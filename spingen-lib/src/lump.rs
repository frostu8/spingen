//! Data lumps.

use zip::ZipArchive;

use std::fmt::{self, Debug, Formatter};
use std::io::{Cursor, Read as _};

use crate::Error;

use bytes::Bytes;

/// A data storage that's either a reference to a pk3 or just raw bytes.
///
/// Cheap-to-clone.
#[derive(Clone, Debug)]
pub struct Lump {
    inner: LumpInner,
}

impl Lump {
    /// Creates a new lump from raw bytes.
    pub fn new(bytes: impl Into<Bytes>) -> Lump {
        Lump {
            inner: LumpInner::Bytes(bytes.into()),
        }
    }

    /// Creates a new lump from a zip file.
    pub fn new_from_zip(zip: ZipArchive<Cursor<Bytes>>, ix: usize) -> Lump {
        Lump {
            inner: LumpInner::Zip(zip, ix),
        }
    }
}

#[derive(Clone)]
enum LumpInner {
    Zip(ZipArchive<Cursor<Bytes>>, usize),
    Bytes(Bytes),
}

impl Debug for LumpInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LumpInner").finish_non_exhaustive()
    }
}

impl Lump {
    /// Reads the inner lump data.
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        match self.inner {
            LumpInner::Zip(ref mut zip, index) => {
                let mut entry = zip.by_index(index)?;
                let mut buf = Vec::with_capacity(entry.size() as usize);
                entry.read_to_end(&mut buf)?;

                Ok(buf)
            }
            LumpInner::Bytes(ref bytes) => Ok(Vec::from(bytes.clone())),
        }
    }
}
