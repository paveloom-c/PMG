//! Velocities in Galactic heliocentric spherical coordinates

use super::{Measurement, Object};
use crate::model::Params;

use core::fmt::{Debug, Display};

use num::{traits::FloatConst, Float};

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
impl<F> Object<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Compute the heliocentric velocity with the specific values
    fn compute_v_r_with<F2>(&self, v_lsr: F, params: &Params<F2>) -> F
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        // Get the parameters
        let u_sun_standard: F = params.u_sun_standard.into();
        let v_sun_standard: F = params.v_sun_standard.into();
        let w_sun_standard: F = params.w_sun_standard.into();
        // Compute the heliocentric velocity
        v_lsr
            - (u_sun_standard * l.cos() + v_sun_standard * l.sin()) * b.cos()
            - w_sun_standard * b.sin()
    }
    /// Compute the heliocentric velocity in distance
    pub fn compute_v_r<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let v_lsr = self.v_lsr.as_ref().unwrap();
        // Compute the heliocentric velocity
        let v_r = self.compute_v_r_with(v_lsr.v, params);
        let v_r_u = self.compute_v_r_with(v_lsr.v_u, params);
        let v_r_l = self.compute_v_r_with(v_lsr.v_l, params);
        self.v_r = Some(Measurement {
            v: v_r,
            v_u: v_r_u,
            v_l: v_r_l,
            e_p: v_r_u - v_r,
            e_m: v_r - v_r_l,
        });
    }
    /// Compute the velocities in longitude and
    /// latitude with the specific values
    fn compute_v_l_v_b_with<F2>(&self, r_h: F, mu_l: F, mu_b: F, params: &Params<F2>) -> (F, F)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let b = self.b.unwrap();
        // Get the parameters
        let k: F = params.k.into();
        // Compute the heliocentric velocity
        let v_l = k * r_h * mu_l * b.cos();
        let v_b = k * r_h * mu_b;
        (v_l, v_b)
    }
    /// Compute the velocities in longitude and latitude
    pub fn compute_v_l_v_b<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.as_ref().unwrap();
        let mu_l = self.mu_l.as_ref().unwrap();
        let mu_b = self.mu_b.as_ref().unwrap();
        // Compute the heliocentric velocity
        let (v_l, v_b) = self.compute_v_l_v_b_with(r_h.v, mu_l.v, mu_b.v, params);
        let (v_l_u, v_b_u) = self.compute_v_l_v_b_with(r_h.v_u, mu_l.v_u, mu_b.v_u, params);
        let (v_l_l, v_b_l) = self.compute_v_l_v_b_with(r_h.v_l, mu_l.v_l, mu_b.v_l, params);
        self.v_l = Some(Measurement {
            v: v_l,
            v_u: v_l_u,
            v_l: v_l_l,
            e_p: v_l_u - v_l,
            e_m: v_l - v_l_l,
        });
        self.v_b = Some(Measurement {
            v: v_b,
            v_u: v_b_u,
            v_l: v_b_l,
            e_p: v_b_u - v_b,
            e_m: v_b - v_b_l,
        });
    }
}
