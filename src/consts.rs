//! Numerical constants

use crate::utils::{dms2rad, hms2rad};
use std::fmt::Debug;

use lazy_static::lazy_static;
use num::Float;

/// Galactocentric distance to the Sun (kpc)
pub const R_0_1: f64 = 8.;

/// Galactocentric distance to the Sun (kpc)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
pub const R_0_2: f64 = 8.15;

/// Standard Solar Motion toward GC (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
pub const U_SUN_STANDARD: f64 = 10.3;

/// Peculiar motion locally toward GC (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
pub const U_SUN: f64 = 10.7;

/// Standard Solar Motion toward l = 90 degrees (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
pub const V_SUN_STANDARD: f64 = 15.3;

/// Full circular velocity of the Sun (km/s)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
pub const THETA_SUN: f64 = 247.;

/// Standard Solar Motion toward NGP (km/s)
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
pub const W_SUN_STANDARD: f64 = 7.7;

/// Linear velocities units conversion coefficient
///
/// Sources: Gromov, Nikiforov (2016)
pub const K: f64 = 4.7406;

lazy_static! {
    /// The right ascension of the north galactic pole (radians)
    ///
    /// Source: Reid et al. (2009)
    pub static ref ALPHA_NGP: f64 = hms2rad(12., 51., 26.2817);
    /// The declination of the north galactic pole (radians)
    ///
    /// Source: Reid et al. (2009)
    pub static ref DELTA_NGP: f64 = dms2rad(27., 7., 42.013);
    /// The longitude of the north celestial pole (radians)
    ///
    /// Source: Reid et al. (2009)
    pub static ref L_NCP: f64 = 122.932.to_radians();
}

/// Galactocentric distance to the Sun (kpc)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn r_0_1<F: Float + Debug>() -> F {
    F::from(R_0_1).unwrap()
}

/// Galactocentric distance to the Sun (kpc)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn r_0_2<F: Float + Debug>() -> F {
    F::from(R_0_2).unwrap()
}

/// Standard Solar Motion toward GC (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn u_sun_standard<F: Float + Debug>() -> F {
    F::from(U_SUN_STANDARD).unwrap()
}

/// Peculiar motion locally toward GC (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn u_sun<F: Float + Debug>() -> F {
    F::from(U_SUN).unwrap()
}

/// Standard Solar Motion toward l = 90 degrees (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn v_sun_standard<F: Float + Debug>() -> F {
    F::from(V_SUN_STANDARD).unwrap()
}

/// Full circular velocity of the Sun (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn theta_sun<F: Float + Debug>() -> F {
    F::from(THETA_SUN).unwrap()
}

/// Standard Solar Motion toward NGP (km/s)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn w_sun_standard<F: Float + Debug>() -> F {
    F::from(W_SUN_STANDARD).unwrap()
}

/// Linear velocities units conversion coefficient
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub fn k<F: Float + Debug>() -> F {
    F::from(K).unwrap()
}

/// The right ascension of the north galactic pole (radians)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn alpha_ngp<F: Float + Debug>() -> F {
    F::from(*ALPHA_NGP).unwrap()
}

/// The declination of the north galactic pole (radians)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn delta_ngp<F: Float + Debug>() -> F {
    F::from(*DELTA_NGP).unwrap()
}

/// The longitude of the north celestial pole (radians)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn l_ncp<F: Float + Debug>() -> F {
    F::from(*L_NCP).unwrap()
}
