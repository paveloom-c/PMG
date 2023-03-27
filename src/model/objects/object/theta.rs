//! Azimuthal velocity

use super::{Measurement, Object};
use crate::model::Params;

use core::fmt::{Debug, Display};

use autodiff::FT;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Compute the azimuthal velocity with the specific values
    #[allow(clippy::many_single_char_names)]
    fn compute_theta_with<F2>(&self, r_h: F, r_g: F, u: F, v: F, params: &Params<F2>) -> F
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        // Get the parameters
        let u_sun: F = params.u_sun.into();
        let theta_sun: F = params.theta_sun.into();
        let r_0: F = params.r_0.into();
        // Convert to the Galactocentric coordinate
        // system associated with the Sun
        let u_g = u + u_sun;
        let v_g = v + theta_sun;
        // Compute the projection of the heliocentric distance in the XY plane
        let d = r_h * b.cos();
        // Compute the azimuthal velocity
        let sin_lambda = d / r_g * l.sin();
        let cos_lambda = (r_0 - d * l.cos()) / r_g;
        v_g * cos_lambda + u_g * sin_lambda
    }
    /// Compute the azimuthal velocity (nominal value only)
    pub fn compute_theta_nominal<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        let r_h = self.r_h.as_ref().unwrap();
        let r_g = self.r_g.as_ref().unwrap();
        let u = self.u.as_ref().unwrap();
        let v = self.v.as_ref().unwrap();
        self.theta = Some(Measurement {
            v: self.compute_theta_with(r_h.v, r_g.v, u.v, v.v, params),
            ..Default::default()
        });
    }
    /// Compute the azimuthal velocity
    pub fn compute_theta<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.as_ref().unwrap();
        let r_g = self.r_g.as_ref().unwrap();
        let u = self.u.as_ref().unwrap();
        let v = self.v.as_ref().unwrap();
        // Compute the azimuthal velocity
        let theta = self.compute_theta_with(r_h.v, r_g.v, u.v, v.v, params);
        let theta_u = self.compute_theta_with(r_h.v_u, r_g.v_u, u.v_u, v.v_u, params);
        let theta_l = self.compute_theta_with(r_h.v_l, r_g.v_l, u.v_l, v.v_l, params);
        self.theta = Some(Measurement {
            v: theta,
            v_u: theta_u,
            v_l: theta_l,
            e_p: theta_u - theta,
            e_m: theta - theta_l,
        });
    }
    /// Compute the uncertainty in the azimuthal velocity inherited from velocities
    ///
    /// Note that we compute all values again, starting from the independent errors
    ///
    /// Sources: Gromov, Nikiforov, Ossipkov (2016)
    pub(super) fn compute_e_vel_theta(&mut self, params: &Params<F>) {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let v_lsr = self.v_lsr.as_ref().unwrap();
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        let par = self.par.as_ref().unwrap();
        // Compute the partial derivative of the azimuthal
        // velocity by the Local Standard of Rest velocity
        let mut object = Object {
            alpha: Some(FT::cst(alpha)),
            delta: Some(FT::cst(delta)),
            l: Some(FT::cst(l)),
            b: Some(FT::cst(b)),
            par: Some(Measurement {
                v: FT::cst(par.v),
                ..Default::default()
            }),
            v_lsr: Some(Measurement {
                v: FT::var(v_lsr.v),
                ..Default::default()
            }),
            mu_x: Some(Measurement {
                v: FT::cst(mu_x.v),
                ..Default::default()
            }),
            mu_y: Some(Measurement {
                v: FT::cst(mu_y.v),
                ..Default::default()
            }),
            ..Default::default()
        };
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_mu_b_nominal(params);
        object.compute_v_r_nominal(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let d_theta_v_lsr = object.theta.as_ref().unwrap().v.deriv();
        // Compute the partial derivative of the azimuthal
        // velocity by the Eastward proper motion
        object.v_lsr = Some(Measurement {
            v: FT::cst(v_lsr.v),
            ..Default::default()
        });
        object.mu_x = Some(Measurement {
            v: FT::var(mu_x.v),
            ..Default::default()
        });
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_mu_b_nominal(params);
        object.compute_v_r_nominal(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let d_theta_mu_x = object.theta.as_ref().unwrap().v.deriv();
        // Compute the partial derivative of the azimuthal
        // velocity by the Northward proper motion
        object.mu_x = Some(Measurement {
            v: FT::cst(mu_x.v),
            ..Default::default()
        });
        object.mu_y = Some(Measurement {
            v: FT::var(mu_y.v),
            ..Default::default()
        });
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_mu_b_nominal(params);
        object.compute_v_r_nominal(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let d_theta_mu_y = object.theta.as_ref().unwrap().v.deriv();
        // Compute the uncertainty
        self.e_vel_theta = Some(F::sqrt(
            d_theta_v_lsr.powi(2) * v_lsr.e_p.powi(2)
                + d_theta_mu_x.powi(2) * mu_x.e_p.powi(2)
                + d_theta_mu_y.powi(2) * mu_y.e_p.powi(2),
        ));
    }
}
