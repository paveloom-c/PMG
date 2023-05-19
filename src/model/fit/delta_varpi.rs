//! Compute the systematical error in parallaxes

use crate::Model;

use core::fmt::{Debug, Display};
use std::io::Write;
use std::{fs::File, io::BufWriter};

use anyhow::Result;
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
    pub fn write_delta_varpi(
        &self,
        plain_writer: &mut BufWriter<File>,
        dat_writer: &mut BufWriter<File>,
    ) -> Result<()>
    where
        F: Float + Debug + Display,
    {
        if self.fit_params.is_none() {
            return Ok(());
        }

        let n = self.n.unwrap();
        let n_objects = F::from(self.l_stroke_1_n.unwrap()).unwrap();

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
        let sigma = F::sqrt(1. / (n_objects - 1.) * (p_x_sq - p * (x_mean - a).powi(2)));
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

        writeln!(
            plain_writer,
            "{}",
            formatdoc!(
                "
                {s:25}n:  {n}

                {s:16}\\sum_i p_i: {p:>18.15}
                {s:7}\\sum_i p_i(x_i - a): {p_x:>18.15}
                {s:5}\\sum_i p_i(x_i - a)^2: {p_x_sq:>18.15}

                {s:14}\\overline{{x}}: {x_mean:>18.15}
                {s:7}\\sigma_\\overline{{x}}: {sigma_x_mean:>18.15}
                {s:20}\\sigma: {sigma:>18.15}

                \\sum_i p_i(x_i - x_mean)^2: {p_x_mean_sq:>18.15}

                {s:19}\\sigma': {sigma_stroke:>18.15}
                ",
                s = " ",
            ),
        )?;

        writeln!(
            dat_writer,
            "{n} {x_mean} {sigma_x_mean} {sigma} {sigma_stroke}"
        )?;

        Ok(())
    }
}
