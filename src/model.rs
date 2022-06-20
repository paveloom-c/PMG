//! Model of the Galaxy

mod coordinates;
mod fun;
mod io;

use coordinates::Galactic;

use std::fmt::Debug;

use num::Float;

/// Names of the objects
pub type Names = Vec<String>;

/// Types of the objects
pub type ObjTypes = Vec<String>;

/// Sources of the data
pub type Sources = Vec<String>;

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F: Float> {
    /// Names of the objects
    names: Names,
    /// Coordinates (in the Galactic heliocentric Cartesian system)
    coords: Galactic<F>,
    /// Types of the objects
    obj_types: ObjTypes,
    /// Sources of the data
    sources: Sources,
}
