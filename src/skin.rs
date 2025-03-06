//! Discovers and compiles all skins.

use crate::doom::skin::SkinDefine;

use std::{path::PathBuf, str::FromStr};

use bytes::Bytes;

use wad::Name;

use derive_more::{Deref, Display, Error};

/// An index for the skin sprites.
#[derive(Clone, Debug)]
pub struct SpriteIndex {
    /// The sprite name.
    name: Name,
    /// The index of the sprite, starting from A.
    index: u8,
}

impl SpriteIndex {
    /// An iterator of asymmetric angles.
    pub fn angles<'a>(&'a self) -> impl Iterator<Item = Name> + ExactSizeIterator + 'a {
        (0u8..8u8).map(|s| s + b'1').map(|by| {
            let mut bytes: [u8; 8] = (*self.name).clone();
            bytes[4] = self.index as u8;
            bytes[5] = by;

            Name::from_bytes(&bytes).expect("valid name")
        })
    }

    /// Creates a new sprite index from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<SpriteIndex, SpriteIndexError> {
        if bytes.len() < 5 {
            return Err(SpriteIndexError::InvalidLength(bytes.len()));
        }
        // the first four bytes are the sprite name
        // TODO proper error tunneling
        let name = Name::from_bytes(&bytes[..4]).map_err(SpriteIndexError::Name)?;

        // the fifth character is the sprite index
        if bytes[4] < b'A' {
            return Err(SpriteIndexError::InvalidFrame(bytes[4] as char));
        }

        Ok(SpriteIndex {
            name,
            index: bytes[4],
        })
    }
}

impl FromStr for SpriteIndex {
    type Err = SpriteIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SpriteIndex::from_bytes(s.as_bytes())
    }
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

/// An error for [`SpriteIndex::from_bytes`]
#[derive(Debug, Display, Error)]
pub enum SpriteIndexError {
    #[display("invalid len {_0}")]
    InvalidLength(#[error(not(source))] usize),
    #[display("invalid frame {_0}")]
    InvalidFrame(#[error(not(source))] char),
    Name(wad::NameParseError),
}
