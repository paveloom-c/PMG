//! Numerical constants

use std::fmt::Debug;

use num::Float;

/// Constants
#[derive(Default, Debug)]
pub struct Consts {
    /// The right ascension of the north galactic pole (radians)
    pub alpha_ngp: f64,
    /// The declination of the north galactic pole (radians)
    pub delta_ngp: f64,
    /// Linear velocities units conversion coefficient
    pub k: f64,
    /// The longitude of the north celestial pole (radians)
    pub l_ncp: f64,
    /// Galactocentric distance to the Sun (kpc) [coords]
    pub r_0_1: f64,
    /// Galactocentric distance to the Sun (kpc) [rotcurve]
    pub r_0_2: f64,
    /// Full circular velocity of the Sun (km/s)
    pub theta_sun: f64,
    /// Peculiar motion locally toward GC (km/s)
    pub u_sun: f64,
    /// Standard Solar Motion toward GC (km/s)
    pub u_sun_standard: f64,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    pub v_sun_standard: f64,
    /// Standard Solar Motion toward NGP (km/s)
    pub w_sun_standard: f64,
}

#[allow(clippy::unwrap_used)]
impl Consts {
    /// The right ascension of the north galactic pole (radians)
    pub fn alpha_ngp<F: Float>(&self) -> F {
        F::from(self.alpha_ngp).unwrap()
    }
    /// The declination of the north galactic pole (radians)
    pub fn delta_ngp<F: Float>(&self) -> F {
        F::from(self.delta_ngp).unwrap()
    }
    /// Linear velocities units conversion coefficient
    pub fn k<F: Float>(&self) -> F {
        F::from(self.k).unwrap()
    }
    /// The longitude of the north celestial pole (radians)
    pub fn l_ncp<F: Float>(&self) -> F {
        F::from(self.l_ncp).unwrap()
    }
    /// Galactocentric distance to the Sun (kpc) [coords]
    pub fn r_0_1<F: Float>(&self) -> F {
        F::from(self.r_0_1).unwrap()
    }
    /// Galactocentric distance to the Sun (kpc) [rotcurve]
    pub fn r_0_2<F: Float>(&self) -> F {
        F::from(self.r_0_2).unwrap()
    }
    /// Full circular velocity of the Sun (km/s)
    pub fn theta_sun<F: Float>(&self) -> F {
        F::from(self.theta_sun).unwrap()
    }
    /// Peculiar motion locally toward GC (km/s)
    pub fn u_sun<F: Float>(&self) -> F {
        F::from(self.u_sun).unwrap()
    }
    /// Standard Solar Motion toward GC (km/s)
    pub fn u_sun_standard<F: Float>(&self) -> F {
        F::from(self.u_sun_standard).unwrap()
    }
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    pub fn v_sun_standard<F: Float>(&self) -> F {
        F::from(self.v_sun_standard).unwrap()
    }
    /// Standard Solar Motion toward NGP (km/s)
    pub fn w_sun_standard<F: Float>(&self) -> F {
        F::from(self.w_sun_standard).unwrap()
    }
}
