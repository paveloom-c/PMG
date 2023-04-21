//! Fit of the model (rotation curve)

use super::io::output;
use super::Model;

use core::fmt::{Debug, Display};
use std::path::Path;

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
        let r_0 = fit_params.r_0;
        let theta_0 = fit_params.theta_0;
        let theta_1 = fit_params.theta_1;
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
                    let theta = theta_0 + theta_1 * delta_r_g;
                    RotationCurvePoint { r_g, theta }
                })
                .collect(),
        );
    }
    /// Serialize the fitted parameters
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_fit_rotcurve(&self, output_dir: &Path) -> Result<()>
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
        output::serialize_to(output_dir, "fit_rotcurve", &header, records)
    }
}
