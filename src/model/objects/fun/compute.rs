//! Perform computations based on goals

use super::super::Objects;
use crate::Goal;

use anyhow::{Context, Result};
use num::Float;

impl<F: Float> Objects<F> {
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        // Perform computations for each object
        for object in self.iter_mut() {
            object
                .compute(goals)
                .with_context(|| "Couldn't perform computations for an object")?;
        }
        Ok(())
    }
}
