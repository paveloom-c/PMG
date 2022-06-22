//! Galactic heliocentric Cartesian coordinates

use super::super::Object;
use crate::utils::to_cartesian;

use std::fmt::Debug;

use anyhow::Result;
use num::Float;

/// Galactic heliocentric Cartesian coordinates
#[derive(Debug)]
pub(in crate::model) struct GalacticCartesian<F: Float> {
    /// X coordinate (kpc)
    pub(in crate::model) x: F,
    /// Y coordinate (kpc)
    pub(in crate::model) y: F,
    /// Z coordinate (kpc)
    pub(in crate::model) z: F,
}

#[allow(clippy::many_single_char_names)]
impl<F: Float> TryFrom<&Object<F>> for GalacticCartesian<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (l, b) = object.galactic_s()?.into();
        let par = object.par()?.v;
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = to_cartesian(l, b, par);
        Ok(Self { x, y, z })
    }
}

impl<F: Float> From<&GalacticCartesian<F>> for (F, F, F) {
    fn from(s: &GalacticCartesian<F>) -> Self {
        (s.x, s.y, s.z)
    }
}
