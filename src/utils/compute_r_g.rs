//! Compute the distance in the Galactocentric
//! coordinate system associated with the object

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Galactocentric distance to the Sun (kpc)
const R_0: f64 = 8.;

/// Galactocentric distance to the Sun (kpc)
#[allow(clippy::unwrap_used)]
#[inline]
fn r_0<F: Float + Debug>() -> F {
    F::from(R_0).unwrap()
}

/// Compute the distance in the Galactocentric
/// coordinate system associated with the object
///
/// Source: Nikiforov (2014)
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_r_g<F: Float + Debug>(l: F, b: F, r: F) -> F {
    F::sqrt(
        r_0::<F>().powi(2) + r.powi(2) * b.cos().powi(2) - 2. * r_0::<F>() * r * l.cos() * b.cos(),
    )
}
