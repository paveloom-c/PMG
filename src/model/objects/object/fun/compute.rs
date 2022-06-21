//! Perform computations based on goals

use super::super::Object;
use crate::Goal;

use anyhow::{bail, Context, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        match goals[..] {
            [Goal::Coords] => {
                // Convert equatorial coordinates to Galactic
                // heliocentric spherical coordinates
                self.compute_galactic_s()
                    .with_context(|| "Couldn't compute the Galactic spherical coordinates")?;
                // Convert equatorial coordinates to Galactic
                // heliocentric Cartesian coordinates
                self.compute_galactic_c()
                    .with_context(|| "Couldn't compute the Galactic Cartesian coordinates")?;
            }
            _ => bail!("This combination of goals wasn't expected."),
        }
        Ok(())
    }
}
