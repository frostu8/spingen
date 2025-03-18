//! Spraycan selection, otherwise known as palettes.
//!
//! Named [`Spray`] so it isn't confused with a palette, like the PLAYPAL kind.

mod basegame;

pub use basegame::sprays;

use super::patch::Palette;

/// A single spray.
///
/// A palette is effectively just a color mapping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spray {
    /// The slot of the palette, which is used as the palette's identifier.
    pub id: String,
    /// The name of the palette.
    pub name: String,
    /// The actual description of the palette.
    pub ramp: [u8; 16],
}

impl Spray {
    /// Remaps a palette with the spraycan colors and a startcolor.
    pub fn remap(&self, palette: &Palette, startcolor: usize) -> Palette {
        // create a new palette
        let mut new_palette = palette.clone();

        for (i, new_color_ix) in self.ramp.iter().enumerate() {
            // remap from old palette
            new_palette[startcolor + i] = palette[*new_color_ix as usize];
        }

        new_palette
    }
}

impl Default for Spray {
    fn default() -> Spray {
        basegame::sprays().first().cloned().expect("valid spray")
    }
}
