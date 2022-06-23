//! Compute the distance in the Galactocentric
//! coordinate system associated with the object

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Galactocentric distance to the Sun (kpc)
const R_0_1: f64 = 8.;

/// Galactocentric distance to the Sun (kpc)
///
/// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
const R_0_2: f64 = 8.15;

/// Galactocentric distance to the Sun (kpc)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn r_0_1<F: Float + Debug>() -> F {
    F::from(R_0_1).unwrap()
}

/// Galactocentric distance to the Sun (kpc)
#[allow(clippy::inline_always)]
#[allow(clippy::unwrap_used)]
#[inline(always)]
pub(super) fn r_0_2<F: Float + Debug>() -> F {
    F::from(R_0_2).unwrap()
}

/// Compute the distance in the Galactocentric
/// coordinate system associated with the object
///
/// Source: Nikiforov (2014)
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_r_g_1<F: Float + Debug>(l: F, b: F, r: F) -> F {
    let r_0: F = r_0_1();
    F::sqrt(r_0.powi(2) + r.powi(2) * b.cos().powi(2) - 2. * r_0 * r * l.cos() * b.cos())
}

/// Compute the distance in the Galactocentric
/// coordinate system associated with the object
///
/// Source: Nikiforov (2014)
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_r_g_2<F: Float + Debug>(l: F, b: F, r: F) -> F {
    let r_0: F = r_0_2();
    F::sqrt(r_0.powi(2) + r.powi(2) * b.cos().powi(2) - 2. * r_0 * r * l.cos() * b.cos())
}
