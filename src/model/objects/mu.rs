//! Proper motions in Galactic coordinates

use crate::model::fit::params::VEL_TERM;

use super::{Object, Params};

use core::fmt::Debug;

use autodiff::FT;
use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F> {
    /// Compute the proper motions in Galactic coordinates
    pub fn compute_mu_l_cos_b_mu_b<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug + Default,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        let mu_x = self.mu_x.unwrap();
        let mu_y = self.mu_y.unwrap();
        // Convert the proper motions in equatorial
        // coordinates from mas/yr to rad/yr
        let mu_alpha = (mu_x / delta.cos() / 3600. / 1000.).to_radians();
        let mu_delta = (mu_y / 3600. / 1000.).to_radians();
        // Compute the proper motions in Galactic coordinates
        // (the difference in the coordinates in 1-year period)
        let mut object = Object {
            alpha: Some(alpha + mu_alpha),
            delta: Some(delta + mu_delta),
            ..Default::default()
        };
        object.compute_l_b(params);
        let l_ahead = object.l.unwrap();
        let b_ahead = object.b.unwrap();
        let mu_l_rad = l_ahead - l;
        let mu_b_rad = b_ahead - b;
        // Convert the proper motions in Galactic
        // coordinates from rad/yr to mas/yr
        self.mu_l_cos_b = Some(mu_l_rad.to_degrees() * 3600. * 1000. * b.cos());
        self.mu_b = Some(mu_b_rad.to_degrees() * 3600. * 1000.);
    }
    /// Compute the dispersions of `mu_l * cos(b)` and `mu_b`
    ///
    /// Note that only values with independent
    /// errors (from the catalog) are unpacked.
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    pub fn compute_d_mu_l_cos_b_mu_b(&self, params: &Params<F>) -> (F, F)
    where
        F: Float + Debug + Default,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let mu_x = self.mu_x.unwrap();
        let mu_y = self.mu_y.unwrap();
        let mu_x_e = self.mu_x_e.unwrap();
        let mu_y_e = self.mu_y_e.unwrap();
        let r_h = self.r_h.unwrap();
        // Get the parameters
        let k = params.k;
        // Compute the partial derivatives of
        // `mu_l * cos(b)` by `mu_alpha * cos(delta)`
        // and `mu_b` by `mu_alpha * cos(delta)`
        let mut object = Object {
            alpha: Some(FT::cst(alpha)),
            delta: Some(FT::cst(delta)),
            mu_x: Some(FT::var(mu_x)),
            mu_y: Some(FT::cst(mu_y)),
            ..Default::default()
        };
        object.compute_l_b(params);
        object.compute_mu_l_cos_b_mu_b(params);
        let deriv_mu_l_cos_b_mu_x_sq = object.mu_l_cos_b.unwrap().deriv().powi(2);
        let deriv_mu_b_mu_x_sq = object.mu_b.unwrap().deriv().powi(2);
        // Compute the partial derivatives of
        // `mu_l * cos(b)` by `mu_delta`
        // and `mu_b` by `mu_delta`
        object.mu_x = Some(FT::cst(mu_x));
        object.mu_y = Some(FT::var(mu_y));
        object.compute_l_b(params);
        object.compute_mu_l_cos_b_mu_b(params);
        let deriv_mu_l_cos_b_mu_y_sq = object.mu_l_cos_b.unwrap().deriv().powi(2);
        let deriv_mu_b_mu_y_sq = object.mu_b.unwrap().deriv().powi(2);
        // Compute the observed dispersions
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
            d_mu_x = d_mu_x + extra_term_mu;
            d_mu_y = d_mu_y + extra_term_mu;
        }
        // Compute the dispersion of `mu_l * cos(b)`
        let d_mu_l_cos_b = deriv_mu_l_cos_b_mu_x_sq * d_mu_x + deriv_mu_l_cos_b_mu_y_sq * d_mu_y;
        // Compute the dispersion of `mu_b`
        let d_mu_b = deriv_mu_b_mu_x_sq * d_mu_x + deriv_mu_b_mu_y_sq * d_mu_y;
        // Return the results
        (d_mu_l_cos_b, d_mu_b)
    }
}
