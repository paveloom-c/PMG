//! Data object

mod coordinates;
mod fun;

use coordinates::{EquatorialSpherical, GalacticCartesian, GalacticSpherical};

use num::Float;

/// Data object
#[derive(Debug)]
pub(in crate::model) struct Object<F: Float> {
    /// Name of the object
    name: Option<String>,
    /// Equatorial spherical coordinates
    equatorial_s: Option<EquatorialSpherical<F>>,
    /// Galactic heliocentric spherical coordinates
    galactic_s: Option<GalacticSpherical<F>>,
    /// Galactic heliocentric Cartesian coordinates
    galactic_c: Option<GalacticCartesian<F>>,
    /// Type of the object
    obj_type: Option<String>,
    /// Source of the data
    source: Option<String>,
}
