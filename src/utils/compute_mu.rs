//! Convert the proper motions in equatorial coordinates
//! to proper motions in Galactic coordinates

#![allow(clippy::module_name_repetitions)]

use super::to_spherical;
use crate::model::Params;

use core::fmt::Debug;

use autodiff::FT;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Compute proper motions in equatorial
/// coordinates from the array of arguments
pub fn compute_mu_from<F: Float + FloatConst + Debug>(
    args: &[FT<F>; 6],
    params: &Params<F>,
) -> (FT<F>, FT<F>) {
    compute_mu(args[0], args[1], args[2], args[3], args[4], args[5], params)
}

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
    F: Float + Debug + FloatConst + From<F2>,
    F2: Float + Debug,
{
    // Note that we don't use `to_radians` and `to_degrees` methods
    // here because those (possibly a bug) result in a loss of
    // the value of the derivative in dual numbers
    //
    // Convert the proper motions in equatorial
    // coordinates from mas/yr to rad/yr
    let mu_alpha = (mu_x / delta.cos() / 3600. / 1000.) * F::PI() / 180.;
    let mu_delta = (mu_y / 3600. / 1000.) * F::PI() / 180.;
    // Compute the proper motions in Galactic coordinates
    // (the difference in the coordinates in 1-year period)
    let (l_ahead, b_ahead) = to_spherical(alpha + mu_alpha, delta + mu_delta, params);
    let mu_l_rad = l_ahead - l;
    let mu_b_rad = b_ahead - b;
    // Convert the proper motions in Galactic
    // coordinates from rad/yr to mas/yr
    let mu_l = mu_l_rad * 180. / F::PI() * 3600. * 1000.;
    let mu_b = mu_b_rad * 180. / F::PI() * 3600. * 1000.;
    (mu_l, mu_b)
}
