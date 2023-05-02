//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

extern crate alloc;

mod cli;
mod model;
mod utils;

use cli::Goal;
use model::fit::OuterOptimizationProblem;
use model::{Model, N_MAX};

use alloc::rc::Rc;
use core::cell::RefCell;
use indoc::indoc;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};

/// Run the program
#[allow(clippy::indexing_slicing)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::too_many_lines)]
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

                // Prepare the fit log file
                let fit_log_path = model.output_dir.join("fit.log");
                let fit_log_file = File::create(fit_log_path)
                    .with_context(|| "Couldn't create the `fit.log` file")?;
                let fit_log_writer = Rc::new(RefCell::new(BufWriter::new(fit_log_file)));

                let mut sample_iteration = 0;
                let mut current_nonblacklisted_count = model.objects.borrow().len();
                'outer: loop {
                    'inner: loop {
                        // Try to fit a model with the specified degree
                        model
                            .try_fit(n, sample_iteration, &fit_log_writer)
                            .with_context(|| "Couldn't fit the model")?;

                        // Check the vicinity of the found minimum,
                        // make sure there are no big discrepancies
                        model
                            .try_compute_frozen_profiles(n)
                            .with_context(|| "Couldn't compute the frozen profiles")?;

                        let nonblacklisted_count = model.count_not_blacklisted();
                        if nonblacklisted_count == current_nonblacklisted_count {
                            break 'inner;
                        }

                        current_nonblacklisted_count = nonblacklisted_count;
                        sample_iteration += 1;
                    }

                    // Check if the vicinities of the reduced parallaxes are smooth
                    let problem = OuterOptimizationProblem {
                        objects: &model.objects,
                        params: &model.params,
                        triples: &Rc::clone(&model.triples),
                        output_dir: &model.output_dir,
                    };
                    let best_point = model.fit_params.as_ref().unwrap().to_vec(n);
                    problem.inner_cost(&best_point, false, true, false)?;

                    let nonblacklisted_count = model.count_not_blacklisted();
                    if nonblacklisted_count == current_nonblacklisted_count {
                        break 'outer;
                    }

                    current_nonblacklisted_count = nonblacklisted_count;
                    sample_iteration += 1;
                }

                // Output extra information for the blacklisted objects
                if n == 4 && args.with_blacklisted {
                    let problem = OuterOptimizationProblem {
                        objects: &model.objects,
                        params: &model.params,
                        triples: &Rc::clone(&model.triples),
                        output_dir: &model.output_dir,
                    };
                    let best_point = model.fit_params.as_ref().unwrap().to_vec(n);
                    problem.inner_cost(&best_point, false, false, true)?;
                }
            }

            // Serialize the costs and errors in azimuthal velocity
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
                    let l_1 = model.best_cost.unwrap();
                    let sigma_theta = model.fit_params.as_ref().unwrap().sigma_theta;

                    writeln!(l_1_writer, "{n} {l_1}")?;
                    writeln!(sigma_theta_writer, "{n} {sigma_theta}")?;
                }
            }

            for model in &models {
                std::fs::create_dir_all(&model.output_dir).with_context(|| {
                    format!(
                        "Couldn't create the output directory {:?}",
                        model.output_dir
                    )
                })?;

                if let Some(ref fit_params) = model.fit_params {
                    model
                        .serialize_to_objects("fit_objects", fit_params)
                        .with_context(|| "Couldn't write the objects to a file")?;
                    model
                        .serialize_to_fit_params()
                        .with_context(|| "Couldn't write the fitted parameters to a file")?;
                    model
                        .serialize_to_fit_rotcurve()
                        .with_context(|| "Couldn't write the fitted rotation curve to a file")?;
                }
            }

            // Choose a model for extra computations
            let chosen_i = 0;
            let chosen_n = chosen_i + 1;
            let chosen_model = &mut models[chosen_i];

            let chosen_n_file_path = args.output_dir.join("chosen_n");
            if let Ok(mut chosen_n_file) = File::create(chosen_n_file_path) {
                writeln!(chosen_n_file, "{chosen_n}").ok();
            }

            if args.with_errors {
                chosen_model
                    .try_fit_errors(chosen_n)
                    .with_context(|| "Couldn't define the confidence intervals")?;
                chosen_model
                    .serialize_to_fit_params()
                    .with_context(|| "Couldn't write the fitted parameters to a file")?;
            }
            if args.with_conditional_profiles {
                chosen_model
                    .try_compute_conditional_profiles(chosen_n)
                    .with_context(|| "Couldn't compute the conditional profiles")?;
            }
        }
    }
    Ok(())
}
