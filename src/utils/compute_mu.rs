//! Convert the proper motions in equatorial coordinates
//! to proper motions in Galactic coordinates

#![allow(clippy::module_name_repetitions)]

use crate::model::{Object, Params};

use core::fmt::Debug;

use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Convert proper motions in equatorial coordinates
/// to proper motions in Galactic coordinates
#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(<F as num::NumCast>::from(literal).unwrap())]
pub fn compute_mu<F, F2>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    mu_x: F,
    mu_y: F,
    params: &Params<F2>,
) -> (F, F)
where
    F: Float + Debug + Default + FloatConst,
    F2: Float + Debug + Into<F>,
{
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
