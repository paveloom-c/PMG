//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

extern crate alloc;

mod cli;
mod model;
mod utils;

use cli::{Args, Goal};
use model::{Model, N_MAX};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};
use indoc::indoc;

/// Run the program
#[allow(clippy::indexing_slicing)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::unwrap_used)]
pub fn main() -> Result<()> {
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
            let mut fit_log_writers = Vec::with_capacity(N_MAX);
            for i in 0..N_MAX {
                let n = i + 1;
                let output_dir = args.output_dir.join(format!("n = {n}"));

                let model = Model::<f64>::try_from(&args, output_dir)
                    .with_context(|| "Couldn't load the data from the input files")?;

                let fit_log_path = model.output_dir.join("fit.log");
                let fit_log_file = File::create(fit_log_path)
                    .with_context(|| "Couldn't create the `fit.log` file")?;
                let fit_log_writer = Rc::new(RefCell::new(BufWriter::new(fit_log_file)));

                models.push(model);
                fit_log_writers.push(fit_log_writer);
            }

            let mut sample_iteration = 0;
            'samples: loop {
                // Fit the parameters for each model
                for i in 0..N_MAX {
                    let n = i + 1;

                    let model = &mut models[i];
                    let fit_log_writer = &fit_log_writers[i];

                    // Try to fit a model with the specified degree
                    model
                        .try_fit_params(n, sample_iteration, fit_log_writer)
                        .with_context(|| "Couldn't fit the model")?;
                }

                // Choose the best model
                let best_i = 4;

                // Check the discrepancies of the best model
                {
                    let best_model = &mut models[best_i];

                    let before_nonoutliers_count = best_model.count_non_outliers();
                    best_model
                        .check_discrepancies()
                        .with_context(|| "Couldn't check the discrepancies")?;
                    let after_nonoutliers_count = best_model.count_non_outliers();

                    if before_nonoutliers_count == after_nonoutliers_count {
                        break 'samples;
                    }
                }

                // Update the outliers
                let outliers_mask = models[best_i].get_outliers_mask();
                for i in 0..models.len() {
                    if i != best_i {
                        let model = &mut models[i];
                        model.apply_outliers_mask(&outliers_mask);
                    }
                }

                sample_iteration += 1;
            }

            serialize_n_results(&args, &models)
                .with_context(|| "Couldn't serialize the `n` results")?;

            for i in 0..N_MAX {
                let n = i + 1;

                let model = &mut models[i];
                if model.fit_params.is_none() {
                    continue;
                }

                model
                    .try_compute_frozen_profiles(n)
                    .with_context(|| "Couldn't compute frozen profiles")?;

                if args.with_errors {
                    model
                        .try_fit_errors(n)
                        .with_context(|| "Couldn't define the confidence intervals")?;
                }

                model.post_fit();
                model.write_fit_data()?;
            }

            // Choose a model for extra computations
            let chosen_i = 0;
            let chosen_n = chosen_i + 1;
            let chosen_model = &mut models[chosen_i];

            if args.with_conditional_profiles {
                chosen_model
                    .try_compute_conditional_profiles(chosen_n)
                    .with_context(|| "Couldn't compute the conditional profiles")?;
            }
        }
    }
    Ok(())
}

/// Serialize the costs and errors in azimuthal velocity
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
fn serialize_n_results<F>(args: &Args, models: &[Model<F>]) -> Result<()>
where
    F: Display,
{
    let l_1_file = File::create(args.output_dir.join("L_1.dat"))?;
    let mut l_1_writer = BufWriter::new(l_1_file);
    let sigma_theta_file = File::create(args.output_dir.join("sigma_theta.dat"))?;
    let mut sigma_theta_writer = BufWriter::new(sigma_theta_file);

    writeln!(
        l_1_writer,
        indoc!(
            "
            # Best costs (L_1) as a dependency of the degree
            # of the polynomial of the rotation curve
            n L_1"
        )
    )?;

    writeln!(
        sigma_theta_writer,
        indoc!(
            "
            # Errors in the azimuthal velocity as a dependency of
            # the degree of the polynomial of the rotation curve
            n sigma_theta"
        )
    )?;

    for (i, model) in models.iter().enumerate() {
        if model.fit_params.is_none() {
            continue;
        };

        let n = i + 1;
        let l_1 = model.best_cost.as_ref().unwrap();
        let sigma_theta = &model.fit_params.as_ref().unwrap().sigma_theta;

        writeln!(l_1_writer, "{n} {l_1}")?;
        writeln!(sigma_theta_writer, "{n} {sigma_theta}")?;
    }

    Ok(())
}
