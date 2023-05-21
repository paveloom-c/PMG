//! Azimuthal velocity

use crate::model::fit::params::VEL_TERM;

use super::{Object, Params};

use core::fmt::Debug;

use autodiff::FT;
use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F> {
    /// Compute the azimuthal velocity with the specific values
    #[allow(clippy::many_single_char_names)]
    fn compute_theta_with<F2>(&self, r_h: F, r_g: F, u: F, v: F, params: &Params<F2>) -> F
    where
        F: Float + Debug,
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
        u_g * sin_lambda + v_g * cos_lambda
    }
    /// Compute the azimuthal velocity (nominal value only)
    pub fn compute_theta_nominal<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug + Default,
        F2: Float + Debug + Into<F>,
    {
        let r_h = self.r_h.unwrap();
        let r_g = self.r_g.unwrap();
        let u = self.u.unwrap();
        let v = self.v.unwrap();
        self.theta = Some(self.compute_theta_with(r_h, r_g, u, v, params));
    }
    /// Compute the azimuthal velocity
    #[allow(clippy::similar_names)]
    pub fn compute_theta<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.unwrap();
        let r_h_p = self.r_h_p.unwrap();
        let r_h_m = self.r_h_m.unwrap();
        let r_g = self.r_g.unwrap();
        let r_g_p = self.r_g_p.unwrap();
        let r_g_m = self.r_g_m.unwrap();
        let u = self.u.unwrap();
        let u_p = self.u_p.unwrap();
        let u_m = self.u_m.unwrap();
        let v = self.v.unwrap();
        let v_p = self.v_p.unwrap();
        let v_m = self.v_m.unwrap();
        // Compute the azimuthal velocity
        let theta = self.compute_theta_with(r_h, r_g, u, v, params);
        let theta_p = self.compute_theta_with(r_h_p, r_g_p, u_p, v_p, params);
        let theta_m = self.compute_theta_with(r_h_m, r_g_m, u_m, v_m, params);
        self.theta = Some(theta);
        self.theta_p = Some(theta_p);
        self.theta_m = Some(theta_m);
        self.theta_ep = Some(theta_p - theta);
        self.theta_em = Some(theta - theta_m);
    }
    /// Compute the uncertainty in the azimuthal velocity inherited from velocities
    ///
    /// Note that we compute all values again, starting from the independent errors
    ///
    /// Sources: Gromov, Nikiforov, Ossipkov (2016)
    #[allow(clippy::similar_names)]
    pub(super) fn compute_theta_evel(&mut self, params: &Params<F>)
    where
        F: Float + Debug + Default,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let par = self.par.unwrap();
        let v_lsr = self.v_lsr.unwrap();
        let v_lsr_e = self.v_lsr_e.unwrap();
        let mu_x = self.mu_x.unwrap();
        let mu_x_e = self.mu_x_e.unwrap();
        let mu_y = self.mu_y.unwrap();
        let mu_y_e = self.mu_y_e.unwrap();
        let r_h = self.r_h.unwrap();
        // Get the parameters
        let k = params.k;
        // Compute the partial derivative of the azimuthal
        // velocity by the Local Standard of Rest velocity
        let mut object = Object {
            alpha: Some(FT::cst(alpha)),
            delta: Some(FT::cst(delta)),
            par: Some(FT::cst(par)),
            v_lsr: Some(FT::var(v_lsr)),
            mu_x: Some(FT::cst(mu_x)),
            mu_y: Some(FT::cst(mu_y)),
            ..Default::default()
        };
        object.compute_l_b(params);
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_cos_b_mu_b(params);
        object.compute_v_r(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let deriv_theta_v_lsr = object.theta.unwrap().deriv();
        // Compute the partial derivative of the azimuthal
        // velocity by the Eastward proper motion
        object.v_lsr = Some(FT::cst(v_lsr));
        object.mu_x = Some(FT::var(mu_x));
        object.compute_l_b(params);
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_cos_b_mu_b(params);
        object.compute_v_r(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let deriv_theta_mu_x = object.theta.as_ref().unwrap().deriv();
        // Compute the partial derivative of the azimuthal
        // velocity by the Northward proper motion
        object.mu_x = Some(FT::cst(mu_x));
        object.mu_y = Some(FT::var(mu_y));
        object.compute_l_b(params);
        object.compute_r_h_nominal();
        object.compute_r_g_nominal(params);
        object.compute_mu_l_cos_b_mu_b(params);
        object.compute_v_r(params);
        object.compute_v_l_v_b_nominal(params);
        object.compute_u_v_w_nominal();
        object.compute_theta_nominal(params);
        let deriv_theta_mu_y = object.theta.as_ref().unwrap().deriv();

        let mut d_v_lsr = v_lsr_e.powi(2);
        let mut d_mu_x = mu_x_e.powi(2);
        let mut d_mu_y = mu_y_e.powi(2);
        // We account for the uncertainty in transferring the
        // maser motions to that of the central star by adding
        // an error term here for non-Reid objects.
        //
        // See Reid et al. (2019)
        if !self.from_reid.as_ref().unwrap() {
            let extra_term_v = F::from(VEL_TERM).unwrap().powi(2);
            let extra_term_mu = extra_term_v / k.powi(2) / r_h.powi(2);
            d_v_lsr = d_v_lsr + extra_term_v;
            d_mu_x = d_mu_x + extra_term_mu;
            d_mu_y = d_mu_y + extra_term_mu;
        }

        // Compute the uncertainty
        self.theta_evel = Some(F::sqrt(
            deriv_theta_v_lsr.powi(2) * d_v_lsr
                + deriv_theta_mu_x.powi(2) * d_mu_x
                + deriv_theta_mu_y.powi(2) * d_mu_y,
        ));
    }
}
