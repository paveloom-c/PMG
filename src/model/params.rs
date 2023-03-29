//! Model parameters

use core::fmt::{Debug, Display};

use num::Float;
use simulated_annealing::Point;

/// Model parameters
#[derive(Default, Debug, Clone)]
pub struct Params<F: Float + Debug> {
    /// The right ascension of the north galactic pole (radians)
    pub alpha_ngp: F,
    /// The declination of the north galactic pole (radians)
    pub delta_ngp: F,
    /// Linear velocities units conversion coefficient
    pub k: F,
    /// The longitude of the north celestial pole (radians)
    pub l_ncp: F,
    /// Galactocentric distance to the Sun (kpc)
    pub r_0: F,
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
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    pub omega_0: F,
    /// Oort's A constant (km/s/kpc)
    pub a: F,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_r: F,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: F,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_z: F,
}

impl<F> Params<F>
where
    F: Float + Default + Display + Debug,
{
    /// Update the parameters with the point in the parameter space
    ///
    /// Note that not all fields are updated, but only those needed for fitting
    pub fn update_with(&mut self, p: &Point<F, 9>) {
        self.r_0 = p[0];
        self.omega_0 = p[1];
        self.a = p[2];
        self.u_sun_standard = p[3];
        self.v_sun_standard = p[4];
        self.w_sun_standard = p[5];
        self.sigma_r = p[6];
        self.sigma_theta = p[7];
        self.sigma_z = p[8];
    }
    /// Construct a point in the parameter space from the parameters
    ///
    /// Note that not all fields are used, but only those needed for fitting
    pub fn to_point(&self) -> Point<F, 9> {
        [
            self.r_0,
            self.omega_0,
            self.a,
            self.u_sun_standard,
            self.v_sun_standard,
            self.w_sun_standard,
            self.sigma_r,
            self.sigma_theta,
            self.sigma_z,
        ]
    }
}
