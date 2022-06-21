//! Construct a model from a path to a file with data

use super::super::Model;

use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result};
use num::Float;
use serde::de::DeserializeOwned;

impl<F> TryFrom<Vec<PathBuf>> for Model<F>
where
    F: Float + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(paths: Vec<PathBuf>) -> Result<Self> {
        // Initialize an empty model
        let mut model = Model::default();
        // Extend it using the data from the files
        for path in paths {
            model
                .extend(&path)
                .with_context(|| format!("Couldn't load the data from the file {path:?}"))?;
        }
        // Return the result
        Ok(model)
    }
}
