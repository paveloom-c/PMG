//! Convert equatorial coordinates to Galactic
//! heliocentric spherical coordinates

use super::super::{GalacticSpherical, Object};

use anyhow::{bail, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Convert equatorial coordinates to Galactic
    /// heliocentric spherical coordinates
    pub(super) fn compute_galactic_s(&mut self) -> Result<()> {
        match self.equatorial_s {
            Some(ref equatorial_s) => self
                .galactic_s
                .replace(GalacticSpherical::from(equatorial_s)),
            None => bail!("Couldn't get the equatorial coordinates"),
        };
        Ok(())
    }
}
