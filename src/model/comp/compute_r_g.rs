//! Compute the distance in the Galactocentric
//! coordinate system associated with the object

use crate::model::Consts;

use std::fmt::{Debug, Display};

use num::Float;
use numeric_literals::replace_float_literals;

impl<F: Float + Default + Display + Debug> Consts<F> {
    /// Compute the distance in the Galactocentric
    /// coordinate system associated with the object
    pub fn compute_r_g_1(&self, l: F, b: F, r_h: F) -> F {
        compute_r_g(l, b, r_h, self.r_0_1)
    }

    /// Compute the distance in the Galactocentric
    /// coordinate system associated with the object
    pub fn compute_r_g_2(&self, l: F, b: F, r_h: F) -> F {
        compute_r_g(l, b, r_h, self.r_0_2)
    }
}

/// Compute the distance in the Galactocentric
/// coordinate system associated with the object
///
/// Source: Nikiforov (2014)
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_r_g<F: Float + Debug>(l: F, b: F, r_h: F, r_0: F) -> F {
    // Compute the projection of the heliocentric distance in the XY plane
    let d = r_h * b.cos();
    // Compute the Galactocentric distance
    F::sqrt(r_0.powi(2) + d.powi(2) - 2. * r_0 * d * l.cos())
}
