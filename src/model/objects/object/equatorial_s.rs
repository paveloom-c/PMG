//! Equatorial spherical coordinates

use crate::model::io::input;
use crate::utils;

use core::fmt::Debug;
use core::str::FromStr;
use std::error::Error;

use anyhow::{bail, Context, Result};
use num::Float;

/// Equatorial spherical coordinates
#[derive(Clone, Debug)]
pub struct EquatorialSpherical<F: Float + Debug> {
    /// Right ascension (radians)
    pub alpha: F,
    /// Declination (radians)
    pub delta: F,
}

impl<F> TryFrom<&input::Record<F>> for EquatorialSpherical<F>
where
    F: Float + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(record: &input::Record<F>) -> Result<Self> {
        // Parse the right ascension string and convert the angle to radians
        let alpha = match utils::str2vec(&record.alpha)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.alpha))?[..]
        {
            [hours, minutes, seconds] => utils::hms2rad(hours, minutes, seconds),
            _ => bail!("Three values were expected"),
        };
        // Parse the declination string and convert the angle to radians
        let delta = match utils::str2vec(&record.delta)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.delta))?[..]
        {
            [degrees, minutes, seconds] => utils::dms2rad(degrees, minutes, seconds),
            _ => bail!("Three values were expected"),
        };
        Ok(Self { alpha, delta })
    }
}

impl<F: Float + Debug> From<&EquatorialSpherical<F>> for (F, F) {
    fn from(s: &EquatorialSpherical<F>) -> Self {
        (s.alpha, s.delta)
    }
}
