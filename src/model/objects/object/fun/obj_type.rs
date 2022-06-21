//! Unwrap the type of the object

use super::super::Object;

use anyhow::{anyhow, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Unwrap the type of the object
    pub(in crate::model) fn obj_type(&self) -> Result<&String> {
        self.obj_type
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the type of the object"))
    }
}
