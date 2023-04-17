//! Model of the Galaxy

mod fit;
mod io;
mod objects;
mod params;
mod sample_description;

use crate::cli::Args;
use crate::utils;
use crate::Goal;
pub use fit::rotcurve::RotationCurve;
pub use objects::{Object, Objects};
pub use params::Params;

use core::fmt::{Debug, Display};
use core::iter::Sum;
use core::str::FromStr;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use argmin::core::ArgminFloat;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use finitediff::FiniteDiff;
use num::Float;
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
    /// Sample description
    sample_description: Option<String>,
    /// Output directory
    output_dir: PathBuf,
}

impl<F> Model<F> {
    /// Perform computations based on the goal
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn compute(&mut self) -> Result<()>
    where
        F: Float
            + Debug
            + Default
            + Display
            + Sync
            + Send
            + Sum
            + ArgminFloat
            + ArgminL2Norm<F>
            + ArgminSub<F, F>
            + ArgminAdd<F, F>
            + ArgminDot<F, F>
            + ArgminMul<F, F>
            + ArgminZeroLike
            + ArgminMul<Vec<F>, Vec<F>>,
        Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
        Vec<F>: ArgminSub<F, Vec<F>>,
        Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
        Vec<F>: ArgminAdd<F, Vec<F>>,
        Vec<F>: ArgminMul<F, Vec<F>>,
        Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
        Vec<F>: ArgminL1Norm<F>,
        Vec<F>: ArgminSignum,
        Vec<F>: ArgminMinMax,
        Vec<F>: ArgminDot<Vec<F>, F>,
        Vec<F>: ArgminL2Norm<F>,
        Vec<F>: FiniteDiff,
    {
        match self.goal {
            Goal::Objects => {
                // Perform per-object computations
                for object in &mut self.objects {
                    object.compute(&self.params);
                }
            }
            Goal::Fit => {
                // Try to fit the model
                self.try_fit_from()
                    .with_context(|| "Couldn't fit the model")?;
                // Perform per-object computations
                // with the optimized parameters
                for object in &mut self.objects {
                    object.compute(self.fit_params.as_ref().unwrap());
                }
                // Compute the rotation curve based on the fitted parameters
                self.compute_fit_rotcurve();
            }
        }
        Ok(())
    }
    /// Write the model data to files in the
    /// output directory based on the goal
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
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
                let params = &self.params;
                self.serialize_to_objects(dat_dir, bin_dir, "objects", params)
                    .with_context(|| "Couldn't write the objects to a file")?;
            }
            Goal::Fit => {
                let params = self.fit_params.as_ref().unwrap();
                self.serialize_to_objects(dat_dir, bin_dir, "fit_objects", params)
                    .with_context(|| "Couldn't write the objects to a file")?;
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
                r_0: utils::cast(args.r_0)?,
                omega_0: utils::cast(args.omega_0)?,
                a: utils::cast(args.a)?,
                u_sun: utils::cast(args.u_sun)?,
                theta_sun: utils::cast(args.theta_sun)?,
                w_sun: utils::cast(args.w_sun)?,
                sigma_r: utils::cast(args.sigma_r)?,
                sigma_theta: utils::cast(args.sigma_theta)?,
                sigma_z: utils::cast(args.sigma_z)?,
                alpha_ngp: utils::cast(args.alpha_ngp)?,
                delta_ngp: utils::cast(args.delta_ngp)?,
                l_ncp: utils::cast(args.l_ncp)?,
                k: utils::cast(args.k)?,
                u_sun_standard: utils::cast(args.u_sun_standard)?,
                v_sun_standard: utils::cast(args.v_sun_standard)?,
                w_sun_standard: utils::cast(args.w_sun_standard)?,
            },
            objects: Objects::<F>::default(),
            fit_params: None,
            fit_rotcurve: None,
            goal: args.goal,
            sample_description: None,
            output_dir: args.output_dir.clone(),
        };
        // Make sure the output directory exists
        fs::create_dir_all(&model.output_dir).with_context(|| {
            format!(
                "Couldn't create the output directory {:?}",
                &model.output_dir
            )
        })?;
        model.try_read_sample_description_from(&args.input)?;
        model
            .try_load_data_from(&args.input)
            .with_context(|| format!("Couldn't load the data from the file {:?}", args.input))?;
        // Return the result
        Ok(model)
    }
}
