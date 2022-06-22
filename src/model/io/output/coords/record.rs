//! Output data record

mod from;

use num::Float;
use serde::Serialize;

/// Output data record
#[derive(Serialize)]
pub(in crate::model) struct Record<'a, F: Float> {
    /// Name
    pub(in crate::model) name: &'a String,
    /// Longitude
    pub(in crate::model) l: F,
    /// Latitude
    pub(in crate::model) b: F,
    /// X coordinate
    pub(in crate::model) x: F,
    /// Y coordinate
    pub(in crate::model) y: F,
    /// Z coordinate
    pub(in crate::model) z: F,
    /// Heliocentric distance
    pub(in crate::model) r_h: F,
    /// Galactocentric distance
    pub(in crate::model) r_g: F,
    /// Type of the object
    pub(in crate::model) obj_type: &'a String,
    /// Source of the data
    pub(in crate::model) source: &'a String,
}
