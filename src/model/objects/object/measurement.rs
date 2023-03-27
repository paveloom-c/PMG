//! Measurement data

use core::fmt::Debug;

use num::Float;

/// Measurement data
///
/// Uncertainties here are inherited from the parallax only!
/// Uncertainties inherited from velocities (`v_lsr`, `mu_x`,
/// and `mu_y`) are computed separately.
#[derive(Clone, Debug, Default)]
pub struct Measurement<F: Float + Debug> {
    /// Nominal value
    pub v: F,
    /// Upper bound of the value (if independent) or a
    /// value computed from an upper bound of another
    /// value (if dependent)
    ///
    /// Note that in the latter case the value
    /// can be less than the nominal value.
    pub v_u: F,
    /// Lower bound of the value (if independent) or a
    /// value computed from a lower bound of another
    /// value (if dependent)
    ///
    /// Note that in the latter case the value
    /// can be greater than the nominal value.
    pub v_l: F,
    /// Plus uncertainty:
    /// the difference between `v_u` and `v`
    ///
    /// Note that the value can be negative.
    pub e_p: F,
    /// Minus uncertainty:
    /// the difference between `v` and `v_l`
    ///
    /// Note that the value can be negative.
    pub e_m: F,
}
