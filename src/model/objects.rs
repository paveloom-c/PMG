//! Data objects

mod fun;
mod object;

pub(in crate::model) use object::Object;

use std::fmt::Debug;

use num::Float;

/// Data objects
#[derive(Debug)]
pub struct Objects<F: Float>(Vec<Object<F>>);
