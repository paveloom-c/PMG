//! Model of the Galaxy

mod fit;
mod io;
mod objects;
mod params;
mod sample_description;

use crate::cli::Args;
use crate::utils;
use crate::{Goal, Task};
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
    /// Computation task
    task: Task,
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
        match self.task.goal {
            Goal::Objects => {
                // Perform per-object computations
                for object in &mut self.objects {
                    object.compute(&self.params);
                }
            }
            Goal::Fit => {
                // Try to fit the model
                self.try_fit_params()
                    .with_context(|| "Couldn't fit the model")?;
                // Try to define the confidence intervals if requested
                if self.task.with_errors {
                    self.try_fit_errors()
                        .with_context(|| "Couldn't define the confidence intervals")?;
                }
                // Perform per-object computations
                // with the optimized parameters
                let fit_params = self.fit_params.as_ref().unwrap();
                for object in &mut self.objects {
                    object.compute(fit_params);
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
        let output_dir = &self.output_dir;
        // Make sure the output directory exists
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Couldn't create the output directory {output_dir:?}"))?;
        // Serialize the data
        match self.task.goal {
            Goal::Objects => {
                let params = &self.params;
                self.serialize_to_objects(output_dir, "objects", params)
                    .with_context(|| "Couldn't write the objects to a file")?;
            }
            Goal::Fit => {
                let fit_params = self.fit_params.as_ref().unwrap();
                self.serialize_to_objects(output_dir, "fit_objects", fit_params)
                    .with_context(|| "Couldn't write the objects to a file")?;
                self.serialize_to_fit_params(output_dir)
                    .with_context(|| "Couldn't write the fitted parameters to a file")?;
                self.serialize_to_fit_rotcurve(output_dir)
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
                r_0_ep: None,
                r_0_em: None,
                omega_0: utils::cast(args.omega_0)?,
                omega_0_ep: None,
                omega_0_em: None,
                a: utils::cast(args.a)?,
                a_ep: None,
                a_em: None,
                u_sun: utils::cast(args.u_sun)?,
                u_sun_ep: None,
                u_sun_em: None,
                v_sun: utils::cast(args.v_sun)?,
                v_sun_ep: None,
                v_sun_em: None,
                w_sun: utils::cast(args.w_sun)?,
                w_sun_ep: None,
                w_sun_em: None,
                sigma_r_g: utils::cast(args.sigma_r_g)?,
                sigma_r_g_ep: None,
                sigma_r_g_em: None,
                sigma_theta: utils::cast(args.sigma_theta)?,
                sigma_theta_ep: None,
                sigma_theta_em: None,
                sigma_z: utils::cast(args.sigma_z)?,
                sigma_z_ep: None,
                sigma_z_em: None,
                alpha_ngp: utils::cast(args.alpha_ngp)?,
                delta_ngp: utils::cast(args.delta_ngp)?,
                theta_0: F::zero(),
                theta_1: F::zero(),
                theta_sun: utils::cast(args.theta_sun)?,
                l_ncp: utils::cast(args.l_ncp)?,
                k: utils::cast(args.k)?,
                u_sun_standard: utils::cast(args.u_sun_standard)?,
                v_sun_standard: utils::cast(args.v_sun_standard)?,
                w_sun_standard: utils::cast(args.w_sun_standard)?,
            },
            objects: Objects::<F>::default(),
            fit_params: None,
            fit_rotcurve: None,
            task: Task {
                goal: args.goal,
                with_errors: args.with_errors,
            },
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
