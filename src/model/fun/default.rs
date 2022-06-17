//! Get an empty model

use crate::model::{Galactic, Model, Names, ObjTypes};

use num::Float;

impl<F: Float> Default for Model<F> {
    fn default() -> Self {
        Self {
            names: Names::default(),
            coords: Galactic::default(),
            obj_types: ObjTypes::default(),
        }
    }
}
