//! Modules related to fitting

mod inner;
mod logger;
mod outer;
pub mod params;
pub mod rotcurve;

use super::io;
use super::{Model, Object, Objects, Params};

use inner::InnerOptimizationProblem;
use logger::Logger;
use outer::OuterOptimizationProblem;
