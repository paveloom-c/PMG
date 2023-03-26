//! Equatorial heliocentric spherical coordinates

use crate::model::io::input;
use crate::model::Object;
use crate::utils;

use core::fmt::Debug;
use core::str::FromStr;
use std::error::Error;

use anyhow::{bail, Context, Result};
use num::Float;

impl<F> Object<F>
where
    F: Float + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    /// Parse the right ascension string and convert the angle to radians
    pub(in crate::model) fn try_parse_alpha(&mut self, record: &input::Record<F>) -> Result<()> {
        self.alpha = Some(
            match utils::str2vec(&record.alpha)
                .with_context(|| format!("Couldn't parse the string {:?}", &record.alpha))?[..]
            {
                [hours, minutes, seconds] => utils::hms2rad(hours, minutes, seconds),
                _ => bail!("Three values were expected"),
            },
        );
        Ok(())
    }
    /// Parse the declination string and convert the angle to radians
    pub(in crate::model) fn try_parse_delta(&mut self, record: &input::Record<F>) -> Result<()> {
        self.delta = Some(
            match utils::str2vec(&record.delta)
                .with_context(|| format!("Couldn't parse the string {:?}", &record.delta))?[..]
            {
                [degrees, minutes, seconds] => utils::dms2rad(degrees, minutes, seconds),
                _ => bail!("Three values were expected"),
            },
        );
        Ok(())
    }
}
