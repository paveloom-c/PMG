//! Extend the model by parsing and appending the data
//! from the file, doing conversions where necessary

use crate::model::coordinates::Galactic;
use crate::model::io::input::Data;
use crate::model::Model;

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
    ///
    /// # Errors
    ///
    /// Will return `Err` if deserializing data from the file wasn't successful
    pub fn extend(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Debug + FromStr + DeserializeOwned,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the data from the file
        let data = Data::try_from(path).with_context(|| "Couldn't parse the data")?;
        // Extend the model
        self.names.extend(data.names);
        self.coords.extend(Galactic::from(data.coords));
        self.obj_types.extend(data.obj_types);
        Ok(())
    }
}
