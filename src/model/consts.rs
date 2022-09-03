//! Numerical constants

use std::fmt::Debug;

use num::Float;

/// Constants
#[derive(Default, Debug)]
pub struct Consts<F: Float> {
    /// The right ascension of the north galactic pole (radians)
    pub alpha_ngp: F,
    /// The declination of the north galactic pole (radians)
    pub delta_ngp: F,
    /// Linear velocities units conversion coefficient
    pub k: F,
    /// The longitude of the north celestial pole (radians)
    pub l_ncp: F,
    /// Galactocentric distance to the Sun (kpc)
    pub r_0_1: F,
    /// Galactocentric distance to the Sun (kpc)
    pub r_0_2: F,
    /// Full circular velocity of the Sun (km/s)
    pub theta_sun: F,
    /// Peculiar motion locally toward GC (km/s)
    pub u_sun: F,
    /// Standard Solar Motion toward GC (km/s)
    pub u_sun_standard: F,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    pub v_sun_standard: F,
    /// Standard Solar Motion toward NGP (km/s)
    pub w_sun_standard: F,
}
