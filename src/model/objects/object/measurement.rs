//! Measurement data

use std::fmt::Debug;

use num::Float;

/// Measurement data
#[derive(Debug)]
pub(in crate::model) struct Measurement<F: Float + Debug> {
    /// Nominal value
    pub(in crate::model) v: F,
    /// Upper bound of the value (if independent) or a
    /// value computed from an upper bound of another
    /// value (if dependent)
    ///
    /// Note that in the latter case the value
    /// can be less than the nominal value.
    pub(in crate::model) v_u: F,
    /// Lower bound of the value (if independent) or a
    /// value computed from a lower bound of another
    /// value (if dependent)
    ///
    /// Note that in the latter case the value
    /// can be greater than the nominal value.
    pub(in crate::model) v_l: F,
    /// Plus uncertainty:
    /// the difference between `v_u` and `v`
    ///
    /// Note that the value can be negative.
    pub(in crate::model) e_p: F,
    /// Minus uncertainty:
    /// the difference between `v` and `v_l`
    ///
    /// Note that the value can be negative.
    pub(in crate::model) e_m: F,
}
