//! Compute the distance in the Galactocentric
//! coordinate system associated with the object

#![allow(clippy::module_name_repetitions)]

use crate::model::Params;

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Compute the distance in the Galactocentric
/// coordinate system associated with the object
///
/// Source: Nikiforov (2014)
#[allow(clippy::unwrap_used)]
#[replace_float_literals(<F as num::NumCast>::from(literal).unwrap())]
pub fn compute_r_g<F, F2>(l: F, b: F, r_h: F, params: &Params<F2>) -> F
where
    F: Float + Debug + From<F2>,
    F2: Float + Debug,
{
    // Prepare the Galactocentric distance to the Sun
    let r_0: F = params.r_0.into();
    // Compute the projection of the heliocentric distance in the XY plane
    let d = r_h * b.cos();
    // Compute the Galactocentric distance
    F::sqrt(r_0.powi(2) + d.powi(2) - 2. * r_0 * d * l.cos())
}
