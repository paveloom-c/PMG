//! Model of the Galaxy

mod comp;
mod consts;
mod io;
mod objects;

use crate::cli::Args;
use crate::Goal;
pub use consts::Consts;
use objects::{Object, Objects};

use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::create_dir_all;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Model of the Galaxy
#[derive(Debug, Default)]
pub struct Model<F: Float + Default + Display + Debug> {
    /// Constants
    consts: Consts<F>,
    /// Data objects
    objects: Objects<F>,
}

impl<F: Float + Default + Display + Debug> Model<F> {
    /// Perform computations based on goals
    pub fn compute(&mut self, goals: &[Goal]) -> Result<()> {
        self.objects.compute(goals, &self.consts)?;
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
        create_dir_all(dat_dir)
            .with_context(|| format!("Couldn't create the output directory {dat_dir:?}"))?;
        create_dir_all(bin_dir)
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
        Ok(())
    }
}

impl<F> TryFrom<&Args> for Model<F>
where
    F: Float + Default + Display + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(args: &Args) -> Result<Self> {
        // Initialize an empty model
        let mut model = Self {
            consts: Consts {
                alpha_ngp: F::from(args.alpha_ngp).unwrap(),
                delta_ngp: F::from(args.delta_ngp).unwrap(),
                k: F::from(args.k).unwrap(),
                l_ncp: F::from(args.l_ncp).unwrap(),
                r_0_1: F::from(args.r_0_1).unwrap(),
                r_0_2: F::from(args.r_0_2).unwrap(),
                theta_sun: F::from(args.theta_sun).unwrap(),
                u_sun: F::from(args.u_sun).unwrap(),
                u_sun_standard: F::from(args.u_sun_standard).unwrap(),
                v_sun_standard: F::from(args.v_sun_standard).unwrap(),
                w_sun_standard: F::from(args.w_sun_standard).unwrap(),
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
