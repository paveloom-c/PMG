//! Parallax data

use std::fmt::Debug;

use num::Float;

/// Parallax data
#[derive(Debug)]
pub(in crate::model) struct Measurement<F: Float> {
    /// Value
    pub(in crate::model) v: F,
    /// Uncertainty
    pub(in crate::model) e: F,
}
