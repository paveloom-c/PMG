//! Convert equatorial coordinates to Galactic
//! heliocentric Cartesian coordinates

use super::super::{GalacticCartesian, Object};

use anyhow::{bail, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Convert equatorial coordinates to Galactic
    /// heliocentric Cartesian coordinates
    pub(super) fn compute_galactic_c(&mut self) -> Result<()> {
        match self.galactic_s {
            Some(ref galactic_s) => self.galactic_c.replace(GalacticCartesian::from(galactic_s)),
            None => bail!("Couldn't unwrap the Galactic spherical coordinates"),
        };
        Ok(())
    }
}
