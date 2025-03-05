//! Spraycan selection, otherwise known as palettes.
//!
//! Named [`Spray`] so it isn't confused with a palette, like the PLAYPAL kind.

mod basegame;

use std::collections::HashMap;

use ahash::RandomState;

/// A collection of sprays.
#[derive(Clone, Debug)]
pub struct SpraySet {
    sprays: HashMap<String, Spray, RandomState>,
}

impl SpraySet {
    /// Creates a new, empty `SpraySet`.
    pub fn new() -> SpraySet {
        SpraySet {
            sprays: HashMap::default(),
        }
    }

    /// Inserts a spray into the `SpraySet`.
    pub fn insert(&mut self, spray: Spray) -> Option<Spray> {
        self.sprays.insert(spray.id.clone(), spray)
    }
}

impl Default for SpraySet {
    fn default() -> Self {
        basegame::default()
    }
}

/// A single spray.
///
/// A palette is effectively just a color mapping.
#[derive(Clone, Debug)]
pub struct Spray {
    /// The slot of the palette, which is used as the palette's identifier.
    pub id: String,
    /// The name of the palette.
    pub name: String,
    /// The actual description of the palette.
    pub ramp: [u8; 16],
}
