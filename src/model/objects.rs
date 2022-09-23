//! Data objects

mod object;

use crate::model::io::input;
use crate::model::Params;
use crate::Goal;

pub use object::{Measurement, Object};

use core::fmt::{Debug, Display};
use core::slice::{Iter, IterMut};
use core::str::FromStr;
use std::error::Error;
use std::path::Path;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use num::Float;
use serde::de::DeserializeOwned;

/// Data objects
#[derive(Clone, Debug, Default)]
pub struct Objects<F: Float + Default + Debug>(Vec<Object<F>>);

impl<F> Objects<F>
where
    F: Float + Default + Display + Debug,
{
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal], params: &Params<F>) -> Result<()> {
        // Perform computations for each object
        for object in self.iter_mut() {
            object
                .compute(goals, params)
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

/// Parse a record into an object
fn deserialize<F>(result: Result<input::Record<F>, csv::Error>) -> Result<Object<F>>
where
    F: Float + Default + Debug + FromStr,
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
    F: Float + Default + Debug + FromStr + DeserializeOwned,
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
