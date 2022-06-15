//! Input data

use super::Record;
use crate::model::coordinates::Equatorial;
use crate::model::Names;

use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use num::Float;
use serde::de::DeserializeOwned;

/// Input data
pub(in crate::model) struct Data<F: Float> {
    /// Names of the objects
    pub(in crate::model) names: Names,
    /// Coordinates (in the equatorial system)
    pub(in crate::model) coords: Equatorial<F>,
}

impl<F> TryFrom<&Path> for Data<F>
where
    F: Float + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        // Create a reader
        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(path)
            .with_context(|| format!("Couldn't read from the file {path:?}"))?;
        // Prepare storage
        let mut names = Names::new();
        let mut coords = Equatorial::<F>::new();
        // For each record in the reader
        for result in rdr.deserialize() {
            // Try to deserialize the record
            let record: Record<F> = result.with_context(|| "Couldn't deserialize the record")?;
            // Push the data to the according storage
            names.push(record.name.clone());
            coords.push(&record)?;
        }
        Ok(Data { names, coords })
    }
}
