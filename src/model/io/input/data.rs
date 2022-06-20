//! Input data

mod fun;

use crate::model::coordinates::Equatorial;
use crate::model::{Names, ObjTypes, Sources};

use std::fmt::Debug;

use num::Float;

/// Input data
#[derive(Debug)]
pub(in crate::model) struct Data<F: Float> {
    /// Names of the objects
    pub(in crate::model) names: Names,
    /// Coordinates (in the equatorial system)
    pub(in crate::model) coords: Equatorial<F>,
    /// Types of the objects
    pub(in crate::model) obj_types: ObjTypes,
    /// Sources of the data
    pub(in crate::model) sources: Sources,
}
