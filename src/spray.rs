//! Spray can types.

use crate::doom::spray::{sprays as doom_sprays, Spray as DoomSpray};

use std::ops::Deref;
use std::sync::Arc;

/// An easily cloneable spray can.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spray(Arc<DoomSpray>);

impl From<DoomSpray> for Spray {
    fn from(value: DoomSpray) -> Self {
        Spray(Arc::new(value))
    }
}

impl Deref for Spray {
    type Target = DoomSpray;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A list of all sprays in the base game.
pub fn sprays() -> im::Vector<Spray> {
    doom_sprays()
        .into_iter()
        .map(|spray| Spray::from(spray))
        .collect()
}
