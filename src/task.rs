//! Computation task

use core::cmp::Ord;

/// Computation goal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Goal {
    /// Perform per-object computations
    Objects,
    /// Fit the model of the Galaxy to the data
    Fit,
}

/// Computation task
#[derive(Debug)]
pub struct Task {
    /// Computation goal
    pub goal: Goal,
    /// Try to define the confidence intervals? (fit goal only)
    pub with_errors: bool,
}
