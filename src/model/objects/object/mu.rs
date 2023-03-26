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
    /// Compute the galactocentric distance with the specific values
    pub fn compute_mu_l_mu_b_with<F2>(&self, mu_x: F, mu_y: F, params: &Params<F2>) -> (F, F)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let l = self.l.unwrap();
        let b = self.b.unwrap();
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
        let mu_l = mu_l_rad.to_degrees() * 3600. * 1000.;
        let mu_b = mu_b_rad.to_degrees() * 3600. * 1000.;
        (mu_l, mu_b)
    }
    /// Compute the galactocentric distance (nominal values only)
    pub fn compute_mu_l_mu_b_nominal<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        // Convert the proper motions in Galactic
        // coordinates from rad/yr to mas/yr
        let (mu_l, mu_b) = self.compute_mu_l_mu_b_with(mu_x.v, mu_y.v, params);
        self.mu_l = Some(Measurement {
            v: mu_l,
            ..Default::default()
        });
        self.mu_b = Some(Measurement {
            v: mu_b,
            ..Default::default()
        });
    }
    /// Compute the galactocentric distance
    pub fn compute_mu_l_mu_b<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        // Convert the proper motions in Galactic
        // coordinates from rad/yr to mas/yr
        let (mu_l, mu_b) = self.compute_mu_l_mu_b_with(mu_x.v, mu_y.v, params);
        let (mu_l_u, mu_b_u) = self.compute_mu_l_mu_b_with(mu_x.v_u, mu_y.v_u, params);
        let (mu_l_l, mu_b_l) = self.compute_mu_l_mu_b_with(mu_x.v_l, mu_y.v_l, params);
        self.mu_l = Some(Measurement {
            v: mu_l,
            v_u: mu_l_u,
            v_l: mu_l_l,
            e_p: mu_l_u - mu_l,
            e_m: mu_l - mu_l_l,
        });
        self.mu_b = Some(Measurement {
            v: mu_b,
            v_u: mu_b_u,
            v_l: mu_b_l,
            e_p: mu_b_u - mu_b,
            e_m: mu_b - mu_b_l,
        });
    }
    /// Compute the dispersions of `mu_l * cos(b)` and `mu_b`
    ///
    /// Note that only values with independent errors are in the parameters.
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    pub fn compute_e_mu_l_mu_b(&self, params: &Params<F>) -> (F, F) {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        let mu_x = self.mu_x.as_ref().unwrap();
        let mu_y = self.mu_y.as_ref().unwrap();
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        // Compute the observed dispersions
        let d_mu_x = mu_x.e_p.powi(2);
        let d_mu_y = mu_y.e_p.powi(2);
        // Compute the partial derivatives of
        // `mu_l * cos(b)` by `mu_alpha * cos(delta)`
        // and `mu_b` by `mu_alpha * cos(delta)`
        let mut object = Object {
            alpha: Some(FT::cst(alpha)),
            delta: Some(FT::cst(delta)),
            l: Some(FT::cst(l)),
            b: Some(FT::cst(b)),
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
        object.compute_mu_l_mu_b_nominal(params);
        let deriv_mu_l_cos_b_mu_x_sq = object.mu_l.as_ref().unwrap().v.deriv().powi(2);
        let deriv_mu_b_mu_x_sq = object.mu_b.as_ref().unwrap().v.deriv().powi(2);
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
        object.compute_mu_l_mu_b_nominal(params);
        let deriv_mu_l_cos_b_mu_y_sq = object.mu_l.as_ref().unwrap().v.deriv().powi(2);
        let deriv_mu_b_mu_y_sq = object.mu_b.as_ref().unwrap().v.deriv().powi(2);
        // Compute the dispersion of `mu_l * cos(b)`
        let sigma_mu_l_cos_b_sq =
            deriv_mu_l_cos_b_mu_x_sq * d_mu_x + deriv_mu_l_cos_b_mu_y_sq * d_mu_y;
        // Compute the dispersion of `mu_b`
        let sigma_mu_b_sq = deriv_mu_b_mu_x_sq * d_mu_x + deriv_mu_b_mu_y_sq * d_mu_y;
        // Return the results
        (sigma_mu_l_cos_b_sq, sigma_mu_b_sq)
    }
}
