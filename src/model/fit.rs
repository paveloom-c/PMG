//! Modules related to fitting

mod errors;
mod errors_logger;
mod fit_logger;
mod frozen_outer;
mod inner;
mod outer;
pub mod params;
pub mod rotcurve;

use super::io;
use super::{Model, Object, Objects, Params};

use errors::ConfidenceIntervalProblem;
use errors_logger::ErrorsLogger;
use fit_logger::FitLogger;
use frozen_outer::FrozenOuterOptimizationProblem;
use inner::InnerOptimizationProblem;
use outer::OuterOptimizationProblem;
