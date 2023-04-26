//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

mod cli;
mod model;
mod utils;

use cli::Goal;
use model::{Model, N_MAX};

use anyhow::{Context, Result};

/// Run the program
#[allow(clippy::indexing_slicing)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::unwrap_used)]
pub fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse();
    match args.goal {
        Goal::Objects => {
            let output_dir = args.output_dir.join("objects");
            let mut model = Model::<f64>::try_from(&args, output_dir)
                .with_context(|| "Couldn't load the data from the input files")?;
            model.compute_objects();
            model
                .write_objects_data()
                .with_context(|| "Couldn't write the model data")?;
        }
        Goal::Fit => {
            // Prepare several models
            let mut models = Vec::with_capacity(N_MAX);
            for i in 0..N_MAX {
                let n = i + 1;
                let output_dir = args.output_dir.join(format!("n = {n}"));
                let model = Model::<f64>::try_from(&args, output_dir)
                    .with_context(|| "Couldn't load the data from the input files")?;
                models.push(model);
            }

            // For each model
            for i in 0..N_MAX {
                let model = &mut models[i];
                let n = i + 1;
                // Try to fit a model with the specified degree
                model.try_fit(n).with_context(|| "Couldn't fit the model")?;
            }

            // Choose the best fit
            let mut best_i = 0;
            {
                let mut best_cost = models[best_i].best_cost.unwrap();
                for j in 1..N_MAX {
                    let cost = models[j].best_cost.unwrap();
                    if cost < best_cost {
                        best_i = j;
                        best_cost = cost;
                    }
                }
            }
            let best_model = &mut models[best_i];
            let best_n = best_i + 1;

            if args.with_errors {
                best_model
                    .try_fit_errors(best_n)
                    .with_context(|| "Couldn't define the confidence intervals")?;
            }
            if args.with_conditional_profiles {
                best_model
                    .try_compute_conditional_profiles(best_n)
                    .with_context(|| "Couldn't compute the conditional profiles")?;
            }
            if args.with_frozen_profiles {
                best_model
                    .try_compute_frozen_profiles(best_n)
                    .with_context(|| "Couldn't compute the frozen profiles")?;
            }

            for model in &models {
                model
                    .write_fit_data()
                    .with_context(|| "Couldn't write the model data")?;
            }
        }
    }
    Ok(())
}
