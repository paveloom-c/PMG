//! Convert between equatorial coordinates and
//! Galactic heliocentric coordinates

mod to_cartesian;
mod to_galactic;
mod to_spherical;

use to_cartesian::to_cartesian;
pub(super) use to_galactic::to_galactic;
use to_spherical::to_spherical;
