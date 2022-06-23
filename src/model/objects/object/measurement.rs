//! Measurement data

use std::fmt::Debug;

use num::Float;

/// Measurement data
#[derive(Debug)]
pub(in crate::model) struct Measurement<F: Float + Debug> {
    /// Nominal value
    pub(in crate::model) v: F,
    /// Upper bound of the value
    pub(in crate::model) v_u: F,
    /// Lower bound of the value
    pub(in crate::model) v_l: F,
    /// Uncertainty plus
    pub(in crate::model) e_p: F,
    /// Uncertainty minus
    pub(in crate::model) e_m: F,
}
