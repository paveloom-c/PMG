//! Get data objects from the file with the given path

use super::super::{Object, Objects};
use crate::model::io::input;

use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use num::Float;
use serde::de::DeserializeOwned;

/// Parse a record into an object
fn deserialize<F>(result: Result<input::Record<F>, csv::Error>) -> Result<Object<F>>
where
    F: Float + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    // Try to deserialize the record
    let record: input::Record<F> = result.with_context(|| "Couldn't deserialize a record")?;
    // Parse an object from the record
    let object =
        Object::try_from(record).with_context(|| "Couldn't parse a record into an object")?;
    Ok(object)
}

impl<F> TryFrom<&Path> for Objects<F>
where
    F: Float + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        // Create a reader
        let mut rdr = ReaderBuilder::default()
            .delimiter(b' ')
            .comment(Some(b'#'))
            .from_path(path)
            .with_context(|| format!("Couldn't read from the file {path:?}"))?;
        // Try to collect objects
        let objects = rdr
            .deserialize()
            .map(deserialize)
            .collect::<Result<Vec<Object<F>>>>()
            .with_context(|| format!("Couldn't get objects from the file {path:?}"))?;
        Ok(Objects(objects))
    }
}
