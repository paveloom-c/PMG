//! Get the default value of the type

use crate::model::coordinates::Galactic;

use num::Float;

impl<F: Float> Default for Galactic<F> {
    fn default() -> Self {
        Self {
            x: Vec::<F>::default(),
            y: Vec::<F>::default(),
            z: Vec::<F>::default(),
        }
    }
}
