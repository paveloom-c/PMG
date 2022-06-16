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
    let data_path = Path::new("data/input/data.dat");
    let model = Model::<f64>::try_from(data_path)
        .with_context(|| format!("Couldn't load the data from {data_path:?}"))?;
    // Write the model data to files in the directory
    model
        .write_to(Path::new("data/output"))
        .with_context(|| "Couldn't write the model data")?;
    Ok(())
}
