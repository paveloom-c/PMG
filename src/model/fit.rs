//! Modules related to fitting

#![allow(clippy::module_name_repetitions)]

mod covariance;
mod errors;
mod errors_logger;
mod fit_logger;
mod frozen_outer;
mod inner;
mod outer;
mod outliers;
mod parallaxes;
pub mod params;
mod profiles;
pub mod rotcurve;
mod sigma_outer;
mod steepest_descent;

use super::io;
use super::{Model, Object, Objects, Params, PARAMS_N, PARAMS_NAMES};

pub use errors::ConfidenceIntervalProblem;
pub use errors_logger::ErrorsLogger;
pub use fit_logger::FitLogger;
pub use frozen_outer::FrozenOuterOptimizationProblem;
pub use inner::{
    compute_relative_discrepancy, prepare_inner_problem, InnerOptimizationProblem, Triple, Triples,
};
pub use outer::OuterOptimizationProblem;
pub use profiles::ProfileType;
pub use rotcurve::RotationCurve;
pub use sigma_outer::SigmaOuterOptimizationProblem;
