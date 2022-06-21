//! Unwrap the Galactic heliocentric spherical coordinates

use super::super::{GalacticSpherical, Object};

use anyhow::{anyhow, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Unwrap the Galactic heliocentric spherical coordinates
    pub(in crate::model) fn galactic_s(&self) -> Result<&GalacticSpherical<F>> {
        self.galactic_s
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactic spherical coordinates"))
    }
}
