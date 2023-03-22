//! Galactic heliocentric spherical coordinates

use super::{Measurement, Object};
use crate::model::Params;
use crate::utils;

use core::fmt::{Debug, Display};

use anyhow::Result;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Galactic heliocentric spherical coordinates
#[derive(Clone, Debug)]
pub struct GalacticSpherical<F: Float + Debug> {
    /// Heliocentric distance (kpc)
    pub r_h: Measurement<F>,
    /// Longitude (radians)
    pub l: F,
    /// Latitude (radians)
    pub b: F,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> GalacticSpherical<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Try to convert the object into this struct
    pub(super) fn try_from(object: &Object<F>, params: &Params<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        let par = object.par()?;
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = utils::to_spherical(alpha, delta, params);
        // Compute the heliocentric distance
        let r_h = 1. / par.v;
        let r_h_u = 1. / par.v_u;
        let r_h_l = 1. / par.v_l;
        Ok(Self {
            r_h: Measurement {
                v: r_h,
                v_u: r_h_u,
                v_l: r_h_l,
                e_p: r_h_u - r_h,
                e_m: r_h - r_h_l,
            },
            l,
            b,
        })
    }
}

impl<'a, F: Float + Debug> From<&'a GalacticSpherical<F>> for (&'a Measurement<F>, F, F) {
    fn from(s: &'a GalacticSpherical<F>) -> Self {
        (&s.r_h, s.l, s.b)
    }
}
