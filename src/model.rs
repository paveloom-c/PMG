//! Model of the Galaxy

mod io;
mod objects;

use crate::model::io::output;
use crate::Goal;
use objects::{Object, Objects};

use std::error::Error;
use std::fmt::Debug;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug, Default)]
pub struct Model<F: Float + Default + Debug> {
    /// Data objects
    objects: Objects<F>,
}

impl<F: Float + Default + Debug> Model<F> {
    /// Perform computations based on goals
    pub fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        self.objects.compute(goals)?;
        Ok(())
    }
    /// Extend the model by parsing and appending the data
    /// from the file, doing conversions where necessary
    fn extend(&mut self, path: &Path) -> Result<()>
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
    /// Write the model data to files in the
    /// output directory based on the goals
    pub fn write_to(&self, dir: &Path, goals: &[Goal]) -> Result<()>
    where
        F: Serialize,
    {
        // Make sure the output directories exist
        let dat_dir = &dir.join("dat");
        let bin_dir = &dir.join("bin");
        create_dir_all(dat_dir)
            .with_context(|| format!("Couldn't create the output directory {dat_dir:?}"))?;
        create_dir_all(bin_dir)
            .with_context(|| format!("Couldn't create the output directory {bin_dir:?}"))?;
        // Write the coordinates if that was a goal
        if goals.contains(&Goal::Coords) {
            output::coords::serialize_to(dat_dir, bin_dir, self)
                .with_context(|| "Couldn't write the Galactic coordinates to a file")?;
        };
        // Write the rotation curve if that was a goal
        if goals.contains(&Goal::RotationCurve) {
            output::rotcurve::serialize_to(dat_dir, bin_dir, self)
                .with_context(|| "Couldn't write the rotation curve to a file")?;
        };
        Ok(())
    }
}

impl<F> TryFrom<Vec<PathBuf>> for Model<F>
where
    F: Float + Default + Debug + FromStr + DeserializeOwned,
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
