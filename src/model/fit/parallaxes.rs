//! Do something with reduced parallaxes

use crate::Model;

use core::fmt::{Debug, Display};
use std::io::Write;
use std::{fs::File, io::BufWriter};

use anyhow::{Context, Result};
use indoc::formatdoc;
use itertools::izip;
use num::{Float, Integer};
use numeric_literals::replace_float_literals;

use super::Triple;

impl<F> Model<F> {
    /// Write the parallaxes (original and reduced ones)
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn write_parallaxes(&self) -> Result<()>
    where
        F: Float + Debug + Display,
    {
        if self.fit_params.is_none() {
            return Ok(());
        }

        let parallaxes_path = &self.output_dir.join("parallaxes.dat");
        let parallaxes_file = File::create(parallaxes_path)
            .with_context(|| "Couldn't create the `parallaxes.dat` file")?;
        let mut parallaxes_writer = BufWriter::new(parallaxes_file);

        writeln!(parallaxes_writer, "# Parallaxes")?;
        writeln!(parallaxes_writer, "i par par_e par_r name source")?;

        for (i, (object, triples)) in
            izip!(self.objects.borrow().iter(), self.triples.borrow().iter()).enumerate()
        {
            if object.outlier {
                continue;
            }
            let triple = &triples[3];

            writeln!(
                parallaxes_writer,
                "{} {} {} {} \"{}\" \"{}\"",
                i + 1,
                triple.observed,
                triple.error,
                triple.model,
                object.name.as_ref().unwrap(),
                object.source.as_ref().unwrap(),
            )?;
        }

        Ok(())
    }
    /// Compute the systematical error in parallaxes
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::integer_division)]
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

        let triples: Vec<Triple<F>> =
            izip!(self.objects.borrow().iter(), self.triples.borrow().iter())
                .filter_map(|(object, triples)| {
                    if object.outlier {
                        None
                    } else {
                        let triple = &triples[3];
                        Some(triple.clone())
                    }
                })
                .collect();

        let a = 0.;

        let mut p = 0.;
        let mut p_x = 0.;
        let mut p_x_sq = 0.;

        for triple in &triples {
            let x_i = triple.model - triple.observed;
            let p_i = 1. / triple.error.powi(2);

            p = p + p_i;
            p_x = p_x + p_i * (x_i - a);
            p_x_sq = p_x_sq + p_i * (x_i - a).powi(2);
        }

        let x_mean = 1. / p * p_x + a;
        let sigma = F::sqrt(1. / (n_objects - 1.) * (p_x_sq - p * (x_mean - a).powi(2)));
        let sigma_x_mean = sigma / p.sqrt();
        let sigma_sigma = sigma / F::sqrt(2. * (n_objects - 1.));

        // Fun fact: it's called reciprocal
        let sigma_r = 1. / sigma;
        let sigma_sigma_r = sigma_sigma / sigma.powi(2);

        let mut p_x_mean_sq = 0.;

        for triple in &triples {
            let x_i = triple.model - triple.observed;
            let p_i = 1. / triple.error.powi(2);

            p_x_mean_sq = p_x_mean_sq + p_i * (x_i - x_mean).powi(2);
        }

        let sigma_stroke = F::sqrt(1. / p * p_x_mean_sq);

        let mut sum_sigma_par_sq = 0.;

        for triple in &triples {
            sum_sigma_par_sq = sum_sigma_par_sq + triple.error.powi(2);
        }

        let sigma_par_mean = F::sqrt(1. / n_objects * sum_sigma_par_sq);

        let mut errors: Vec<F> = triples.iter().map(|triple| triple.error).collect();
        errors.sort_by(|i, j| i.partial_cmp(j).unwrap());
        let errors_len = errors.len();
        let sigma_par_median = if errors_len.is_odd() {
            errors[(errors_len / 2)]
        } else {
            let ind_left = errors_len / 2 - 1;
            let ind_right = errors_len / 2;
            (errors[ind_left] + errors[ind_right]) / 2.
        };

        writeln!(
            plain_writer,
            "{}",
            formatdoc!(
                "
                {s:25}n:  {n}

                {s:16}\\sum_i p_i: {p:>23.15}
                {s:7}\\sum_i p_i(x_i - a): {p_x:>23.15}
                {s:5}\\sum_i p_i(x_i - a)^2: {p_x_sq:>23.15}

                {s:14}\\overline{{x}}: {x_mean:>23.15}
                {s:7}\\sigma_\\overline{{x}}: {sigma_x_mean:>23.15}
                {s:20}\\sigma: {sigma:>23.15}
                {s:13}\\sigma_\\sigma: {sigma_sigma:>23.15}
                {s:16}1 / \\sigma: {sigma_r:>23.15}
                {s:7}\\sigma_{{1 / \\sigma}}: {sigma_sigma_r:>23.15}

                \\sum_i p_i(x_i - x_mean)^2: {p_x_mean_sq:>23.15}

                {s:19}\\sigma': {sigma_stroke:>23.15}
                {s:2}\\overline{{\\sigma_\\varpi}}: {sigma_par_mean:>23.15}
                {s:5}\\tilde{{\\sigma_\\varpi}}: {sigma_par_median:>23.15}
                ",
                s = " ",
            ),
        )?;

        writeln!(
            dat_writer,
            "{n} {x_mean} {sigma_x_mean} {sigma} {sigma_sigma} {sigma_r} {sigma_sigma_r} {sigma_stroke} {sigma_par_mean} {sigma_par_median}"
        )?;

        Ok(())
    }
}
