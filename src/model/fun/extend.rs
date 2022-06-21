//! Extend the model by parsing and appending the data
//! from the file, doing conversions where necessary

use super::super::{Model, Objects};

use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use num::Float;
use serde::de::DeserializeOwned;

impl<F: Float> Model<F> {
    /// Extend the model by parsing and appending the data
    /// from the file, doing conversions where necessary
    pub fn extend(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Debug + FromStr + DeserializeOwned,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the data from the file
        let objects = Objects::try_from(path).with_context(|| "Couldn't parse the data")?;
        // Extend the model
        self.objects.extend(objects);
        Ok(())
    }
}
