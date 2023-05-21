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
use indoc::formatdoc;
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
            let mut models = Vec::with_capacity(args.n_max);
            let mut fit_log_writers = Vec::with_capacity(args.n_max);
            for i in 0..args.n_max {
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

            let outliers_log_path = &args.output_dir.join("outliers.log");
            let outliers_log_file = File::create(outliers_log_path)
                .with_context(|| "Couldn't create the `outliers.log` file")?;
            let mut outliers_log_writer = BufWriter::new(outliers_log_file);

            let errors_log_path = &args.output_dir.join("errors.log");
            let errors_log_file = File::create(errors_log_path)
                .with_context(|| "Couldn't create the `errors.log` file")?;
            let errors_log_writer = Rc::new(RefCell::new(BufWriter::new(errors_log_file)));

            let n_data_path = &args.output_dir.join("n.dat");
            let mut n_data_file =
                File::create(n_data_path).with_context(|| "Couldn't create the `n.dat` file")?;

            writeln!(
                outliers_log_writer,
                "{}",
                indoc!(
                    "
                Outliers

                `m` is the index of the discrepancy (starting from 1), as in the array [V_r, mu_l', mu_b, par_r]."
                ),
            )?;

            let best_i = args.n_best - 1;
            let best_n = args.n_best;

            writeln!(n_data_file, "# Progression of the number of the objects\nn")?;
            writeln!(n_data_file, "{}", models[best_i].objects.borrow().len())?;

            let mut sample_iteration = 0;
            for l_stroke in [3, 1] {
                'samples: loop {
                    // Fit the parameters for each model
                    for i in 0..args.n_max {
                        let n = i + 1;

                        let model = &mut models[i];
                        let fit_log_writer = &fit_log_writers[i];

                        // Try to fit a model with the specified degree
                        model
                            .try_fit_params(n, sample_iteration, l_stroke, fit_log_writer)
                            .with_context(|| "Couldn't fit the model")?;
                    }

                    // Check for the outliers via the best model
                    {
                        let best_model = &mut models[best_i];
                        let before_nonoutliers_count = best_model.count_non_outliers();

                        writeln!(
                            outliers_log_writer,
                            "\nsample_iteration: {sample_iteration}",
                        )?;
                        writeln!(
                            outliers_log_writer,
                            "before_nonoutliers_count: {before_nonoutliers_count}"
                        )?;
                        writeln!(outliers_log_writer, "best_n: {best_n}")?;
                        writeln!(outliers_log_writer, "l_stroke: {l_stroke}")?;

                        if args.disable_outliers {
                            break 'samples;
                        }

                        let (one_dimensional_outliers, multi_dimensional_outliers) = best_model
                            .find_outliers(l_stroke)
                            .with_context(|| "Couldn't check for outliers")?;

                        if one_dimensional_outliers.vec.is_empty()
                            && multi_dimensional_outliers.vec.is_empty()
                        {
                            break 'samples;
                        }

                        let objects = best_model.objects.borrow();
                        if !one_dimensional_outliers.vec.is_empty() {
                            writeln!(
                                outliers_log_writer,
                                "\none-dimensional:\nm{s:4}rel_discrepancy{s:1}kappa{s:13}k_005{s:13}i{s:3}source name",
                                s = " "
                            )?;
                            for &(m, i, rel_discrepancy) in &one_dimensional_outliers.vec {
                                let object = &objects[i];
                                writeln!(
                                    outliers_log_writer,
                                    "{} {rel_discrepancy:>18.15} {:<17.15} {:<17.15} {:<3} {:6} {}",
                                    m + 1,
                                    one_dimensional_outliers.kappa,
                                    one_dimensional_outliers.k_005,
                                    i + 1,
                                    object.source.as_ref().unwrap(),
                                    object.name.as_ref().unwrap(),
                                )?;
                            }
                        }
                        if !multi_dimensional_outliers.vec.is_empty() {
                            writeln!(
                                outliers_log_writer,
                                "\n{}:\n{s:18}z{s:1}kappa{s:14}k_005{s:14}i{s:3}source name",
                                if best_model.disable_inner {
                                    "three-dimensional"
                                } else {
                                    "four-dimensional"
                                },
                                s = " ",
                            )?;
                            for &(i, rel_discrepancy) in &multi_dimensional_outliers.vec {
                                let object = &objects[i];
                                writeln!(
                                    outliers_log_writer,
                                    "{rel_discrepancy:>19.15} {:<18.15} {:<18.15} {:<3} {:6} {}",
                                    multi_dimensional_outliers.kappa,
                                    multi_dimensional_outliers.k_005,
                                    i + 1,
                                    object.source.as_ref().unwrap(),
                                    object.name.as_ref().unwrap(),
                                )?;
                            }
                        }

                        outliers_log_writer.flush()?;
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

                outliers_log_writer.flush()?;
                for fit_log_writer in &fit_log_writers {
                    fit_log_writer.borrow_mut().flush()?;
                }

                let l_stroke_n = models[best_i].count_non_outliers();
                writeln!(n_data_file, "{l_stroke_n}")?;

                for i in 0..args.n_max {
                    let n = i + 1;

                    let model = &mut models[i];
                    if model.fit_params.is_none() {
                        continue;
                    }

                    if l_stroke == 1 {
                        model.l_stroke_1_n = Some(l_stroke_n);
                    } else {
                        model.l_stroke_3_n = Some(l_stroke_n);
                    }

                    model
                        .try_compute_frozen_profiles(l_stroke)
                        .with_context(|| "Couldn't compute frozen profiles")?;

                    if args.with_errors {
                        writeln!(errors_log_writer.borrow_mut(), "n: {n}\n")?;
                        let res = model
                            .try_fit_errors(&errors_log_writer, l_stroke)
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

                    errors_log_writer.borrow_mut().flush()?;

                    if l_stroke == 1 {
                        model.post_fit();
                        model.write_fit_data()?;

                        if !model.disable_inner {
                            model
                                .write_parallaxes()
                                .with_context(|| "Couldn't write the parallaxes to a file")?;

                            if n == best_n {
                                model.analyze_inner_profiles().with_context(|| {
                                    "Couldn't compute the profiles of the inner targer function"
                                })?;
                            }
                        }

                        write_fit_rotcurve_to_plain(&args, &models)
                            .with_context(|| "Couldn't write to the `fit_rotcurve.plain` file")?;
                    }

                    write_fit_params_to_plain(&args, &models)
                        .with_context(|| "Couldn't write to the `fit_params.plain` file")?;
                }

                if !args.disable_inner && l_stroke == 1 {
                    write_delta_varpi(&args, &models, l_stroke_n)?;
                }

                let best_model = &mut models[best_i];
                if args.with_conditional_profiles {
                    let res = best_model
                        .try_compute_conditional_profiles(l_stroke)
                        .with_context(|| "Couldn't compute the conditional profiles");
                    if let Err(ref err) = res {
                        eprintln!("{err:?}");
                    }
                }
            }

            serialize_n_results(&args, &models)
                .with_context(|| "Couldn't serialize the `n` results")?;
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

/// Write all mean systematical errors to a `plain` file
#[allow(clippy::indexing_slicing)]
#[allow(clippy::unwrap_used)]
fn write_delta_varpi<F>(args: &Args, models: &[Model<F>], l_stroke_1_n: usize) -> Result<()>
where
    F: Float + Debug + Display,
{
    let plain_path = &args.output_dir.join("Delta_varpi.plain");
    let plain_file =
        File::create(plain_path).with_context(|| "Couldn't create the `Delta_varpi.plain` file")?;
    let mut plain_writer = BufWriter::new(plain_file);

    let dat_path = &args.output_dir.join("Delta_varpi.dat");
    let dat_file =
        File::create(dat_path).with_context(|| "Couldn't create the `Delta_varpi.dat` file")?;
    let mut dat_writer = BufWriter::new(dat_file);

    writeln!(
        plain_writer,
        "{}",
        formatdoc!(
            "
            Mean systematical error in the parallaxes

            N: {l_stroke_1_n}
            a: 0",
        ),
    )?;

    writeln!(
        dat_writer,
        "{}",
        formatdoc!(
            "
            # Mean systematical error in the parallaxes
            #
            # N: {l_stroke_1_n}
            # a: 0
            n x_mean sigma_x_mean sigma sigma_sigma sigma_r sigma_sigma_r sigma_stroke sigma_par_mean",
        ),
    )?;

    for model in models.iter() {
        model.write_delta_varpi(&mut plain_writer, &mut dat_writer)?;
    }

    Ok(())
}
