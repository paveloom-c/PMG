//! Output related

pub mod fit_params;
pub mod objects;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};
use bincode::Options;
use serde::Serialize;

/// Serialize records to the files
pub fn serialize_to(
    dat_dir: &Path,
    bin_dir: &Path,
    name: &str,
    header: &str,
    records: &[impl Serialize],
) -> Result<()> {
    // Define paths to the text and the binary files
    let dat_path = &dat_dir.join(format!("{name}.dat"));
    let bin_path = &bin_dir.join(format!("{name}.bin"));
    // Open files for writing
    let mut dat_file = File::create(dat_path)
        .with_context(|| format!("Couldn't open the file {dat_path:?} in write-only mode"))?;
    let bin_file = File::create(bin_path)
        .with_context(|| format!("Couldn't open the file {bin_path:?} in write-only mode"))?;
    // Write the header to the text file
    write!(&mut dat_file, "{header}")
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
            .serialize(record)
            .with_context(|| format!("Couldn't write a record to {dat_path:?}"))?;
    }
    Ok(())
}
