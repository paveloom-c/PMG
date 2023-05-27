//! Fit of the model (rotation curve)

use super::io::output;
use super::{Model, Params};

use core::fmt::{Debug, Display};

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
    /// Confidence interval error
    sigma: F,
}

impl<F> Model<F> {
    /// Compute the rotation curve based on the fitted parameters
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn compute_fit_rotcurve(&mut self)
    where
        F: Float + Debug,
    {
        let fit_params = self.fit_params.as_ref().unwrap();
        let n = self.n.unwrap();
        let m = fit_params.to_vec(n, false).len();

        let n_points_int = 1000;
        let n_points_float = F::from(n_points_int).unwrap();
        let start = 0.;
        let end = 15.;
        let h = (end - start) / n_points_float;

        self.fit_rotcurve = Some(
            (0..=n_points_int)
                .map(|i_int| {
                    let i_float = F::from(i_int).unwrap();

                    let r_g = start + i_float * h;
                    let theta = compute_rot_curve(r_g, fit_params);

                    let sigma = if let Some(ref covariance_matrix) = self.covariance_matrix {
                        let diffs: Vec<F> = (0..m)
                            .map(|i| compute_rot_curve_partial(r_g, fit_params, n, i))
                            .collect();

                        let mut dispersion = 0.;
                        for k in 0..m {
                            for l in 0..m {
                                dispersion =
                                    dispersion + diffs[k] * diffs[l] * covariance_matrix[(k, l)];
                            }
                        }

                        F::sqrt(dispersion)
                    } else {
                        0.
                    };

                    RotationCurvePoint { r_g, theta, sigma }
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
            # 03 sigma: Confidence interval error [km/s]
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
}

/// Compute the model rotation curve
fn compute_rot_curve<F>(r_g: F, fit_params: &Params<F>) -> F
where
    F: Float + Debug,
{
    let Params { r_0, omega_0, .. } = *fit_params;
    let delta_r_g = r_g - r_0;
    omega_0 * r_g + compute_rot_curve_series(delta_r_g, fit_params)
}

/// Compute a partial derivative of the model rotation curve
#[allow(clippy::indexing_slicing)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
fn compute_rot_curve_partial<F>(r_g: F, fit_params: &Params<F>, n: usize, i: usize) -> F
where
    F: Float + Debug,
{
    let h = 1e-8;

    let best_p = fit_params.to_vec(n, false);

    let mut new_fit_params = fit_params.clone();
    let mut p = best_p.clone();

    p[i] = best_p[i] + h;
    new_fit_params.update_with(&p);
    let plus_f = compute_rot_curve(r_g, &new_fit_params);

    p[i] = best_p[i] - h;
    new_fit_params.update_with(&p);
    let minus_f = compute_rot_curve(r_g, &new_fit_params);

    (plus_f - minus_f) / (2. * h)
}

/// Compute the linear rotation curve approximation via Taylor series
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
