//! Return an iterator over objects
//! that allows modifying each value

use super::super::{Object, Objects};

use std::slice::IterMut;

use num::Float;

impl<F: Float> Objects<F> {
    /// Return an iterator over objects
    /// that allows modifying each value
    pub(in crate::model) fn iter_mut(&mut self) -> IterMut<Object<F>> {
        self.0.iter_mut()
    }
}
