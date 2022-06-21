//! Unwrap the Galactic heliocentric Cartesian coordinates

use super::super::{GalacticCartesian, Object};

use anyhow::{anyhow, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Unwrap the Galactic heliocentric Cartesian coordinates
    pub(in crate::model) fn galactic_c(&self) -> Result<&GalacticCartesian<F>> {
        self.galactic_c
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactic Cartesian coordinates"))
    }
}
