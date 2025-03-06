//! Discovers and compiles all skins.

use crate::doom::skin::SkinDefine;

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use derive_more::Deref;

/// The main `Skin` type.
///
/// Stores some useful, cached data.
#[derive(Clone, Debug)]
pub struct Skin {
    data: Arc<SkinData>,
}

impl From<SkinData> for Skin {
    fn from(value: SkinData) -> Self {
        Skin {
            data: Arc::new(value),
        }
    }
}

impl Deref for Skin {
    type Target = SkinData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// The actual internal skin data.
///
/// Contains information about the skin, and all the patches associated with
/// it. This data never changes, so it is exchanged around in an [`Arc`].
#[derive(Clone, Debug, Deref)]
pub struct SkinData {
    /// The path of the skin.
    pub path: PathBuf,
    /// The skin description.
    #[deref]
    pub skin: SkinDefine,
}

/// A patch.
///
/// Exposes methods for encoding the patch data into more usable formats.
#[derive(Clone, Debug)]
pub struct Patch {
    data: Vec<u8>,
}

impl Patch {
    /// Creates a new patch from its raw bytes.
    pub fn new(data: impl Into<Vec<u8>>) -> Patch {
        Patch { data: data.into() }
    }
}
