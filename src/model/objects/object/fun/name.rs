//! Unwrap the name of the object

use super::super::Object;

use anyhow::{anyhow, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Unwrap the name of the object
    pub(in crate::model) fn name(&self) -> Result<&String> {
        self.name
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the name"))
    }
}
