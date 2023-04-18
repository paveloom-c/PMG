//! Modules related to fitting

mod errors;
mod frozen_outer;
mod inner;
mod logger;
mod outer;
pub mod params;
pub mod rotcurve;

use super::io;
use super::{Model, Object, Objects, Params};

use errors::ConfidenceIntervalProblem;
use frozen_outer::FrozenOuterOptimizationProblem;
use inner::InnerOptimizationProblem;
use logger::Logger;
use outer::OuterOptimizationProblem;
