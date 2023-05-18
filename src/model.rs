//! Model of the Galaxy

extern crate alloc;

pub mod fit;
mod io;
mod objects;
mod params;
mod sample_description;

use crate::cli::Args;
use crate::utils;
pub use fit::{ProfileType, RotationCurve, Triple, Triples};
pub use objects::{Object, Objects};
pub use params::{Params, PARAMS_N, PARAMS_NAMES};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::str::FromStr;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug, Clone, Default)]
pub struct Model<F> {
    /// Initial model parameters
    pub params: Params<F>,
    /// Data objects
    pub objects: Objects<F>,

    /// Disable the inner optimization?
    pub disable_inner: bool,
    /// Tolerance of the L-BFGS algorithm
    pub lbfgs_tolerance: F,
    /// The degree of the polynomial of the rotation curve
    pub n: Option<usize>,
    /// Number of the objects after the L' = 3 run
    pub l_stroke_3_n: Option<usize>,
    /// Number of the objects after the L' = 1 run
    pub l_stroke_1_n: Option<usize>,
    /// The best value of the cost function
    pub best_cost: Option<F>,
    /// Fit of the model (parameters)
    pub fit_params: Option<Params<F>>,
    /// Fit of the model (rotation curve)
    pub fit_rotcurve: Option<RotationCurve<F>>,
    /// Triples
    pub triples: Rc<RefCell<Vec<Triples<F>>>>,

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
        for object in self.objects.borrow_mut().iter_mut() {
            object.compute(&self.params);
        }
    }
    /// Run the post-fit computations
    #[allow(clippy::unwrap_used)]
    pub fn post_fit(&mut self)
    where
        F: Float + Debug + Default,
    {
        self.compute_fit_rotcurve();

        let fit_params = self.fit_params.as_ref().unwrap();
        for object in self.objects.borrow_mut().iter_mut() {
            object.compute(fit_params);
        }
    }
    /// Write the objects data
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn write_objects_data(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        let params = &self.params;

        self.serialize_to_objects("objects", params)
            .with_context(|| "Couldn't write the objects to a file")?;
        self.serialize_to_params()
            .with_context(|| "Couldn't write the initial parameters to a file")?;

        Ok(())
    }
    /// Write the fit data
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn write_fit_data(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        let fit_params = &self.fit_params.as_ref().unwrap();

        self.serialize_to_objects("fit_objects", fit_params)
            .with_context(|| "Couldn't write the objects to a file")?;
        self.serialize_to_fit_params()
            .with_context(|| "Couldn't write the fitted parameters to a file")?;
        self.serialize_to_fit_rotcurve()
            .with_context(|| "Couldn't write the fitted rotation curve to a file")?;

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
            output_dir,
            disable_inner: args.disable_inner,
            lbfgs_tolerance: utils::cast(args.lbfgs_tolerance)?,
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

        let triple = vec![Triple::<F>::default(); 4];
        model.triples = Rc::new(RefCell::new(vec![triple; model.objects.borrow().len()]));

        Ok(model)
    }
}
