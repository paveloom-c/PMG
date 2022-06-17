//! Get the default value of the type

use crate::model::coordinates::Equatorial;

use num::Float;

impl<F: Float> Default for Equatorial<F> {
    fn default() -> Self {
        Self {
            alpha: Vec::<F>::default(),
            delta: Vec::<F>::default(),
            par: Vec::<F>::default(),
        }
    }
}
