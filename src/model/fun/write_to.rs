//! Write the model data to files inside the directory

use crate::model::io::output;
use crate::model::Model;

use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Context, Result};
use bincode::Options;
use num::Float;
use serde::Serialize;

impl<F: Float> Model<F> {
    /// Write the model data to files inside the directory
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the path isn't a directory;
    /// - couldn't write to a file;
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
        let mut dat_wtr = csv::WriterBuilder::default()
            .delimiter(b' ')
            .from_path(dat_path)
            .with_context(|| format!("Couldn't write to the file {dat_path:?}"))?;
        // Create a writer for the binary file
        let mut bin_wtr = BufWriter::new(
            File::create(&bin_path).with_context(|| "Couldn't write to the file {bin_path:?}")?,
        );
        // Create an options struct for `bincode`
        let bin_options = bincode::DefaultOptions::default()
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
