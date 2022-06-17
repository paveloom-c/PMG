//! Extend the data of the struct

use crate::model::coordinates::Galactic;

use num::Float;

impl<F: Float> Galactic<F> {
    /// Extend the data of the struct
    pub(in crate::model) fn extend(&mut self, coords: Galactic<F>) {
        self.x.extend(coords.x);
        self.y.extend(coords.y);
        self.z.extend(coords.z);
    }
}
