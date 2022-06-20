//! Data object

mod coordinates;
mod fun;

use coordinates::{EquatorialSpherical, GalacticCartesian};

use num::Float;

/// Data object
#[derive(Debug)]
pub(in crate::model) struct Object<F: Float> {
    /// Name of the object
    pub(in crate::model) name: String,
    /// Equatorial spherical coordinates
    #[allow(dead_code)]
    pub(in crate::model) equatorial_s: EquatorialSpherical<F>,
    /// Galactic heliocentric Cartesian coordinates
    pub(in crate::model) galactic_c: GalacticCartesian<F>,
    /// Type of the object
    pub(in crate::model) obj_type: String,
    /// Source of the data
    pub(in crate::model) source: String,
}
