//! Model of the Galaxy

mod fit_params;
mod fit_rotcurve;
mod io;
mod objects;
mod params;

use crate::cli::Args;
use crate::utils;
use crate::Goal;
pub use fit_rotcurve::RotationCurve;
pub use objects::{Measurement, Object, Objects};
pub use params::Params;

use core::fmt::{Debug, Display};
use core::str::FromStr;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use num::Float;
use rand::distributions::uniform::SampleUniform;
use rand_distr::Distribution;
use rand_distr::StandardNormal;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F> {
    /// Initial model parameters
    params: Params<F>,
    /// Data objects
    objects: Objects<F>,
    /// Fit of the model (parameters)
    fit_params: Option<Params<F>>,
    /// Fit of the model (rotation curve)
    fit_rotcurve: Option<RotationCurve<F>>,
    /// Computation goal
    goal: Goal,
    /// Output directory
    output_dir: PathBuf,
}

impl<F> Model<F> {
    /// Perform computations based on the goal
    pub fn compute(&mut self) -> Result<()>
    where
        F: Float + Debug + Default + Display + SampleUniform + Sync + Send,
        StandardNormal: Distribution<F>,
    {
        match self.goal {
            Goal::Objects => {
                // Perform per-object computations
                self.objects.compute(&self.params);
            }
            Goal::Fit => {
                // Try to fit the model
                self.try_fit_from()
                    .with_context(|| "Couldn't fit the model")?;
                // Compute the rotation curve based on the fitted parameters
                self.compute_fit_rotcurve();
            }
        }
        Ok(())
    }
    /// Extend the model by parsing and appending the data
    /// from the file, doing conversions where necessary
    fn extend(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Debug + Default + DeserializeOwned + FromStr,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the data from the file
        let objects = Objects::try_from(path).with_context(|| "Couldn't parse the data")?;
        // Extend the model
        self.objects.extend(objects);
        Ok(())
    }
    /// Write the model data to files in the
    /// output directory based on the goal
    pub fn write(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        // Make sure the output directories exist
        let dat_dir = &self.output_dir.join("dat");
        let bin_dir = &self.output_dir.join("bin");
        fs::create_dir_all(dat_dir)
            .with_context(|| format!("Couldn't create the output directory {dat_dir:?}"))?;
        fs::create_dir_all(bin_dir)
            .with_context(|| format!("Couldn't create the output directory {bin_dir:?}"))?;
        // Serialize the data
        match self.goal {
            Goal::Objects => {
                self.serialize_to_objects(dat_dir, bin_dir)
                    .with_context(|| "Couldn't write the objects to a file")?;
            }
            Goal::Fit => {
                self.serialize_to_fit_params(dat_dir, bin_dir)
                    .with_context(|| "Couldn't write the fitted parameters to a file")?;
                self.serialize_to_fit_rotcurve(dat_dir, bin_dir)
                    .with_context(|| "Couldn't write the fitted rotation curve to a file")?;
            }
        }
        Ok(())
    }
}

impl<F> TryFrom<&Args> for Model<F>
where
    F: Float + Debug + Default + DeserializeOwned + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
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
            objects: Objects::<F>::default(),
            fit_params: None,
            fit_rotcurve: None,
            goal: args.goal,
            output_dir: args.output_dir.clone(),
        };
        // Make sure the output directory exists
        fs::create_dir_all(&model.output_dir).with_context(|| {
            format!(
                "Couldn't create the output directory {:?}",
                &model.output_dir
            )
        })?;
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
