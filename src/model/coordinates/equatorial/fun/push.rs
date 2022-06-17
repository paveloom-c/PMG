//! Parse values from the record and push them to the storage

use crate::model::coordinates::Equatorial;

use std::error::Error;
use std::str::FromStr;

use crate::model::io::input::Record;
use crate::utils::{dms2rad, hms2rad, str2vec};

use anyhow::{Context, Result};
use num::Float;

impl<F: Float> Equatorial<F> {
    /// Parse values from the record and push them to the storage
    ///
    /// # Errors
    ///
    /// Will return `Err` if parsing either the right
    /// ascension or the declination wasn't successful
    pub(in crate::model) fn push(&mut self, record: &Record<F>) -> Result<()>
    where
        F: FromStr,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the right ascension string and convert the angle to radians, push
        if let [hours, minutes, seconds] = str2vec(&record.alpha)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.alpha))?[..]
        {
            let alpha = hms2rad(hours, minutes, seconds);
            self.alpha.push(alpha);
        }
        // Parse the declination string and convert the angle to radians, push
        if let [degrees, minutes, seconds] = str2vec(&record.delta)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.delta))?[..]
        {
            let delta = dms2rad(degrees, minutes, seconds);
            self.delta.push(delta);
        }
        // Push the parallax
        self.par.push(record.par);
        Ok(())
    }
}
