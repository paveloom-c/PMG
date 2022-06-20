//! Default value

use super::super::Objects;

use num::Float;

impl<F: Float> Default for Objects<F> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
