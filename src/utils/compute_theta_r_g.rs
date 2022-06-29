//! Compute the azimuthal velocity and Galactocentric distance

use super::compute_r_g::r_0_2;
use super::{compute_r_g_2, to_spherical};

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Standard Solar Motion toward GC (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const U_SUN_STANDARD: f64 = 10.3;

/// Peculiar motion locally toward GC (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
const U_SUN: f64 = 10.7;

/// Standard Solar Motion toward l = 90 degrees (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const V_SUN_STANDARD: f64 = 15.3;

/// Full circular velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
const THETA_SUN: f64 = 247.;

/// Standard Solar Motion toward NGP (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const W_SUN_STANDARD: f64 = 7.7;

/// Linear velocities units conversion coefficient
///
/// Sources: Gromov, Nikiforov (2016)
const K: f64 = 4.7406;

/// Standard Solar Motion toward GC (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn u_sun_standard<F: Float + Debug>() -> F {
    F::from(U_SUN_STANDARD).unwrap()
}

/// Peculiar motion locally toward GC (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn u_sun<F: Float + Debug>() -> F {
    F::from(U_SUN).unwrap()
}

/// Standard Solar Motion toward l = 90 degrees (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn v_sun_standard<F: Float + Debug>() -> F {
    F::from(V_SUN_STANDARD).unwrap()
}

/// Full circular velocity of the Sun (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn theta_sun<F: Float + Debug>() -> F {
    F::from(THETA_SUN).unwrap()
}

/// Standard Solar Motion toward NGP (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn w_sun_standard<F: Float + Debug>() -> F {
    F::from(W_SUN_STANDARD).unwrap()
}

/// Linear velocities units conversion coefficient
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn k<F: Float + Debug>() -> F {
    F::from(K).unwrap()
}

/// Compute the azimuthal velocity and Galactocentric distance
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::many_single_char_names)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_theta_r_g<F: Float + Debug>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    par: F,
    v_lsr: F,
    mu_x: F,
    mu_y: F,
) -> (F, F) {
    // Wrap the constants
    let u_sun_standard: F = u_sun_standard();
    let u_sun: F = u_sun();
    let theta_sun: F = theta_sun();
    let v_sun_standard: F = v_sun_standard();
    let w_sun_standard: F = w_sun_standard();
    let k: F = k();
    let r_0_2: F = r_0_2();
    // Compute the heliocentric velocity
    let v_h = v_lsr
        - (u_sun_standard * l.cos() + v_sun_standard * l.sin()) * b.cos()
        - w_sun_standard * b.sin();
    // Convert the proper motions in equatorial
    // coordinates from mas/yr to rad/yr
    let mu_alpha = (mu_x / delta.cos() / 3600. / 1000.).to_radians();
    let mu_delta = (mu_y / 3600. / 1000.).to_radians();
    // Compute the proper motions in Galactic coordinates
    // (the difference in the coordinates in 1-year period)
    let (l_ahead, b_ahead) = to_spherical(alpha + mu_alpha, delta + mu_delta);
    let mu_l = l_ahead - l;
    let mu_b = b_ahead - b;
    // Convert the proper motions in Galactic
    // coordinates from rad/yr to mas/yr
    let mu_l = mu_l.to_degrees() * 3600. * 1000.;
    let mu_b = mu_b.to_degrees() * 3600. * 1000.;
    // Compute the heliocentric distance
    let r_h = 1. / par;
    // Compute the linear velocities
    let v_l = k * r_h * mu_l * b.cos();
    let v_b = k * r_h * mu_b;
    // Convert the velocities to the Cartesian
    // heliocentric coordinate system
    let v_aux = v_h * b.cos() - v_b * b.sin();
    let u = v_aux * l.cos() - v_l * l.sin();
    let v = v_aux * l.sin() + v_l * l.cos();
    // Convert to the Galactocentric coordinate
    // system associated with the Sun
    let u_g = u + u_sun;
    let v_g = v + theta_sun;
    // Compute the projection of the heliocentric distance in the XY plane
    let d = r_h * b.cos();
    // Compute the Galactocentric distance
    let r_g = compute_r_g_2(l, b, r_h);
    // Compute the azimuthal velocity
    let sin_beta = d / r_g * l.sin();
    let cos_beta = (r_0_2 - d * l.cos()) / r_g;
    let theta = v_g * cos_beta + u_g * sin_beta;
    (theta, r_g)
}

/// Compute the azimuthal velocity from the array of arguments
pub(super) fn compute_theta<F: Float + Debug>(args: &[F; 8]) -> F {
    compute_theta_r_g(
        args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7],
    )
    .0
}
