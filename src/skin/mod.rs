//! Discovers and compiles all skins.

pub mod loader;

use crate::doom::{patch::Patch as DoomPatch, skin::SkinDefine};

use loader::{Error as LoaderError, SkinLoader};

use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use wad::Name;

use derive_more::{Deref, Display};

/// The actual internal skin data.
///
/// Contains information about the skin, and all the patches associated with
/// it. This data never changes, so it is exchanged around in an [`Arc`].
#[derive(Clone, Deref)]
pub struct Skin {
    /// The loader allocated for this skin.
    loader: Arc<Box<dyn SkinLoader>>,
    /// The skin description.
    #[deref]
    skin: SkinDefine,
}

impl Skin {
    /// Reads a sprite from the skin.
    pub fn read_sprite(&self, name: Name) -> Result<Sprite, LoaderError> {
        self.loader.read_sprite(name)
    }

    /// Reads a set of sprites by prefix.
    pub fn read_prefix(&self, prefix: &str) -> Result<Vec<Sprite>, LoaderError> {
        self.loader.read_prefix(prefix)
    }
}

impl Debug for Skin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Skin")
            .field("skin", &self.skin)
            .finish_non_exhaustive()
    }
}

/// A parsed sprite name.
#[derive(Clone, Copy, Debug, Deref)]
pub struct SpriteName {
    #[deref]
    name: Name,
    frame: SpriteFrame,
    mirrored_frame: Option<SpriteFrame>,
}

impl SpriteName {
    /// The 4-character sprite identifier.
    pub fn identifier(&self) -> Name {
        Name::from_bytes(&self.name[..4]).expect("valid subname")
    }
}

impl TryFrom<Name> for SpriteName {
    type Error = FromNameError;

    fn try_from(value: Name) -> Result<Self, Self::Error> {
        if value.as_str().len() >= 6 {
            let to_frame = |bytes: &[u8]| {
                if bytes[0] < b'A' {
                    return Err(FromNameError {
                        name: value,
                        kind: FromNameErrorKind::InvalidFrame(bytes[0] as char),
                    });
                }
                let frame = bytes[0];

                if bytes[1] < b'0' {
                    return Err(FromNameError {
                        name: value,
                        kind: FromNameErrorKind::InvalidAngle(bytes[1] as char),
                    });
                }
                let angle = bytes[1] - b'0';

                Ok(SpriteFrame { frame, angle })
            };

            let frame = to_frame(&value[4..6])?;
            let mirrored_frame = if value.as_str().len() == 8 {
                Some(to_frame(&value[6..8])?)
            } else {
                None
            };

            Ok(SpriteName {
                name: value,
                frame,
                mirrored_frame,
            })
        } else {
            Err(FromNameError {
                name: value,
                kind: FromNameErrorKind::InvalidLength(value.as_str().len()),
            })
        }
    }
}

/// A frame of a sprite.
#[derive(Clone, Copy, Debug)]
pub struct SpriteFrame {
    pub frame: u8,
    pub angle: u8,
}

/// A patch with a name.
#[derive(Clone, Debug, Deref)]
pub struct Sprite {
    name: SpriteName,
    #[deref]
    patch: DoomPatch,
}

impl Sprite {
    /// The name of the sprite.
    pub fn name(&self) -> &Name {
        &self.name.name
    }

    /// The frame of the sprite.
    pub fn frame(&self) -> &SpriteFrame {
        &self.name.frame
    }

    /// The mirrored frame of the sprite, if any.
    pub fn mirrored_frame(&self) -> Option<&SpriteFrame> {
        self.name.mirrored_frame.as_ref()
    }
}

#[derive(Debug, Display)]
#[display("invalid sprite name \"{name}\": {kind}")]
pub struct FromNameError {
    name: Name,
    kind: FromNameErrorKind,
}

impl std::error::Error for FromNameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            FromNameErrorKind::Name(wad) => Some(wad),
            _ => None,
        }
    }
}

#[derive(Debug, Display)]
pub enum FromNameErrorKind {
    #[display("invalid len {_0}")]
    InvalidLength(usize),
    #[display("invalid frame {_0}")]
    InvalidFrame(char),
    #[display("invalid angle {_0}")]
    InvalidAngle(char),
    Name(wad::NameParseError),
}
