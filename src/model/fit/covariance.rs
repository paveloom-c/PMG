//! Covariance matrix

extern crate alloc;

use super::{Model, OuterOptimizationProblem};
use crate::model::PARAMS_NAMES;
use crate::utils::FiniteDiff;

use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{anyhow, Context, Result};
use argmin::core::ArgminFloat;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use nalgebra::{ComplexField, DMatrix};
use num::Float;
use numeric_literals::replace_float_literals;

impl<F> Model<F> {
    /// Compute the covariance matrix
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn compute_covariance_matrix(&mut self) -> Result<()>
    where
        F: Float
            + Debug
            + Default
            + Display
            + Sum
            + Sync
            + Send
            + ArgminFloat
            + ArgminL2Norm<F>
            + ArgminSub<F, F>
            + ArgminAdd<F, F>
            + ArgminDot<F, F>
            + ArgminMul<F, F>
            + ArgminZeroLike
            + ArgminMul<Vec<F>, Vec<F>>
            + ComplexField,
        Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
        Vec<F>: ArgminSub<F, Vec<F>>,
        Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
        Vec<F>: ArgminAdd<F, Vec<F>>,
        Vec<F>: ArgminMul<F, Vec<F>>,
        Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
        Vec<F>: ArgminL1Norm<F>,
        Vec<F>: ArgminSignum,
        Vec<F>: ArgminMinMax,
        Vec<F>: ArgminDot<Vec<F>, F>,
        Vec<F>: ArgminL2Norm<F>,
        Vec<F>: FiniteDiff<F>,
    {
        let covariance_plain_path = &self.output_dir.join("covariance.plain");
        let covariance_plain_file = File::create(covariance_plain_path)
            .with_context(|| "Couldn't create the `covariance.plain` file")?;
        let mut covariance_plain_writer = BufWriter::new(covariance_plain_file);

        let covariance_dat_path = &self.output_dir.join("covariance.dat");
        let covariance_dat_file = File::create(covariance_dat_path)
            .with_context(|| "Couldn't create the `covariance.dat` file")?;
        let mut covariance_dat_writer = BufWriter::new(covariance_dat_file);

        let n = self.n.unwrap();

        let problem = OuterOptimizationProblem {
            disable_inner: false,
            objects: &self.objects,
            params: &self.params,
            triples: &self.triples,
            output_dir: &self.output_dir,
        };

        let best_p = self.fit_params.as_ref().unwrap().to_vec(n, false);
        let m = best_p.len();

        let hessian_vec = compute_hessian(&problem, &best_p)?;

        writeln!(covariance_plain_writer, "H: ")?;
        write!(covariance_plain_writer, "{:11}", " ")?;
        for name in PARAMS_NAMES.iter().take(m) {
            write!(covariance_plain_writer, " {name:>20}")?;
        }
        writeln!(covariance_plain_writer)?;
        for j in 0..m {
            write!(covariance_plain_writer, "{:>11}", PARAMS_NAMES[j])?;
            for i in 0..m {
                write!(
                    covariance_plain_writer,
                    " {:>20.15}",
                    hessian_vec[i + j * m]
                )?;
            }
            writeln!(covariance_plain_writer)?;
        }

        let hessian_matrix = DMatrix::from_vec(m, m, hessian_vec);

        writeln!(covariance_plain_writer, "\nminors: ")?;
        for k in 2..=m {
            let determinant = hessian_matrix.view((0, 0), (k, k)).determinant();
            writeln!(covariance_plain_writer, "{k:>2}: {determinant:.15}")?;
        }

        let covariance_matrix = hessian_matrix
            .try_inverse()
            .ok_or(anyhow!("Couldn't compute the inverse of the Hessian"))?;

        writeln!(covariance_plain_writer, "\nC: ")?;
        write!(covariance_plain_writer, "{:11}", " ")?;
        for name in PARAMS_NAMES.iter().take(m) {
            write!(covariance_plain_writer, " {name:>20}")?;
        }
        writeln!(covariance_plain_writer)?;
        for (j, row) in covariance_matrix.row_iter().enumerate() {
            write!(covariance_plain_writer, "{:>11}", PARAMS_NAMES[j])?;
            for c in row.iter() {
                write!(covariance_plain_writer, " {c:>20.15}")?;
            }
            writeln!(covariance_plain_writer)?;
        }

        let dispersions = covariance_matrix.diagonal();
        let errors: Vec<F> = dispersions.iter().map(|x| num::Float::sqrt(*x)).collect();

        writeln!(covariance_plain_writer, "\nerrors: ")?;
        for (i, error) in errors.iter().enumerate() {
            writeln!(
                covariance_plain_writer,
                "{:>11}: {error:19.15}",
                PARAMS_NAMES[i]
            )?;
        }

        writeln!(
            covariance_dat_writer,
            "# Linear correlation coefficients, written by rows\nr"
        )?;

        let r_matrix = covariance_matrix.clone();
        writeln!(covariance_plain_writer, "\nR: ")?;
        write!(covariance_plain_writer, "{:11}", " ")?;
        for name in PARAMS_NAMES.iter().take(m) {
            write!(covariance_plain_writer, " {name:>20}")?;
        }
        writeln!(covariance_plain_writer)?;
        for (j, row) in r_matrix.row_iter().enumerate() {
            write!(covariance_plain_writer, "{:>11}", PARAMS_NAMES[j])?;
            for (i, c) in row.iter().enumerate() {
                let r = *c / errors[i] / errors[j];
                write!(covariance_plain_writer, " {r:>20.15}")?;
                writeln!(covariance_dat_writer, "{r}")?;
            }
            writeln!(covariance_plain_writer)?;
        }

        self.covariance_matrix = Some(covariance_matrix);

        Ok(())
    }
}

