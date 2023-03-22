//! Data object

mod equatorial_s;
mod galactic_c;
mod galactic_s;
mod measurement;
mod r_g;
mod theta;

use crate::model::io::input;
use crate::model::Params;
use crate::Goal;
use equatorial_s::EquatorialSpherical;
use galactic_c::GalacticCartesian;
use galactic_s::GalacticSpherical;
pub use measurement::Measurement;
use r_g::GalactocentricDistance;
use theta::AzimuthalVelocity;

use core::fmt::{Debug, Display};
use core::str::FromStr;
use std::error::Error;

use anyhow::{anyhow, Context, Result};
use num::{traits::FloatConst, Float};

/// Data object
#[derive(Clone, Debug, Default)]
pub struct Object<F>
where
    F: Float + Debug,
{
    /// Name of the object
    name: Option<String>,
    /// Equatorial spherical coordinates
    equatorial_s: Option<EquatorialSpherical<F>>,
    /// Parallax (mas)
    par: Option<Measurement<F>>,
    /// Local Standard of Rest velocity (km/s)
    v_lsr: Option<Measurement<F>>,
    /// Eastward proper motion (mas/yr)
    mu_x: Option<Measurement<F>>,
    /// Northward proper motion (mas/yr)
    mu_y: Option<Measurement<F>>,
    /// Galactic heliocentric spherical coordinates
    galactic_s: Option<GalacticSpherical<F>>,
    /// Galactocentric distance (kpc)
    r_g: Option<GalactocentricDistance<F>>,
    /// Galactic heliocentric Cartesian coordinates
    galactic_c: Option<GalacticCartesian<F>>,
    /// Azimuthal velocity
    theta: Option<AzimuthalVelocity<F>>,
    /// Type of the object
    obj_type: Option<String>,
    /// Source of the data
    source: Option<String>,
}

impl<F: Float + FloatConst + Default + Display + Debug> Object<F> {
    /// Unwrap the name of the object
    pub(in crate::model) fn name(&self) -> Result<&String> {
        self.name
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the name"))
    }
    /// Unwrap the equatorial spherical coordinates
    pub fn equatorial_s(&self) -> Result<&EquatorialSpherical<F>> {
        self.equatorial_s
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the equatorial spherical coordinates"))
    }
    /// Unwrap the Galactic heliocentric Cartesian coordinates
    pub(in crate::model) fn galactic_c(&self) -> Result<&GalacticCartesian<F>> {
        self.galactic_c
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactic Cartesian coordinates"))
    }
    /// Unwrap the Galactic heliocentric spherical coordinates
    pub fn galactic_s(&self) -> Result<&GalacticSpherical<F>> {
        self.galactic_s
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactic spherical coordinates"))
    }
    /// Unwrap the parallax
    pub fn par(&self) -> Result<&Measurement<F>> {
        self.par
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the parallax"))
    }
    /// Unwrap the Local Standard of Rest velocity
    pub fn v_lsr(&self) -> Result<&Measurement<F>> {
        self.v_lsr
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Local Standard of Rest velocity"))
    }
    /// Unwrap the Eastward proper motion
    pub fn mu_x(&self) -> Result<&Measurement<F>> {
        self.mu_x
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Eastward proper motion"))
    }
    /// Unwrap the Northward proper motion
    pub fn mu_y(&self) -> Result<&Measurement<F>> {
        self.mu_y
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Northward proper motion"))
    }
    /// Unwrap the Galactocentric distance
    pub(in crate::model) fn r_g(&self) -> Result<&GalactocentricDistance<F>> {
        self.r_g
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactocentric distance"))
    }
    /// Unwrap the azimuthal velocity
    pub(in crate::model) fn theta(&self) -> Result<&AzimuthalVelocity<F>> {
        self.theta
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the azimuthal velocity"))
    }
    /// Unwrap the type of the object
    pub(in crate::model) fn obj_type(&self) -> Result<&String> {
        self.obj_type
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the type of the object"))
    }
    /// Unwrap the source of the data
    pub(in crate::model) fn source(&self) -> Result<&String> {
        self.source
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the source of the data"))
    }
    /// Perform computations based on goals
    pub(in crate::model) fn compute(&mut self, goals: &[Goal], params: &Params<F>) -> Result<()> {
        // If coordinates conversion was requested
        if goals.contains(&Goal::Coords) {
            // Convert equatorial coordinates to Galactic
            // heliocentric spherical coordinates
            self.compute_galactic_s(params)
                .with_context(|| "Couldn't compute the Galactic spherical coordinates")?;
            // Compute the Galactocentric distance
            self.compute_r_g(params)
                .with_context(|| "Couldn't compute the Galactocentric distances")?;
            // Convert equatorial coordinates to Galactic
            // heliocentric Cartesian coordinates
            self.compute_galactic_c()
                .with_context(|| "Couldn't compute the Galactic Cartesian coordinates")?;
        }
        // If there is a goal to compute the rotation curve
        if goals.contains(&Goal::RotationCurve) {
            // Convert equatorial coordinates to Galactic
            // heliocentric spherical coordinates
            self.compute_galactic_s(params)
                .with_context(|| "Couldn't compute the Galactic spherical coordinates")?;
            // Compute the Galactocentric distance
            self.compute_r_g(params)
                .with_context(|| "Couldn't compute the Galactocentric distances")?;
            // Compute the azimuthal velocity
            self.compute_theta(params)
                .with_context(|| "Couldn't compute the azimuthal velocity")?;
        }
        Ok(())
    }
    /// Convert equatorial coordinates to Galactic
    /// heliocentric Cartesian coordinates
    fn compute_galactic_c(&mut self) -> Result<()> {
        self.galactic_c
            .get_or_insert(GalacticCartesian::try_from(&*self)?);
        Ok(())
    }
    /// Convert equatorial coordinates to Galactic
    /// heliocentric spherical coordinates
    pub(in crate::model) fn compute_galactic_s(&mut self, params: &Params<F>) -> Result<()> {
        self.galactic_s
            .get_or_insert(GalacticSpherical::try_from(&*self, params)?);
        Ok(())
    }
    /// Compute the Galactocentric distance
    pub(in crate::model) fn compute_r_g(&mut self, params: &Params<F>) -> Result<()> {
        self.r_g
            .get_or_insert(GalactocentricDistance::try_from(&*self, params)?);
        Ok(())
    }
    /// Compute the azimuthal velocity
    fn compute_theta(&mut self, params: &Params<F>) -> Result<()> {
        self.theta
            .get_or_insert(AzimuthalVelocity::try_from(&*self, params)?);
        Ok(())
    }
}

