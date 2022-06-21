//! Write the model data to files inside the directory

use super::super::Model;
use crate::model::io::output;

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};
use bincode::Options;
use indoc::indoc;
use num::Float;
use serde::Serialize;

/// Header of the `coords.dat` file
const COORDS_CSV_HEADER: &str = indoc! {"
    # Galactic coordinates of the objects in the Cartesian system
    #
    # Descriptions:
    #
    # name: Name of the object
    # x: X coordinate [kpc] [f64]
    # y: Y coordinate [kpc] [f64]
    # z: Z coordinate [kpc] [f64]
    # obj_type: Type of the object
    # source: Source of the data
    #\n
"};

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
        // Open files for writing
        let mut dat_file = File::create(&dat_path)
            .with_context(|| format!("Couldn't open the file {dat_path:?} in write-only mode"))?;
        let bin_file = File::create(&bin_path)
            .with_context(|| format!("Couldn't open the file {bin_path:?} in write-only mode"))?;
        // Write the header to the text file
        write!(&mut dat_file, "{}", COORDS_CSV_HEADER)
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
        // Create a vector of records
        let records = output::coords::Records::from(self);
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
}