/// Compute the Hessian at the best point
#[allow(clippy::indexing_slicing)]
#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
fn compute_hessian<F>(problem: &OuterOptimizationProblem<F>, best_p: &Vec<F>) -> Result<Vec<F>>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sum
        + Sync
        + Send
        + ArgminFloat
        + ArgminL2Norm<F>
        + ArgminSub<F, F>
        + ArgminAdd<F, F>
        + ArgminDot<F, F>
        + ArgminMul<F, F>
        + ArgminZeroLike
        + ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<F, Vec<F>>,
    Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
    Vec<F>: ArgminAdd<F, Vec<F>>,
    Vec<F>: ArgminMul<F, Vec<F>>,
    Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminL1Norm<F>,
    Vec<F>: ArgminSignum,
    Vec<F>: ArgminMinMax,
    Vec<F>: ArgminDot<Vec<F>, F>,
    Vec<F>: ArgminL2Norm<F>,
    Vec<F>: FiniteDiff<F>,
{
    let m = best_p.len();
    let m_sq = m * m;
    let mut hessian_vec = vec![0.; m_sq];

    let h = 1e-5;

    // The matrix is symmetric here since
    // we're using the finite differences

    for j in 0..m {
        for i in 0..m {
            let mut p = best_p.clone();
            hessian_vec[i + j * m] = if i == j {
                p[i] = best_p[i] + h;
                let plus_cost = problem.inner_cost(&p, false)?;

                p[i] = best_p[i] - h;
                let minus_cost = problem.inner_cost(&p, false)?;

                let best_cost = problem.inner_cost(best_p, false)?;

                (plus_cost - 2. * best_cost + minus_cost) / h.powi(2)
            } else {
                p[i] = best_p[i] + h;
                p[j] = best_p[j] + h;
                let pp_cost = problem.inner_cost(&p, false)?;

                p[i] = best_p[i] + h;
                p[j] = best_p[j] - h;
                let pm_cost = problem.inner_cost(&p, false)?;

                p[i] = best_p[i] - h;
                p[j] = best_p[j] + h;
                let mp_cost = problem.inner_cost(&p, false)?;

                p[i] = best_p[i] - h;
                p[j] = best_p[j] - h;
                let mm_cost = problem.inner_cost(&p, false)?;

                (pp_cost - pm_cost - mp_cost + mm_cost) / (4. * h.powi(2))
            }
        }
    }

    Ok(hessian_vec)
}
