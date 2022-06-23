//! Compute the azimuthal velocity and Galactocentric distance

use super::compute_r_g::{compute_r_g_2, r_0_2};
use super::to_spherical::{alpha_ngp, delta_ngp};

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Standard value of the U component of the
/// peculiar velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const U_SUN_STANDARD: f64 = 10.3;

/// U component of the peculiar velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
const U_SUN: f64 = 10.7;

/// Standard value of the V component of the
/// peculiar velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const V_SUN_STANDARD: f64 = 15.3;

/// Standard value of the W component of the
/// peculiar velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
const W_SUN_STANDARD: f64 = 7.7;

/// Linear velocity of the rotation of the Sun
/// around the center of the Galaxy (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
const THETA_SUN: f64 = 247.;

/// Velocity components factor
///
/// Sources: Gromov, Nikiforov (2021)
const K: f64 = 4.7406;

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// U component of the peculiar velocity of the Sun (km/s)
fn u_sun<F: Float + Debug>() -> F {
    F::from(U_SUN).unwrap()
}

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// Standard value of the U component of the
/// peculiar velocity of the Sun (km/s)
fn u_sun_standard<F: Float + Debug>() -> F {
    F::from(U_SUN_STANDARD).unwrap()
}

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// Linear velocity of the rotation of the Sun
/// around the center of the Galaxy (km/s)
fn theta_sun<F: Float + Debug>() -> F {
    F::from(THETA_SUN).unwrap()
}

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// Standard value of the V component of the
/// peculiar velocity of the Sun (km/s)
fn v_sun_standard<F: Float + Debug>() -> F {
    F::from(V_SUN_STANDARD).unwrap()
}

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// Standard value of the W component of the
/// peculiar velocity of the Sun (km/s)
fn w_sun_standard<F: Float + Debug>() -> F {
    F::from(W_SUN_STANDARD).unwrap()
}

#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
/// Velocity components factor
fn k<F: Float + Debug>() -> F {
    F::from(K).unwrap()
}

/// Compute the azimuthal velocity and Galactocentric distance
///
/// Sources: Gromov, Nikiforov, Ossipkov (2016); Poleski
#[allow(clippy::many_single_char_names)]
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
    let alpha_ngp: F = alpha_ngp();
    let delta_ngp: F = delta_ngp();
    let u_sun: F = u_sun();
    let theta_sun: F = theta_sun();
    let u_sun_standard: F = u_sun_standard();
    let v_sun_standard: F = v_sun_standard();
    let w_sun_standard: F = w_sun_standard();
    let k: F = k();
    // Compute the heliocentric velocity
    let v_r = v_lsr
        - u_sun_standard * l.cos() * b.cos()
        - v_sun_standard * l.sin() * b.cos()
        - w_sun_standard * b.sin();
    // Compute the proper motions in Galactic coordinates
    let c_1 =
        delta_ngp.sin() * delta.cos() - delta_ngp.cos() * delta.sin() * F::cos(alpha - alpha_ngp);
    let c_2 = delta_ngp.cos() * F::sin(alpha - alpha_ngp);
    let mu_l = (c_1 * mu_x + c_2 * mu_y) / b.cos().powi(2);
    let mu_b = (c_1 * mu_y - c_2 * mu_x) / b.cos();
    // Compute the heliocentric distance
    let r_h = 1. / par;
    // Compute the velocity components
    let v_l = k * r_h * mu_l * b.cos();
    let v_b = k * r_h * mu_b;
    // Convert the velocities to the Cartesian
    // heliocentric coordinate system
    let u = (v_r * b.cos() - v_b * b.sin()) * l.cos() - v_l * l.sin();
    let v = (v_r * b.cos() - v_b * b.sin()) * l.sin() + v_l * l.cos();
    // Convert to the Galactocentric coordinate
    // system associated with the Sun
    let u_g = u + u_sun;
    let v_g = v + theta_sun;
    // Compute the Galactocentric distance
    let r_g = compute_r_g_2(l, b, r_h);
    // Compute the azimuthal velocity
    let sin_beta = r_h * b.cos() * l.sin() / r_g;
    let cos_beta = (r_0_2::<F>() - r_h * b.cos() * l.cos()) / r_g;
    let theta = v_g * cos_beta + u_g * sin_beta;
    (theta, r_g)
}
