//! Data object

mod distances;
mod equatorial_s;
mod galactic_c;
mod galactic_s;
mod measurement;
mod rotation_c;

use crate::model::io::input;
use crate::Goal;
use distances::Distances;
use equatorial_s::EquatorialSpherical;
use galactic_c::GalacticCartesian;
use galactic_s::GalacticSpherical;
use measurement::Measurement;
use rotation_c::RotationCurve;

use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use num::Float;

/// Data object
#[derive(Debug, Default)]
pub(in crate::model) struct Object<F>
where
    F: Float + Default + Debug,
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
    /// Distances
    distances: Option<Distances<F>>,
    /// Galactic heliocentric spherical coordinates
    galactic_s: Option<GalacticSpherical<F>>,
    /// Galactic heliocentric Cartesian coordinates
    galactic_c: Option<GalacticCartesian<F>>,
    /// Rotation curve
    rotation_c: Option<RotationCurve<F>>,
    /// Type of the object
    obj_type: Option<String>,
    /// Source of the data
    source: Option<String>,
}

impl<F: Float + Default + Debug> Object<F> {
    /// Unwrap the name of the object
    pub(in crate::model) fn name(&self) -> Result<&String> {
        self.name
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the name"))
    }
    /// Unwrap the equatorial spherical coordinates
    pub(in crate::model) fn equatorial_s(&self) -> Result<&EquatorialSpherical<F>> {
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
    pub(in crate::model) fn galactic_s(&self) -> Result<&GalacticSpherical<F>> {
        self.galactic_s
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Galactic spherical coordinates"))
    }
    /// Unwrap the parallax
    pub(in crate::model) fn par(&self) -> Result<&Measurement<F>> {
        self.par
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the parallax"))
    }
    /// Unwrap the Local Standard of Rest velocity
    pub(in crate::model) fn v_lsr(&self) -> Result<&Measurement<F>> {
        self.v_lsr
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Local Standard of Rest velocity"))
    }
    /// Unwrap the Eastward proper motion
    pub(in crate::model) fn mu_x(&self) -> Result<&Measurement<F>> {
        self.mu_x
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Eastward proper motion"))
    }
    /// Unwrap the Northward proper motion
    pub(in crate::model) fn mu_y(&self) -> Result<&Measurement<F>> {
        self.mu_y
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the Northward proper motion"))
    }
    /// Unwrap the distances
    pub(in crate::model) fn distances(&self) -> Result<&Distances<F>> {
        self.distances
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the distances"))
    }
    /// Unwrap the rotation curve
    pub(in crate::model) fn rotation_c(&self) -> Result<&RotationCurve<F>> {
        self.rotation_c
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the rotation curve"))
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
    pub(in crate::model) fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        if goals.contains(&Goal::Coords) {
            // Convert equatorial coordinates to Galactic
            // heliocentric spherical coordinates
            self.compute_galactic_s()
                .with_context(|| "Couldn't compute the Galactic spherical coordinates")?;
            // Compute the distances
            self.compute_distances()
                .with_context(|| "Couldn't compute the distances")?;
            // Convert equatorial coordinates to Galactic
            // heliocentric Cartesian coordinates
            self.compute_galactic_c()
                .with_context(|| "Couldn't compute the Galactic Cartesian coordinates")?;
        }
        if goals.contains(&Goal::RotationCurve) {
            // Convert equatorial coordinates to Galactic
            // heliocentric spherical coordinates
            self.compute_galactic_s()
                .with_context(|| "Couldn't compute the Galactic spherical coordinates")?;
            // Compute the rotation curve
            self.compute_rotation_c()
                .with_context(|| "Couldn't compute the rotation curve")?;
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
    fn compute_galactic_s(&mut self) -> Result<()> {
        self.galactic_s
            .get_or_insert(GalacticSpherical::try_from(&*self)?);
        Ok(())
    }
    /// Compute the distances
    fn compute_distances(&mut self) -> Result<()> {
        self.distances.get_or_insert(Distances::try_from(&*self)?);
        Ok(())
    }
    /// Compute the rotation curve
    fn compute_rotation_c(&mut self) -> Result<()> {
        self.rotation_c
            .get_or_insert(RotationCurve::try_from(&*self)?);
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
            v_l: record.par - record.e_par,
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
