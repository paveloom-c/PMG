//! Data object

mod equatorial_spherical;
mod galactic_cartesian;
mod galactic_spherical;
mod measurement;
mod mu;
mod r_g;
mod theta;

use crate::model::io::input;
use crate::model::Params;
use crate::Goal;
pub use measurement::Measurement;

use core::fmt::{Debug, Display};
use core::str::FromStr;
use std::error::Error;

use anyhow::Result;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Data object
#[derive(Clone, Debug, Default)]
pub struct Object<F>
where
    F: Float + Debug,
{
    /// Name of the object
    pub name: Option<String>,
    /// Type of the object
    pub obj_type: Option<String>,
    /// Source of the data
    pub source: Option<String>,
    /// Right ascension (radians)
    pub alpha: Option<F>,
    /// Declination (radians)
    pub delta: Option<F>,
    /// Parallax (mas)
    pub par: Option<Measurement<F>>,
    /// Local Standard of Rest velocity (km/s)
    pub v_lsr: Option<Measurement<F>>,
    /// Eastward proper motion (mas/yr)
    pub mu_x: Option<Measurement<F>>,
    /// Northward proper motion (mas/yr)
    pub mu_y: Option<Measurement<F>>,
    /// Heliocentric distance (kpc)
    pub r_h: Option<Measurement<F>>,
    /// Longitude (radians)
    pub l: Option<F>,
    /// Latitude (radians)
    pub b: Option<F>,
    /// Proper motion in longitude
    pub mu_l: Option<Measurement<F>>,
    /// Proper motion in latitude
    pub mu_b: Option<Measurement<F>>,
    /// Galactocentric distance (kpc)
    pub r_g: Option<Measurement<F>>,
    /// X coordinate (kpc)
    pub x: Option<Measurement<F>>,
    /// Y coordinate (kpc)
    pub y: Option<Measurement<F>>,
    /// Z coordinate (kpc)
    pub z: Option<Measurement<F>>,
    /// Azimuthal velocity (km/s)
    ///
    /// Uncertainties here are inherited from
    /// the uncertainties of the parallax
    pub theta: Option<Measurement<F>>,
    /// Uncertainty in azimuthal velocity (km/s)
    /// inherited from the velocities
    pub e_vel_theta: Option<F>,
    /// Heliocentric velocity in distance (km/s)
    pub v_r: Option<Measurement<F>>,
    /// Heliocentric velocity in longitude (km/s)
    pub v_l: Option<Measurement<F>>,
    /// Heliocentric velocity in latitude (km/s)
    pub v_b: Option<Measurement<F>>,
    /// U coordinate (kpc)
    pub u: Option<Measurement<F>>,
    /// V coordinate (kpc)
    pub v: Option<Measurement<F>>,
    /// W coordinate (kpc)
    pub w: Option<Measurement<F>>,
}

impl<F: Float + FloatConst + Default + Display + Debug> Object<F> {
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal], params: &Params<F>) {
        let compute_coords = goals.contains(&Goal::Coords);
        let compute_rotation_curve = goals.contains(&Goal::RotationCurve);
        if compute_coords || compute_rotation_curve {
            self.compute_l_b(params);
            self.compute_r_h();
            self.compute_r_g(params);
            self.compute_mu_l_mu_b(params);
            self.compute_v_r(params);
            self.compute_v_l_v_b(params);
            self.compute_u_v_w();
            if compute_coords {
                self.compute_x_y_z();
            }
            if compute_rotation_curve {
                self.compute_theta(params);
                self.compute_e_vel_theta(params);
            }
        }
    }
}

impl<F> TryFrom<input::Record<F>> for Object<F>
where
    F: Float + Default + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn try_from(record: input::Record<F>) -> Result<Self> {
        let mut object = Self::default();
        object.try_parse_alpha(&record)?;
        object.try_parse_delta(&record)?;
        object.name = Some(record.name);
        object.obj_type = Some(record.obj_type);
        object.source = Some(record.source);
        object.par = Some(Measurement {
            v: record.par,
            v_u: record.par + record.e_par,
            // In some cases the uncertainty of the value can be greater than
            // the nominal value, hence leading to non-positive results in this
            // subtraction. We avoid this here since there is no such thing
            // as a non-positive parallax. Instead, we assume the distance to be
            // a finite, but sufficiently big value.
            v_l: {
                let v_l = record.par - record.e_par;
                if v_l > 0.0 {
                    v_l
                } else {
                    1. / 50.
                }
            },
            e_p: record.e_par,
            e_m: record.e_par,
        });
        object.v_lsr = Some(Measurement {
            v: record.v_lsr,
            v_u: record.v_lsr + record.e_v_lsr,
            v_l: record.v_lsr - record.e_v_lsr,
            e_p: record.e_v_lsr,
            e_m: record.e_v_lsr,
        });
        object.mu_x = Some(Measurement {
            v: record.mu_x,
            v_u: record.mu_x + record.e_mu_x,
            v_l: record.mu_x - record.e_mu_x,
            e_p: record.e_mu_x,
            e_m: record.e_mu_x,
        });
        object.mu_y = Some(Measurement {
            v: record.mu_y,
            v_u: record.mu_y + record.e_mu_y,
            v_l: record.mu_y - record.e_mu_y,
            e_p: record.e_mu_y,
            e_m: record.e_mu_y,
        });
        Ok(object)
    }
}
