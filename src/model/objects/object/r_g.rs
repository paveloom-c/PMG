//! Galactocentric distance

use super::{Measurement, Object};
use crate::model::Params;

use core::fmt::{Debug, Display};

use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Compute the galactocentric distance with the specific values
    fn compute_r_g_with<F2>(&self, r_h: F, params: &Params<F2>) -> F
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        // Prepare the Galactocentric distance to the Sun
        let r_0: F = params.r_0.into();
        // Compute the projection of the heliocentric distance in the XY plane
        let d = r_h * b.cos();
        // Compute the Galactocentric distance
        F::sqrt(r_0.powi(2) + d.powi(2) - 2. * r_0 * d * l.cos())
    }
    /// Compute the galactocentric distance (nominal value only)
    pub fn compute_r_g_nominal<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        let r_h = self.r_h.as_ref().unwrap();
        self.r_g = Some(Measurement {
            v: self.compute_r_g_with(r_h.v, params),
            ..Default::default()
        });
    }
    /// Compute the galactocentric distance
    pub fn compute_r_g<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.as_ref().unwrap();
        // Compute the Galactocentric distance
        let r_g = self.compute_r_g_with(r_h.v, params);
        let r_g_u = self.compute_r_g_with(r_h.v_u, params);
        let r_g_l = self.compute_r_g_with(r_h.v_l, params);
        self.r_g = Some(Measurement {
            v: r_g,
            v_u: r_g_u,
            v_l: r_g_l,
            e_p: r_g_u - r_g,
            e_m: r_g - r_g_l,
        });
    }
}
