//! Perform computations based on goals

use super::super::Model;
use crate::Goal;

use anyhow::Result;
use num::Float;

impl<F: Float> Model<F> {
    /// Perform computations based on goals
    pub fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        self.objects.compute(goals)?;
        Ok(())
    }
}
