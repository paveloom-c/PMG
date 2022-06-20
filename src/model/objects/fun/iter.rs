//! Return the inner iterator

use super::super::{Object, Objects};

use num::Float;

impl<F: Float> Objects<F> {
    /// Return the inner iterator
    pub(in crate::model) fn iter(&self) -> impl Iterator<Item = &Object<F>> {
        self.0.iter()
    }
}
