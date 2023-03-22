//! Distances

use super::{Measurement, Object};
use crate::model::Params;
use crate::utils;

use core::fmt::{Debug, Display};

use anyhow::Result;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Galactocentric distance (kpc)
#[derive(Clone, Debug)]
pub(in crate::model) struct GalactocentricDistance<F: Float + Debug> {
    /// Measurement
    pub(in crate::model) m: Measurement<F>,
}

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> GalactocentricDistance<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Try to convert the object into this struct
    pub(super) fn try_from(object: &Object<F>, params: &Params<F>) -> Result<Self> {
        // Unpack the data
        let (r_h, l, b) = object.galactic_s()?.into();
        // Compute the Galactocentric distance
        let r_g = utils::compute_r_g(l, b, r_h.v, params);
        let r_g_u = utils::compute_r_g(l, b, r_h.v_u, params);
        let r_g_l = utils::compute_r_g(l, b, r_h.v_l, params);
        Ok(Self {
            m: Measurement {
                v: r_g,
                v_u: r_g_u,
                v_l: r_g_l,
                e_p: r_g_u - r_g,
                e_m: r_g - r_g_l,
            },
        })
    }
}

impl<'a, F: Float + Debug> From<&'a GalactocentricDistance<F>> for &'a Measurement<F> {
    fn from(s: &'a GalactocentricDistance<F>) -> Self {
        &s.m
    }
}
