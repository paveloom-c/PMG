//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

mod cli;
mod goal;
mod model;
mod utils;

use goal::Goal;
use model::Model;

use anyhow::{Context, Result};

/// Run the program
pub fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse();
    // Initialize a model from the input data
    let mut model = Model::<f64>::try_from(&args)
        .with_context(|| "Couldn't load the data from the input files")?;
    // Perform computations based on the goals
    model
        .compute(&args.goals)
        .with_context(|| "Couldn't perform computations")?;
    // Write the model data to files in the
    // output directory based on the goals
    model
        .write_to(&args.output, &args.goals)
        .with_context(|| "Couldn't write the model data")?;
    Ok(())
}
