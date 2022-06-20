//! Extend the vector of objects

use super::super::Objects;

use num::Float;

impl<F: Float> Objects<F> {
    /// Extend the vector of objects
    pub(in crate::model) fn extend(&mut self, objects: Self) {
        self.0.extend(objects.0);
    }
}
