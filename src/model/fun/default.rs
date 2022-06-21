//! Get an empty model

use super::super::{Model, Objects};

use num::Float;

impl<F: Float> Default for Model<F> {
    fn default() -> Self {
        Self {
            objects: Objects::default(),
        }
    }
}
