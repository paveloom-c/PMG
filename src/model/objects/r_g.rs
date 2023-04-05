//! Galactocentric distance

use super::{Object, Params};

use core::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F> {
    /// Compute the galactocentric distance with the specific values
    fn compute_r_g_with<F2>(&self, r_h: F, params: &Params<F2>) -> F
    where
        F: Float + Debug,
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
        F: Float + Debug + Default,
        F2: Float + Debug + Into<F>,
    {
        let r_h = self.r_h.unwrap();
        self.r_g = Some(self.compute_r_g_with(r_h, params));
    }
    /// Compute the galactocentric distance
    pub fn compute_r_g<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.unwrap();
        let r_h_p = self.r_h_p.unwrap();
        let r_h_m = self.r_h_m.unwrap();
        // Compute the Galactocentric distance
        let r_g = self.compute_r_g_with(r_h, params);
        let r_g_p = self.compute_r_g_with(r_h_p, params);
        let r_g_m = self.compute_r_g_with(r_h_m, params);
        self.r_g = Some(r_g);
        self.r_g_p = Some(r_g_p);
        self.r_g_m = Some(r_g_m);
        self.r_g_ep = Some(r_g_p - r_g);
        self.r_g_em = Some(r_g - r_g_m);
    }
}
