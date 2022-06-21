//! Unwrap the source of the data

use super::super::Object;

use anyhow::{anyhow, Result};
use num::Float;

impl<F: Float> Object<F> {
    /// Unwrap the source of the data
    pub(in crate::model) fn source(&self) -> Result<&String> {
        self.source
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the source of the data"))
    }
}
