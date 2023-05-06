//! Fit of the model (rotation curve)

use super::io::output;
use super::{Model, Params};

use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use anyhow::Result;
use indoc::formatdoc;
use num::Float;
use numeric_literals::replace_float_literals;
use serde::Serialize;

/// Rotation curve
pub type RotationCurve<F> = Vec<RotationCurvePoint<F>>;

/// A point on the rotation curve
#[derive(Debug, Clone, Serialize)]
pub struct RotationCurvePoint<F> {
    /// Galactocentric distance [kpc]
    #[serde(rename = "R")]
    r_g: F,
    /// Azimuthal velocity [km/s]
    theta: F,
}

impl<F> Model<F> {
    /// Compute the rotation curve based on the fitted parameters
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn compute_fit_rotcurve(&mut self)
    where
        F: Float,
    {
        // Unpack some of the parameters
        let fit_params = self.fit_params.as_ref().unwrap();
        let Params { r_0, omega_0, .. } = *fit_params;
        // Compute the rotation curve (linear model for now)
        let n_int = 1000;
        let n_float = F::from(n_int).unwrap();
        let start = 0.;
        let end = 15.;
        let h = (end - start) / n_float;
        self.fit_rotcurve = Some(
            (0..=n_int)
                .map(|i_int| {
                    let i_float = F::from(i_int).unwrap();
                    // Compute the Galactocentric distance
                    let r_g = start + i_float * h;
                    // Compute the azimuthal velocity
                    let delta_r_g = r_g - r_0;
                    let theta = omega_0 * r_g + compute_rot_curve_series(delta_r_g, fit_params);
                    RotationCurvePoint { r_g, theta }
                })
                .collect(),
        );
    }
    /// Serialize the fitted rotation curve
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn serialize_to_fit_rotcurve(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        // Prepare a header
        let fit_params = self.fit_params.as_ref().unwrap();
        let header = formatdoc!(
            "
            # Fit of the model (rotation curve)
            {sample_description}
            # Descriptions:
            #
            # 01 R: Galactocentric distance to the Sun [kpc]
            # 02 theta: Azimuthal velocity [km/s]
            #
            # Parameters used:
            #
            # Galactocentric distance to the Sun [kpc]
            # R_0: {r_0}
            #
            # Circular velocity of the Sun at R = R_0 [km/s/kpc]
            # OMEGA_0: {omega_0}
            #
            # Oort's A constant [km/s/kpc]
            # A: {a}
            #
            ",
            sample_description = self.format_sample_description(),
            r_0 = fit_params.r_0,
            omega_0 = fit_params.omega_0,
            a = fit_params.a,
        );
        let records = self.fit_rotcurve.as_ref().unwrap();
        output::serialize_to(&self.output_dir, "fit_rotcurve", &header, records)
    }
    /// Write the header to the plain file
    pub fn write_fit_rotcurve_header_to_plain(
        &self,
        plain_writer: &mut BufWriter<File>,
    ) -> Result<()>
    where
        F: Display,
    {
        write!(
            plain_writer,
            "{}",
            formatdoc!(
                "
            Fits of the models (rotation curves)
            {sample_description}
            Fitted rotation curves:
            ",
                sample_description = self
                    .format_sample_description()
                    .replace("# ", "")
                    .replace('#', ""),
            )
        )?;
        Ok(())
    }
    /// Write the rotation curve points to the plain file
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn write_fit_rotcurve_to_plain(
        &self,
        plain_writer: &mut BufWriter<File>,
        n: usize,
    ) -> Result<()>
    where
        F: Display,
    {
        let fit_rotcurve = self.fit_rotcurve.as_ref().unwrap();
        writeln!(plain_writer, "\nn i{s:4}R{s:18}theta", s = " ")?;
        for (i, point) in fit_rotcurve.iter().enumerate() {
            writeln!(
                plain_writer,
                "{n} {i:<4} {:<18.15} {:<.15}",
                point.r_g, point.theta,
            )?;
        }
        Ok(())
    }
}

/// Compute the linear rotation curve approximation
/// via Taylor series with the specified degree
#[allow(clippy::as_conversions)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::indexing_slicing)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_rot_curve_series<F>(delta_r_g: F, fit_params: &Params<F>) -> F
where
    F: Float,
{
    let Params {
        a,
        theta_2,
        theta_3,
        theta_4,
        theta_5,
        theta_6,
        theta_7,
        theta_8,
        ..
    } = *fit_params;

    -2. * a * delta_r_g
        + theta_2 / 2. * (delta_r_g).powi(2)
        + theta_3 / 6. * (delta_r_g).powi(3)
        + theta_4 / 24. * (delta_r_g).powi(4)
        + theta_5 / 120. * (delta_r_g).powi(5)
        + theta_6 / 720. * (delta_r_g).powi(6)
        + theta_7 / 5040. * (delta_r_g).powi(7)
        + theta_8 / 40320. * (delta_r_g).powi(8)
}
