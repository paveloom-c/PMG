//! Bounds of the initial parameters

use core::fmt::Debug;
use core::ops::Range;

use num::Float;

/// Bounds of the initial parameters
#[derive(Debug, Default)]
pub struct Bounds<F> {
    /// Bounds of the Galactocentric distance to the Sun (kpc)
    pub r_0: Range<F>,
    /// Bounds of the Circular velocity of the Sun at R = R_0 (km/s/kpc)
    pub omega_0: Range<F>,
    /// Bounds of the Oort's A constant (km/s/kpc)
    pub a: Range<F>,
    /// Bounds of the Standard Solar Motion toward GC (km/s)
    pub u_sun_standard: Range<F>,
    /// Bounds of the Standard Solar Motion toward l = 90 degrees (km/s)
    pub v_sun_standard: Range<F>,
    /// Bounds of the Standard Solar Motion toward NGP (km/s)
    pub w_sun_standard: Range<F>,
    /// Bounds of the radial component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_r: Range<F>,
    /// Bounds of the azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: Range<F>,
    /// Bounds of the vertical component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_z: Range<F>,
}

impl<F> Bounds<F>
where
    F: Float + Debug,
{
    /// Convert the bounds struct to an array
    pub(super) fn to_array(&self) -> [Range<F>; 9] {
        [
            self.r_0.clone(),
            self.omega_0.clone(),
            self.a.clone(),
            self.u_sun_standard.clone(),
            self.v_sun_standard.clone(),
            self.w_sun_standard.clone(),
            self.sigma_r.clone(),
            self.sigma_theta.clone(),
            self.sigma_z.clone(),
        ]
    }
}
