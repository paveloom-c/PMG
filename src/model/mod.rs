//! Model of the Galaxy

mod coordinates;
mod io;

use crate::model::io::input::Data;
use crate::model::io::output;
use coordinates::Galactic;

use std::error::Error;
use std::fmt::Debug;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use bincode::Options;
use itertools::izip;
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Type for names of the objects
pub type Names = Vec<String>;

/// Type for types of the objects
pub type ObjTypes = Vec<String>;

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F: Float> {
    /// Names of the objects
    names: Names,
    /// Coordinates (in the Galactic heliocentric Cartesian system)
    coords: Galactic<F>,
    /// Types of the objects
    obj_types: ObjTypes,
}

impl<F: Float> Model<F> {
    /// Create a new instance of the struct
    fn new() -> Self {
        Self {
            names: Names::new(),
            coords: Galactic::new(),
            obj_types: ObjTypes::new(),
        }
    }
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
    /// Write the model data to files inside the directory
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the path isn't a directory;
    /// - couldn't write to the file;
    /// - couldn't serialize a record
    pub fn write_to(&self, dir: &Path) -> Result<()>
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
        // Define paths to the text and the binary files
        let dat_path = &dat_dir.join("coords.dat");
        let bin_path = &bin_dir.join("coords.bin");
        // Create a writer for the text file
        let mut dat_wtr = csv::WriterBuilder::new()
            .delimiter(b' ')
            .from_path(dat_path)
            .with_context(|| format!("Couldn't write to the file {dat_path:?}"))?;
        // Create a writer for the binary file
        let mut bin_wtr = BufWriter::new(
            File::create(&bin_path).with_context(|| "Couldn't write to the file {bin_path:?}")?,
        );
        // Create an options struct for `bincode`
        let bin_options = bincode::DefaultOptions::new()
            .with_little_endian()
            .with_fixint_encoding();
        // Create a vector of records
        let records = output::coords::Records::from(self);
        // Write the records in the binary format
        bin_options
            .serialize_into(&mut bin_wtr, &records)
            .with_context(|| format!("Couldn't write a record to {bin_path:?}"))?;
        // For each record
        for record in records {
            // Write it in the text format
            dat_wtr
                .serialize(&record)
                .with_context(|| format!("Couldn't write a record to {dat_path:?}"))?;
        }
        Ok(())
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
        let mut model = Model::new();
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
