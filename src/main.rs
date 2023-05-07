//! This binary crate allows a user to infer the parameters
//! of the Galaxy by optimising over its parametric model.

extern crate alloc;

mod cli;
mod model;
mod utils;

use cli::{Args, Goal};
use model::Model;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};
use indoc::indoc;
use num::Float;

/// Run the program
#[allow(clippy::indexing_slicing)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::print_stderr)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::use_debug)]
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
            let mut models = Vec::with_capacity(args.n);
            let mut fit_log_writers = Vec::with_capacity(args.n);
            for i in 0..args.n {
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

            let discrepancies_log_path = &args.output_dir.join("discrepancies.log");
            let discrepancies_log_file = File::create(discrepancies_log_path)
                .with_context(|| "Couldn't create the `discrepancies.log` file")?;
            let mut discrepancies_log_writer = BufWriter::new(discrepancies_log_file);

            let errors_log_path = &args.output_dir.join("errors.log");
            let errors_log_file = File::create(errors_log_path)
                .with_context(|| "Couldn't create the `errors.log` file")?;
            let errors_log_writer = Rc::new(RefCell::new(BufWriter::new(errors_log_file)));

            writeln!(
                discrepancies_log_writer,
                "{}",
                indoc!(
                    "
                Discrepancies

                `m` is the index of the discrepancy (starting from 1), as in the array [V_r, mu_l', mu_b, par_r]."
                ),
            )?;

            let mut sample_iteration = 0;
            'samples: loop {
                // Fit the parameters for each model
                for i in 0..args.n {
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
                let best_n = best_i + 1;

                // Check the discrepancies of the best model
                {
                    let best_model = &mut models[best_i];
                    let before_nonoutliers_count = best_model.count_non_outliers();

                    writeln!(
                        discrepancies_log_writer,
                        "\nsample_iteration: {sample_iteration}",
                    )?;
                    writeln!(
                        discrepancies_log_writer,
                        "before_nonoutliers_count: {before_nonoutliers_count}"
                    )?;
                    writeln!(discrepancies_log_writer, "best_n: {best_n}")?;

                    let (all_outliers, k_1, k_005) = best_model
                        .check_discrepancies()
                        .with_context(|| "Couldn't check the discrepancies")?;

                    if all_outliers.is_empty() {
                        break 'samples;
                    }

                    writeln!(
                        discrepancies_log_writer,
                        "\nm i{s:3}rel_discrepancy{s:3}k_1{s:15}k_005",
                        s = " "
                    )?;
                    for &(m, i, rel_discrepancy) in &all_outliers {
                        writeln!(
                            discrepancies_log_writer,
                            "{} {:<3} {rel_discrepancy:<17.15} {k_1:<17.15} {k_005:<17.15}",
                            m + 1,
                            i + 1,
                        )?;
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

            discrepancies_log_writer.flush()?;
            for fit_log_writer in fit_log_writers {
                fit_log_writer.borrow_mut().flush()?;
            }

            serialize_n_results(&args, &models)
                .with_context(|| "Couldn't serialize the `n` results")?;

            for i in 0..args.n {
                let n = i + 1;

                let model = &mut models[i];
                if model.fit_params.is_none() {
                    continue;
                }

                model
                    .try_compute_frozen_profiles(n)
                    .with_context(|| "Couldn't compute frozen profiles")?;

                model.post_fit();
                model.write_fit_data()?;

                if args.with_errors {
                    writeln!(errors_log_writer.borrow_mut(), "n: {n}\n")?;
                    let res = model
                        .try_fit_errors(n, &errors_log_writer)
                        .with_context(|| "Couldn't compute the errors");
                    match res {
                        Ok(_) => {
                            model.serialize_to_fit_params().with_context(|| {
                                "Couldn't write the fitted parameters to a file"
                            })?;
                        }
                        Err(ref err) => {
                            eprintln!("{err:?}");
                        }
                    }
                }

                write_fit_params_to_plain(&args, &models)
                    .with_context(|| "Couldn't write to the `fit_params.plain` file")?;
                write_fit_rotcurve_to_plain(&args, &models)
                    .with_context(|| "Couldn't write to the `fit_rotcurve.plain` file")?;
            }

            // Choose a model for extra computations
            let chosen_i = 0;
            let chosen_n = chosen_i + 1;
            let chosen_model = &mut models[chosen_i];

            if args.with_conditional_profiles {
                let res = chosen_model
                    .try_compute_conditional_profiles(chosen_n)
                    .with_context(|| "Couldn't compute the conditional profiles");
                if let Err(ref err) = res {
                    eprintln!("{err:?}");
                }
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

/// Write all fitted parameters to a `plain` file
#[allow(clippy::indexing_slicing)]
fn write_fit_params_to_plain<F>(args: &Args, models: &[Model<F>]) -> Result<()>
where
    F: Float + Debug + Display,
{
    let plain_path = &args.output_dir.join("fit_params.plain");
    let plain_file = File::create(plain_path)
        .with_context(|| format!("Couldn't open the file {plain_path:?} in write-only mode",))?;
    let mut plain_writer = BufWriter::new(plain_file);

    models[0].write_fit_params_header_to_plain(&mut plain_writer)?;
    for (i, model) in models.iter().enumerate() {
        let n = i + 1;
        model.write_fit_params_to_plain(&mut plain_writer, n)?;
    }
    models[0].write_fit_params_footer_to_plain(&mut plain_writer)?;

    Ok(())
}

/// Write all rotation curves to a `plain` file
#[allow(clippy::indexing_slicing)]
fn write_fit_rotcurve_to_plain<F>(args: &Args, models: &[Model<F>]) -> Result<()>
where
    F: Float + Debug + Display,
{
    let plain_path = &args.output_dir.join("fit_rotcurve.plain");
    let plain_file = File::create(plain_path)
        .with_context(|| format!("Couldn't open the file {plain_path:?} in write-only mode",))?;
    let mut plain_writer = BufWriter::new(plain_file);

    models[0].write_fit_rotcurve_header_to_plain(&mut plain_writer)?;
    for (i, model) in models.iter().enumerate() {
        let n = i + 1;
        model.write_fit_rotcurve_to_plain(&mut plain_writer, n)?;
    }

    Ok(())
}
