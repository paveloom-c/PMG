//! Output data record

use num::Float;
use serde::Serialize;

/// Output data record
#[derive(Serialize)]
pub(in crate::model) struct Record<F: Float> {
    /// Name
    pub(in crate::model) name: String,
    /// X coordinate
    pub(in crate::model) x: F,
    /// Y coordinate
    pub(in crate::model) y: F,
    /// Z coordinate
    pub(in crate::model) z: F,
}
