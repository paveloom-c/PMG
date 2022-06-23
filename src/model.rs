//! Model of the Galaxy

mod io;
mod objects;

use crate::model::io::output;
use crate::Goal;
use objects::{Object, Objects};

use std::error::Error;
use std::fmt::Debug;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use bincode::Options;
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F: Float + Debug> {
    /// Data objects
    objects: Objects<F>,
}

impl<F: Float + Debug> Model<F> {
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
    /// Serialize records to the file
    fn serialize_to(
        dat_dir: &Path,
        bin_dir: &Path,
        name: &str,
        header: &str,
        records: Vec<impl Serialize>,
    ) -> Result<()> {
        // Define paths to the text and the binary files
        let dat_path = &dat_dir.join(format!("{name}.dat"));
        let bin_path = &bin_dir.join(format!("{name}.bin"));
        // Open files for writing
        let mut dat_file = File::create(&dat_path)
            .with_context(|| format!("Couldn't open the file {dat_path:?} in write-only mode"))?;
        let bin_file = File::create(&bin_path)
            .with_context(|| format!("Couldn't open the file {bin_path:?} in write-only mode"))?;
        // Write the header to the text file
        write!(&mut dat_file, "{}", header)
            .with_context(|| format!("Couldn't write the header to {dat_path:?}"))?;
        // Create a CSV writer for the text file
        let mut dat_wtr = csv::WriterBuilder::default()
            .delimiter(b' ')
            .from_writer(dat_file);
        // Create a `bincode` writer for the binary file
        let mut bin_wtr = BufWriter::new(bin_file);
        // Create an options struct for `bincode`
        let bin_options = bincode::DefaultOptions::default()
            .with_little_endian()
            .with_fixint_encoding();
        // Write the records in the binary format
        bin_options
            .serialize_into(&mut bin_wtr, &records)
            .with_context(|| format!("Couldn't write records to {bin_path:?}"))?;
        // For each record
        for record in records {
            // Write it in the text format
            dat_wtr
                .serialize(&record)
                .with_context(|| format!("Couldn't write a record to {dat_path:?}"))?;
        }
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
            Model::<F>::serialize_to(
                dat_dir,
                bin_dir,
                "coords",
                output::coords::COORDS_CSV_HEADER,
                output::coords::Records::try_from(self)
                    .with_context(|| "Couldn't construct records from objects")?,
            )
            .with_context(|| "Couldn't write the Galactic coordinates to a file")?;
        };
        // Write the rotation curve if that was a goal
        if goals.contains(&Goal::RotationCurve) {
            Model::<F>::serialize_to(
                dat_dir,
                bin_dir,
                "rotcurve",
                output::rotcurve::ROTCURVE_CSV_HEADER,
                output::rotcurve::Records::try_from(self)
                    .with_context(|| "Couldn't construct records from objects")?,
            )
            .with_context(|| "Couldn't write the Galactic coordinates to a file")?;
        };
        Ok(())
    }
}

impl<F: Float + Debug> Default for Model<F> {
    fn default() -> Self {
        Self {
            objects: Objects::default(),
        }
    }
}

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
