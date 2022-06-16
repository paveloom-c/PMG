//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

mod cli;
pub mod model;
mod utils;

use model::Model;

use anyhow::{Context, Result};

/// Run the program
///
/// # Errors
///
/// Will return `Err` if:
/// - couldn't load data from either of the data files;
/// - couldn't write the model data
///
/// Will crash if couldn't parse or validate arguments
pub fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse();
    // Initialize a model from the input data
    let model = Model::<f64>::try_from(args.inputs)
        .with_context(|| "Couldn't load the data from the input files")?;
    // Write the model data to files in the output directory
    model
        .write_to(&args.output)
        .with_context(|| "Couldn't write the model data")?;
    Ok(())
}
