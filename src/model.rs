//! Model of the Galaxy

mod bounds;
mod io;
mod objects;
mod params;

use crate::cli::Args;
use crate::utils;
use crate::Goal;
use bounds::Bounds;
pub use objects::{Measurement, Object, Objects};
pub use params::Params;

use core::fmt::{Debug, Display};
use core::iter::Sum;
use core::str::FromStr;
use std::error::Error;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use num::{traits::FloatConst, Float};
use rand::distributions::uniform::SampleUniform;
use rand_distr::Distribution;
use rand_distr::StandardNormal;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug, Default)]
pub struct Model<F>
where
    F: Float + Debug,
    StandardNormal: Distribution<F>,
{
    /// Initial model parameters
    params: Params<F>,
    /// Bounds of the initial parameters
    bounds: Bounds<F>,
    /// Data objects
    objects: Objects<F>,
    /// Fitted model parameters
    fitted_params: Option<Params<F>>,
}

impl<F> Model<F>
where
    F: Float + FloatConst + SampleUniform + Default + Display + Debug + Sync + Send + Sum,
    StandardNormal: Distribution<F>,
{
    /// Unwrap the fit of the model
    pub(in crate::model) fn fitted_params(&self) -> Result<&Params<F>> {
        self.fitted_params
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't unwrap the fitted parameters"))
    }
    /// Perform computations based on goals
    pub fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        // Perform per-object goals first
        self.objects.compute(goals, &self.params)?;
        // If fitting of the model was requested
        if goals.contains(&Goal::Fit) {
            // Try to fit the model
            self.fitted_params.get_or_insert(
                self.params
                    .try_fit_from(&self.bounds, &self.objects)
                    .with_context(|| "Couldn't fit the model")?,
            );
        }
        Ok(())
    }
    /// Extend the model by parsing and appending the data
    /// from the file, doing conversions where necessary
    fn extend(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Debug + FromStr + DeserializeOwned,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the data from the file
        let objects = Objects::try_from(path).with_context(|| "Couldn't parse the data")?;
        // Extend the model
        self.objects.extend(objects);
        Ok(())
    }
    /// Write the model data to files in the
    /// output directory based on the goals
    pub fn write_to(&self, dir: &Path, goals: &[Goal]) -> Result<()>
    where
        F: Serialize,
    {
        // Make sure the output directories exist
        let dat_dir = &dir.join("dat");
        let bin_dir = &dir.join("bin");
        fs::create_dir_all(dat_dir)
            .with_context(|| format!("Couldn't create the output directory {dat_dir:?}"))?;
        fs::create_dir_all(bin_dir)
            .with_context(|| format!("Couldn't create the output directory {bin_dir:?}"))?;
        // Write the coordinates if that was a goal
        if goals.contains(&Goal::Coords) {
            self.serialize_to_coords(dat_dir, bin_dir)
                .with_context(|| "Couldn't write the Galactic coordinates to a file")?;
        };
        // Write the rotation curve if that was a goal
        if goals.contains(&Goal::RotationCurve) {
            self.serialize_to_rotcurve(dat_dir, bin_dir)
                .with_context(|| "Couldn't write the rotation curve to a file")?;
        };
        // Write the fit of the model if that was a goal
        if goals.contains(&Goal::Fit) {
            self.serialize_to_fit(dat_dir, bin_dir)
                .with_context(|| "Couldn't write the rotation curve to a file")?;
        };
        Ok(())
    }
}

impl<F> TryFrom<&Args> for Model<F>
where
    F: Float
        + FloatConst
        + SampleUniform
        + Default
        + Display
        + Debug
        + FromStr
        + DeserializeOwned
        + Sync
        + Send
        + Sum,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
    StandardNormal: Distribution<F>,
{
    type Error = anyhow::Error;

    fn try_from(args: &Args) -> Result<Self> {
        // Initialize an empty model
        let mut model = Self {
            params: Params {
                alpha_ngp: utils::cast(args.alpha_ngp)?,
                delta_ngp: utils::cast(args.delta_ngp)?,
                k: utils::cast(args.k)?,
                l_ncp: utils::cast(args.l_ncp)?,
                r_0: utils::cast(args.r_0)?,
                theta_sun: utils::cast(args.theta_sun)?,
                u_sun: utils::cast(args.u_sun)?,
                u_sun_standard: utils::cast(args.u_sun_standard)?,
                v_sun_standard: utils::cast(args.v_sun_standard)?,
                w_sun_standard: utils::cast(args.w_sun_standard)?,
                omega_0: utils::cast(args.omega_0)?,
                a: utils::cast(args.a)?,
                sigma_r: utils::cast(args.sigma_r)?,
                sigma_theta: utils::cast(args.sigma_theta)?,
                sigma_z: utils::cast(args.sigma_z)?,
            },
            bounds: Bounds {
                r_0: utils::cast_range(args.r_0_bounds.clone())?,
                omega_0: utils::cast_range(args.omega_0_bounds.clone())?,
                a: utils::cast_range(args.a_bounds.clone())?,
                u_sun_standard: utils::cast_range(args.u_sun_standard_bounds.clone())?,
                v_sun_standard: utils::cast_range(args.v_sun_standard_bounds.clone())?,
                w_sun_standard: utils::cast_range(args.w_sun_standard_bounds.clone())?,
                sigma_r: utils::cast_range(args.sigma_r_bounds.clone())?,
                sigma_theta: utils::cast_range(args.sigma_theta_bounds.clone())?,
                sigma_z: utils::cast_range(args.sigma_z_bounds.clone())?,
            },
            ..Default::default()
        };
        // Extend it using the data from the files
        for path in &args.inputs {
            model
                .extend(path)
                .with_context(|| format!("Couldn't load the data from the file {path:?}"))?;
        }
        // Return the result
        Ok(model)
    }
}
