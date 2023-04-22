//! Modules related to fitting

mod errors;
mod errors_logger;
mod fit_logger;
mod frozen_outer;
mod inner;
mod outer;
pub mod params;
mod profiles;
pub mod rotcurve;

use super::io;
use super::{Model, Object, Objects, Params, PARAMS_N, PARAMS_NAMES};

use errors_logger::ErrorsLogger;
use fit_logger::FitLogger;
use frozen_outer::FrozenOuterOptimizationProblem;
use inner::InnerOptimizationProblem;
use outer::OuterOptimizationProblem;
pub use profiles::Profiles;
pub use rotcurve::RotationCurve;
