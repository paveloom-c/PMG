//! Data objects

mod object;

use crate::model::io::input;
use crate::Goal;
pub(in crate::model) use object::Object;

use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::slice::{Iter, IterMut};
use std::str::FromStr;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use num::Float;
use serde::de::DeserializeOwned;

/// Data objects
#[derive(Debug)]
pub struct Objects<F: Float + Debug>(Vec<Object<F>>);

impl<F: Float + Debug> Objects<F> {
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        // Perform computations for each object
        for object in self.iter_mut() {
            object
                .compute(goals)
                .with_context(|| "Couldn't perform computations for an object")?;
        }
        Ok(())
    }
    /// Extend the vector of objects
    pub(in crate::model) fn extend(&mut self, objects: Self) {
        self.0.extend(objects.0);
    }
    /// Return an iterator over objects
    pub(in crate::model) fn iter(&self) -> Iter<Object<F>> {
        self.0.iter()
    }
    /// Return an iterator over objects
    /// that allows modifying each value
    pub(in crate::model) fn iter_mut(&mut self) -> IterMut<Object<F>> {
        self.0.iter_mut()
    }
}

impl<F: Float + Debug> Default for Objects<F> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

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
