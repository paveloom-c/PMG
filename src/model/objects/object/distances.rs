//! Distances

use super::{Measurement, Object};
use crate::model::Params;
use crate::utils::compute_r_g;

use std::fmt::{Debug, Display};

use anyhow::Result;
use num::Float;
use numeric_literals::replace_float_literals;

/// Distances
#[derive(Clone, Debug)]
pub(in crate::model) struct Distances<F: Float + Debug> {
    /// Heliocentric distance (kpc)
    pub(in crate::model) r_h: Measurement<F>,
    /// Galactocentric distance (kpc)
    ///
    /// Because of the different parameters being used,
    /// this is not the same distance as in the
    /// [`RotationCurve`](super::RotationCurve) struct
    pub(in crate::model) r_g: Measurement<F>,
}

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Distances<F>
where
    F: Float + Default + Display + Debug,
{
    /// Try to convert the object into this struct
    pub(super) fn try_from(object: &Object<F>, params: &Params<F>) -> Result<Self> {
        // Unpack the data
        let (l, b) = object.galactic_s()?.into();
        let par = object.par()?;
        // Compute the heliocentric distance
        let r_h = 1. / par.v;
        let r_h_u = 1. / par.v_u;
        let r_h_l = 1. / par.v_l;
        // Compute the Galactocentric distance
        let r_g = compute_r_g(l, b, r_h, params);
        let r_g_u = compute_r_g(l, b, r_h_u, params);
        let r_g_l = compute_r_g(l, b, r_h_l, params);
        Ok(Self {
            r_h: Measurement {
                v: r_h,
                v_u: r_h_u,
                v_l: r_h_l,
                e_p: r_h_u - r_h,
                e_m: r_h - r_h_l,
            },
            r_g: Measurement {
                v: r_g,
                v_u: r_g_u,
                v_l: r_g_l,
                e_p: r_g_u - r_g,
                e_m: r_g - r_g_l,
            },
        })
    }
}

impl<'a, F: Float + Debug> From<&'a Distances<F>> for (&'a Measurement<F>, &'a Measurement<F>) {
    fn from(s: &'a Distances<F>) -> Self {
        (&s.r_h, &s.r_g)
    }
}
