//! Proper motions in Galactic coordinates

use super::{Measurement, Object};
use crate::model::Params;

use core::fmt::{Debug, Display};

use autodiff::FT;
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
    /// Compute the proper motions in Galactic coordinates
    pub fn compute_mu_l_mu_b<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        // Convert the proper motions in equatorial
        // coordinates from mas/yr to rad/yr
        let mu_alpha = (mu_x.v / delta.cos() / 3600. / 1000.).to_radians();
        let mu_delta = (mu_y.v / 3600. / 1000.).to_radians();
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
        self.mu_l = Some(mu_l_rad.to_degrees() * 3600. * 1000.);
        self.mu_b = Some(mu_b_rad.to_degrees() * 3600. * 1000.);
    }
    /// Compute the dispersions of `mu_l * cos(b)` and `mu_b`
    ///
    /// Note that only values with independent
    /// errors (from the catalog) are unpacked.
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    pub fn compute_e_mu_l_mu_b(&self, params: &Params<F>) -> (F, F) {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        // Compute the observed dispersions
        let d_mu_x = mu_x.e_p.powi(2);
        let d_mu_y = mu_y.e_p.powi(2);
        // Compute the partial derivatives of
        // `mu_l * cos(b)` by `mu_alpha * cos(delta)`
        // and `mu_b` by `mu_alpha * cos(delta)`
        let mut object = Object {
            alpha: Some(FT::cst(alpha)),
            delta: Some(FT::cst(delta)),
            mu_x: Some(Measurement {
                v: FT::var(mu_x.v),
                ..Default::default()
            }),
            mu_y: Some(Measurement {
                v: FT::cst(mu_y.v),
                ..Default::default()
            }),
            ..Default::default()
        };
        object.compute_l_b(params);
        object.compute_mu_l_mu_b(params);
        let deriv_mu_l_cos_b_mu_x_sq = object.mu_l.unwrap().deriv().powi(2);
        let deriv_mu_b_mu_x_sq = object.mu_b.unwrap().deriv().powi(2);
        // Compute the partial derivatives of
        // `mu_l * cos(b)` by `mu_delta`
        // and `mu_b` by `mu_delta`
        object.mu_x = Some(Measurement {
            v: FT::cst(mu_x.v),
            ..Default::default()
        });
        object.mu_y = Some(Measurement {
            v: FT::var(mu_y.v),
            ..Default::default()
        });
        object.compute_l_b(params);
        object.compute_mu_l_mu_b(params);
        let deriv_mu_l_cos_b_mu_y_sq = object.mu_l.unwrap().deriv().powi(2);
        let deriv_mu_b_mu_y_sq = object.mu_b.unwrap().deriv().powi(2);
        // Compute the dispersion of `mu_l * cos(b)`
        let sigma_mu_l_cos_b_sq =
            deriv_mu_l_cos_b_mu_x_sq * d_mu_x + deriv_mu_l_cos_b_mu_y_sq * d_mu_y;
        // Compute the dispersion of `mu_b`
        let sigma_mu_b_sq = deriv_mu_b_mu_x_sq * d_mu_x + deriv_mu_b_mu_y_sq * d_mu_y;
        // Return the results
        (sigma_mu_l_cos_b_sq, sigma_mu_b_sq)
    }
}
