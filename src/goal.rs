//! Computation goal

use core::cmp::Ord;

/// Computation goal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Goal {
    /// Perform per-object computations
    Objects,
    /// Fit the model of the Galaxy to the data
    Fit,
}
