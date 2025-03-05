//! ZDoom skins, and skin definitions.

use serde::Deserialize;

/// A skin definition.
#[derive(Clone, Debug, Deserialize)]
pub struct Skin {
    /// The name that identifies this skin.
    pub name: String,
    /// The real, display name of the skin.
    ///
    /// Note: In Ring Racers, underscores in this name will be replaced with
    /// spaces in UI, so `spingen` will replace any underscores with spaces.
    pub realname: String,
    /// The start color for spray replacement.
    pub startcolor: u8,
    /// The preferred color of the racer.
    ///
    /// In `spingen`, it will automatically select this color.
    pub perfcolor: String,
}
