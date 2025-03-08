//! Discovers and compiles all skins.

pub mod loader;
pub mod spr2;

use crate::doom::{patch::Patch as DoomPatch, skin::SkinDefine};
use spr2::Spr2;

use loader::{Error as LoaderError, SkinLoader};

use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroU8;
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
    /// The skin index
    index: Arc<spr2::Index>,
    /// The skin description.
    #[deref]
    skin: Arc<SkinDefine>,
}

impl Skin {
    /// Reads a patch from the skin.
    pub fn read(&self, name: Name) -> Result<DoomPatch, LoaderError> {
        self.loader.read(name)
    }

    /// Iterates over all unique skin sprite names.
    pub fn iter(&self) -> impl Iterator<Item = Name> {
        self.index.iter()
    }

    /// Iterates over all unique frames of a sprite.
    pub fn iter_frames(&self, name: &Name) -> impl Iterator<Item = u8> {
        self.index.iter_frames(name)
    }

    /// Iterates over all unique sprite angles.
    pub fn iter_angles<'a>(
        &'a self,
        name: &'a Name,
        frame: u8,
    ) -> impl Iterator<Item = &'a Spr2> + 'a {
        self.index.iter_angles(name, frame)
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

    /// The frame of the sprite.
    pub fn frame(&self) -> &SpriteFrame {
        &self.frame
    }

    /// The mirrored frame of the sprite, if any.
    pub fn mirrored_frame(&self) -> Option<&SpriteFrame> {
        self.mirrored_frame.as_ref()
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

                let Some(angle) = SpriteAngle::from_ascii_char(bytes[1]) else {
                    return Err(FromNameError {
                        name: value,
                        kind: FromNameErrorKind::InvalidAngle(bytes[1] as char),
                    });
                };

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
    pub angle: SpriteAngle,
}

/// A sprite angle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpriteAngle(NonZeroU8);

impl SpriteAngle {
    /// The "all" angle.
    pub const ALL: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'0') });
    /// The forward angle.
    pub const FORWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'1') });
    /// The right forward angle.
    pub const RIGHT_FORWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'2') });
    /// The right angle.
    pub const RIGHT: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'3') });
    /// The right backward angle.
    pub const RIGHT_BACKWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'4') });
    /// The backward angle.
    pub const BACKWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'5') });
    /// The left backward angle.
    pub const LEFT_BACKWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'6') });
    /// The left angle.
    pub const LEFT: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'7') });
    /// The left forward angle.
    pub const LEFT_FORWARD: SpriteAngle = SpriteAngle(unsafe { NonZeroU8::new_unchecked(b'8') });

    /// Unwraps the inner `u8`.
    pub fn into_inner(self) -> u8 {
        self.0.get()
    }

    /// Creates a `SpriteAngle` from an ascii char.
    ///
    /// Returns `None` if the angle is invalid.
    pub fn from_ascii_char(byte: u8) -> Option<SpriteAngle> {
        if (b'0'..=b'9').contains(&byte) {
            Some(SpriteAngle(NonZeroU8::new(byte).expect("valid byte")))
        } else {
            None
        }
    }
}

/// A patch with a name.
#[derive(Clone, Debug, Deref)]
pub struct Sprite {
    name: SpriteName,
    #[deref]
    patch: DoomPatch,
}

impl Sprite {
    /// The full name of the sprite.
    pub fn name(&self) -> &Name {
        &self.name.name
    }

    /// The 4-character sprite identifier.
    pub fn identifier(&self) -> Name {
        Name::from_bytes(&self.name.name[..4]).expect("valid subname")
    }

    /// The frame of the sprite.
    pub fn frame(&self) -> &SpriteFrame {
        &self.name.frame
    }

    /// The mirrored frame of the sprite, if any.
    pub fn mirrored_frame(&self) -> Option<&SpriteFrame> {
        self.name.mirrored_frame.as_ref()
    }

    /// Checks if the sprite provides an angle.
    ///
    /// The boolean returned will be `true` if the sprite must be mirrored to
    /// produce the angle.
    pub fn provides(&self, frame: u8, angle: SpriteAngle) -> Option<bool> {
        // terrible naming choices were made
        let f = self.name.frame();
        if (f.angle == angle || f.angle == SpriteAngle::ALL) && f.frame == frame {
            Some(false)
        } else if self
            .name
            .mirrored_frame()
            .map(|f| (f.angle == angle || f.angle == SpriteAngle::ALL) && f.frame == frame)
            .unwrap_or_default()
        {
            Some(true)
        } else {
            None
        }
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
