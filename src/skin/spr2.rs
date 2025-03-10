//! Implementation of the ZDoom SPR2 system.
//!
//! These don't actually store data, they just tell other related systems where
//! to find the data, for easier lookup.

use ahash::{HashMap, HashSet};

use wad::Name;

use crate::doom::patch::Patch;
use crate::lump::Lump;

use super::{Error, FromNameError, FromNameErrorKind, SpriteAngle};

use std::io::Cursor;

/// A SPR2 map.
#[derive(Clone, Debug, Default)]
pub struct Index {
    sprites: HashMap<SpriteIndex, Spr2>,
    patches: HashMap<Name, Lump>,
}

impl Index {
    /// Creates a new `Index`.
    pub fn new() -> Index {
        Index::default()
    }

    /// Adds a Doom graphic to the `Index`.
    ///
    /// If this method returns `Err`, the `Index` will still be in a valid
    /// state.
    pub fn add(&mut self, name: Name, patch: Lump) -> Result<(), Error> {
        if name.as_str().len() < 6 {
            return Err(FromNameError {
                name,
                kind: FromNameErrorKind::InvalidLength(name.as_str().len()),
            }
            .into());
        }

        // get identifier from name
        let identifier = Name::from_bytes(&name[..4]).expect("valid subname");

        // get default sprite
        let frame = name[4];
        let Some(angle) = SpriteAngle::from_ascii_char(name[5]) else {
            return Err(FromNameError {
                name,
                kind: FromNameErrorKind::InvalidAngle(name[5] as char),
            }
            .into());
        };

        // get mirror sprite
        if name.as_str().len() >= 8 {
            let frame = name[6];
            let Some(angle) = SpriteAngle::from_ascii_char(name[7]) else {
                return Err(FromNameError {
                    name,
                    kind: FromNameErrorKind::InvalidAngle(name[7] as char),
                }
                .into());
            };

            self.insert(Spr2::new(
                SpriteIndex::new(identifier, frame, angle),
                name,
                true,
            ));
        }

        self.insert(Spr2::new(
            SpriteIndex::new(identifier, frame, angle),
            name,
            false,
        ));

        // add patch
        self.patches.insert(name, patch);

        Ok(())
    }

    /// Gets a patch.
    pub fn read(&self, name: &Name) -> Result<Patch, Error> {
        self.patches
            .get(name)
            .ok_or_else(|| Error::NotFound(name.to_string()))
            .and_then(|lump| lump.clone().read())
            .and_then(|bytes| Patch::read(Cursor::new(bytes)).map_err(From::from))
    }

    /// Iterates over all the unique sprite names.
    pub fn iter(&self) -> impl Iterator<Item = Name> {
        self.sprites
            .values()
            .map(|spr| spr.index.name)
            .collect::<HashSet<_>>()
            .into_iter()
    }

    /// Iterates over all the frames of a sprite.
    pub fn iter_frames(&self, name: &Name) -> impl Iterator<Item = u8> {
        self.sprites
            .values()
            .filter(|spr| spr.index.name == *name)
            .map(|spr| spr.index.frame)
            .collect::<HashSet<_>>()
            .into_iter()
    }

    /// Iterates over all the angles of a sprite frame.
    pub fn iter_angles<'a>(
        &'a self,
        name: &'a Name,
        frame: u8,
    ) -> impl Iterator<Item = &'a Spr2> + 'a {
        self.sprites
            .values()
            .filter(move |spr| spr.index.name == *name && spr.index.frame == frame)
    }

    fn insert(&mut self, spr2: Spr2) {
        self.sprites.insert(spr2.index, spr2);
    }
}

/// A frame index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpriteIndex {
    /// The base name of the sprite.
    pub name: Name,
    /// The frame.
    pub frame: u8,
    /// The angle.
    pub angle: SpriteAngle,
}

impl SpriteIndex {
    /// Creates a new `FrameIndex`.
    pub fn new(name: Name, frame: u8, angle: SpriteAngle) -> SpriteIndex {
        SpriteIndex { name, frame, angle }
    }
}

/// A single, indexed Spr2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spr2 {
    pub index: SpriteIndex,
    /// The full name of the Doom graphic to find.
    pub name: Name,
    /// Whether to mirror the graphic to produce the final sprite.
    pub mirror: bool,
}

impl Spr2 {
    /// Creates a new `Spr2`.
    pub fn new(index: SpriteIndex, name: Name, mirror: bool) -> Spr2 {
        Spr2 {
            index,
            name,
            mirror,
        }
    }
}
