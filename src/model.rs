//! Model of the Galaxy

mod fun;
mod io;
mod objects;

use objects::{Object, Objects};

use std::fmt::Debug;

use num::Float;

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F: Float> {
    /// Data objects
    objects: Objects<F>,
}
