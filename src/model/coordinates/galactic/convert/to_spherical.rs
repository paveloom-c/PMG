//! Convert equatorial coordinates to spherical Galactic coordinates

use crate::utils::{dms2rad, hms2rad};

use lazy_static::lazy_static;
use num::Float;

lazy_static! {
    /// The right ascension of the north galactic pole
    static ref ALPHA_NGP: f64 = hms2rad(12., 51., 26.2817);
    /// The declination of the north galactic pole
    static ref DELTA_NGP: f64 = dms2rad(27., 7., 42.013);
    /// The longitude of the north celestial pole
    static ref L_NCP: f64 = 122.932;
}

/// The right ascension of the north galactic pole
#[allow(clippy::unwrap_used)]
#[inline]
fn alpha_ngp<F: Float>() -> F {
    F::from(*ALPHA_NGP).unwrap()
}

/// The declination of the north galactic pole
#[allow(clippy::unwrap_used)]
#[inline]
fn delta_ngp<F: Float>() -> F {
    F::from(*DELTA_NGP).unwrap()
}

/// The longitude of the north celestial pole
#[allow(clippy::unwrap_used)]
#[inline]
fn l_ncp<F: Float>() -> F {
    F::from(*L_NCP).unwrap()
}

/// Convert the equatorial coordinates to spherical heliocentric Galactic coordinates
pub(super) fn to_spherical<F: Float>(alpha: F, delta: F) -> (F, F) {
    let phi = F::atan2(
        F::cos(delta) * F::sin(alpha - alpha_ngp()),
        F::cos(delta_ngp()) * F::sin(delta)
            - F::sin(delta_ngp()) * F::cos(delta) * F::cos(alpha - alpha_ngp()),
    );
    let l = l_ncp::<F>() - phi;
    let b = F::asin(
        F::sin(delta_ngp()) * F::sin(delta)
            + F::cos(delta_ngp()) * F::cos(delta) * F::cos(alpha - alpha_ngp()),
    );
    (l, b)
}
