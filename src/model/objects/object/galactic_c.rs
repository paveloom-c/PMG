//! Galactic heliocentric Cartesian coordinates

use super::{Measurement, Object};
use crate::utils::to_cartesian;

use std::fmt::Debug;

use anyhow::Result;
use num::Float;

/// Galactic heliocentric Cartesian coordinates
#[derive(Debug)]
pub(in crate::model) struct GalacticCartesian<F: Float + Debug> {
    /// X coordinate (kpc)
    pub(in crate::model) x: Measurement<F>,
    /// Y coordinate (kpc)
    pub(in crate::model) y: Measurement<F>,
    /// Z coordinate (kpc)
    pub(in crate::model) z: Measurement<F>,
}

#[allow(clippy::many_single_char_names)]
impl<F: Float + Debug> TryFrom<&Object<F>> for GalacticCartesian<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (l, b) = object.galactic_s()?.into();
        let r_h = &object.distances()?.r_h;
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = to_cartesian(l, b, r_h.v);
        let (x_u, y_u, z_u) = to_cartesian(l, b, r_h.v_u);
        let (x_l, y_l, z_l) = to_cartesian(l, b, r_h.v_l);
        Ok(Self {
            x: Measurement {
                v: x,
                v_u: x_u,
                v_l: x_l,
                e_p: (x_u - x).abs(),
                e_m: (x_l - x).abs(),
            },
            y: Measurement {
                v: y,
                v_u: y_u,
                v_l: y_l,
                e_p: (y_u - y).abs(),
                e_m: (y_l - y).abs(),
            },
            z: Measurement {
                v: z,
                v_u: z_u,
                v_l: z_l,
                e_p: (z_u - z).abs(),
                e_m: (z_l - z).abs(),
            },
        })
    }
}

impl<'a, F: Float + Debug> From<&'a GalacticCartesian<F>>
    for (&'a Measurement<F>, &'a Measurement<F>, &'a Measurement<F>)
{
    fn from(s: &'a GalacticCartesian<F>) -> Self {
        (&s.x, &s.y, &s.z)
    }
}
