//! Model of the Galaxy

mod fit;
mod io;
mod objects;
mod params;
mod sample_description;

use crate::cli::Args;
use crate::utils::{self, FiniteDiff};
pub use fit::{ProfileType, Profiles, RotationCurve};
pub use objects::{Object, Objects};
pub use params::{Params, N_MAX, PARAMS_N, PARAMS_NAMES};

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
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug, Clone, Default)]
pub struct Model<F> {
    /// Initial model parameters
    pub params: Params<F>,
    /// Data objects
    pub objects: Objects<F>,

    /// The degree of the polynomial of the rotation curve
    pub n: Option<usize>,
    /// The best value of the cost function
    pub best_cost: Option<F>,
    /// Fit of the model (parameters)
    pub fit_params: Option<Params<F>>,
    /// Fit of the model (rotation curve)
    pub fit_rotcurve: Option<RotationCurve<F>>,
    /// Conditional profiles
    pub conditional_profiles: Option<Profiles<F>>,
    /// Frozen profiles
    pub frozen_profiles: Option<Profiles<F>>,

    /// Sample description
    pub sample_description: Option<String>,
    /// Output directory
    pub output_dir: PathBuf,
}

impl<F> Model<F> {
    /// Perform per-object computations
    pub fn compute_objects(&mut self)
    where
        F: Float + Debug + Default,
    {
        for object in &mut self.objects {
            object.compute(&self.params);
        }
    }
    /// Fit the model with the specified degree of the polynomial
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn try_fit(&mut self, n: usize) -> Result<()>
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
        Vec<F>: FiniteDiff<F>,
    {
        // Try to fit the model
        self.try_fit_params(n)
            .with_context(|| "Couldn't fit the model")?;
        // Perform per-object computations
        // with the optimized parameters
        let fit_params = self.fit_params.as_ref().unwrap();
        for object in &mut self.objects {
            object.compute(fit_params);
        }
        // Compute the rotation curve based on the fitted parameters
        self.compute_fit_rotcurve();
        Ok(())
    }
    /// Write the objects data
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn write_objects_data(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        let params = &self.params;

        fs::create_dir_all(&self.output_dir).with_context(|| {
            format!("Couldn't create the output directory {:?}", self.output_dir)
        })?;

        self.serialize_to_objects("objects", params)
            .with_context(|| "Couldn't write the objects to a file")?;
        self.serialize_to_params()
            .with_context(|| "Couldn't write the initial parameters to a file")?;

        Ok(())
    }
    /// Create a model from the arguments with a specific degree
    /// of the polynomial of the rotation curve
    pub fn try_from(args: &Args, output_dir: PathBuf) -> Result<Self>
    where
        F: Float + Debug + Default + DeserializeOwned + FromStr,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        let mut model = Self {
            params: Params {
                r_0: utils::cast(args.r_0)?,
                omega_0: utils::cast(args.omega_0)?,
                a: utils::cast(args.a)?,
                u_sun: utils::cast(args.u_sun)?,
                v_sun: utils::cast(args.v_sun)?,
                w_sun: utils::cast(args.w_sun)?,
                sigma_r_g: utils::cast(args.sigma_r_g)?,
                sigma_theta: utils::cast(args.sigma_theta)?,
                sigma_z: utils::cast(args.sigma_z)?,
                alpha_ngp: utils::cast(args.alpha_ngp)?,
                delta_ngp: utils::cast(args.delta_ngp)?,
                theta_sun: utils::cast(args.theta_sun)?,
                l_ncp: utils::cast(args.l_ncp)?,
                k: utils::cast(args.k)?,
                u_sun_standard: utils::cast(args.u_sun_standard)?,
                v_sun_standard: utils::cast(args.v_sun_standard)?,
                w_sun_standard: utils::cast(args.w_sun_standard)?,
                ..Default::default()
            },
            objects: Objects::<F>::default(),
            output_dir,
            ..Default::default()
        };

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

        Ok(model)
    }
}
