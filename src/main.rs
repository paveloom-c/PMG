//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

pub mod model;
mod utils;

use model::Model;

use std::path::Path;

use anyhow::{Context, Result};

/// Run the program
///
/// # Errors
///
/// Will return `Err` if:
/// - couldn't load data from either of the data files;
/// - couldn't write the model data
pub fn main() -> Result<()> {
    // Initialize a model from the input data
    let hmsfr_path = Path::new("data/input/hmsfr.dat");
    let non_hmsfr_path = Path::new("data/input/non-hmsfr.dat");
    let mut model = Model::<f64>::try_from(hmsfr_path)
        .with_context(|| format!("Couldn't load the data from {hmsfr_path:?}"))?;
    model
        .extend(non_hmsfr_path)
        .with_context(|| format!("Couldn't load the data from {non_hmsfr_path:?}"))?;
    // Write the model data to files in the directory
    model
        .write_to(Path::new("data/output"))
        .with_context(|| "Couldn't write the model data")?;
    Ok(())
}
