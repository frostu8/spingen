//! Spray can types and loading.

pub mod loaders;

use crate::doom::spray::{sprays as doom_sprays, Spray as DoomSpray};

use std::ops::Deref;
use std::sync::Arc;

use wasm_bindgen::prelude::*;

/// An easily cloneable spray can.
#[derive(Clone, Debug, Default, PartialEq)]
#[wasm_bindgen]
pub struct Spray(Arc<DoomSpray>);

#[wasm_bindgen]
impl Spray {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

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
pub fn sprays() -> Vec<Spray> {
    doom_sprays()
        .into_iter()
        .map(|spray| Spray::from(spray))
        .collect()
}
