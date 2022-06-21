//! Return an iterator over objects

use super::super::{Object, Objects};

use std::slice::Iter;

use num::Float;

impl<F: Float> Objects<F> {
    /// Return an iterator over objects
    pub(in crate::model) fn iter(&self) -> Iter<Object<F>> {
        self.0.iter()
    }
}