impl<F> TryFrom<input::Record<F>> for Object<F>
where
    F: Float + Default + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(record: input::Record<F>) -> Result<Self> {
        // Initialize an empty object
        let mut object = Self::default();
        // Unpack the data into the object
        object.equatorial_s.replace(
            EquatorialSpherical::try_from(&record)
                .with_context(|| "Couldn't parse the equatorial coordinates")?,
        );
        object.name.replace(record.name);
        object.par.replace(Measurement {
            v: record.par,
            v_u: record.par + record.e_par,
            // In some cases the uncertainty of the value can be greater than the nominal value,
            // hence leading to negative results in this subtraction. We avoid this here,
            // since there is no such thing as a negative parallax. In case of zero
            // being the maximum value, we will get the `Inf` distance along the way
            v_l: F::max(F::zero(), record.par - record.e_par),
            e_p: record.e_par,
            e_m: record.e_par,
        });
        object.v_lsr.replace(Measurement {
            v: record.v_lsr,
            v_u: record.v_lsr + record.e_v_lsr,
            v_l: record.v_lsr - record.e_v_lsr,
            e_p: record.e_v_lsr,
            e_m: record.e_v_lsr,
        });
        object.mu_x.replace(Measurement {
            v: record.mu_x,
            v_u: record.mu_x + record.e_mu_x,
            v_l: record.mu_x - record.e_mu_x,
            e_p: record.e_mu_x,
            e_m: record.e_mu_x,
        });
        object.mu_y.replace(Measurement {
            v: record.mu_y,
            v_u: record.mu_y + record.e_mu_y,
            v_l: record.mu_y - record.e_mu_y,
            e_p: record.e_mu_y,
            e_m: record.e_mu_y,
        });
        object.obj_type.replace(record.obj_type);
        object.source.replace(record.source);
        Ok(object)
    }
}
