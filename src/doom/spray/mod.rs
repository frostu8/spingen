//! Spraycan selection, otherwise known as palettes.
//!
//! Named [`Spray`] so it isn't confused with a palette, like the PLAYPAL kind.

mod basegame;

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

impl Default for Spray {
    fn default() -> Spray {
        basegame::sprays().first().cloned().expect("valid spray")
    }
}
