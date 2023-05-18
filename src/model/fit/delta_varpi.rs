//! Compute the systematical error in parallaxes

use crate::Model;

use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;

use anyhow::{Context, Result};
use indoc::formatdoc;
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;

impl<F> Model<F> {
    /// Compute the systematical error in parallaxes
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn compute_delta_varpi(&self) -> Result<()>
    where
        F: Float + Debug + Display,
    {
        let n = F::from(self.l_stroke_1_n.unwrap()).unwrap();

        let a = 0.;

        let mut p = 0.;
        let mut p_x = 0.;
        let mut p_x_sq = 0.;

        for (object, triples) in izip!(self.objects.borrow().iter(), self.triples.borrow().iter()) {
            if object.outlier {
                continue;
            }
            let triple = &triples[3];

            let x_i = triple.model - triple.observed;
            let p_i = triple.error.powi(2);

            p = p + p_i;
            p_x = p_x + p_i * (x_i - a);
            p_x_sq = p_x_sq + p_i * (x_i - a).powi(2);
        }

        let x_mean = 1. / p * p_x + a;
        let sigma = F::sqrt(1. / (n - 1.) * (p_x_sq - p * (x_mean - a).powi(2)));
        let sigma_x_mean = sigma / p.sqrt();

        let mut p_x_mean_sq = 0.;

        for (object, triples) in izip!(self.objects.borrow().iter(), self.triples.borrow().iter()) {
            if object.outlier {
                continue;
            }
            let triple = &triples[3];

            let x_i = triple.model - triple.observed;
            let p_i = triple.error.powi(2);

            p_x_mean_sq = p_x_mean_sq + p_i * (x_i - x_mean).powi(2);
        }

        let sigma_stroke = F::sqrt(1. / p * p_x_mean_sq);

        let delta_varpi_log_path = &self.output_dir.join("Delta_varpi.plain");
        let mut delta_varpi_log_file = File::create(delta_varpi_log_path)
            .with_context(|| "Couldn't create the `Delta_varpi.plain` file")?;

        writeln!(
            delta_varpi_log_file,
            "{}",
            formatdoc!(
                "
                Mean systematical error in the parallaxes

                n: {n}
                a: {a}

                {s:16}\\sum_i p_i: {p:>18.15}
                {s:7}\\sum_i p_i(x_i - a): {p_x:>18.15}
                {s:5}\\sum_i p_i(x_i - a)^2: {p_x_sq:>18.15}

                {s:14}\\overline{{x}}: {x_mean:>18.15}
                {s:20}\\sigma: {sigma:>18.15}
                {s:7}\\sigma_\\overline{{x}}: {sigma_x_mean:>18.15}

                \\sum_i p_i(x_i - x_mean)^2: {p_x_mean_sq:>18.15}

                {s:19}\\sigma': {sigma_stroke:>18.15}
                ",
                s = " ",
            ),
        )?;

        Ok(())
    }
}
