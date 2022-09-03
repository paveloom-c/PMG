//! Model of the Galaxy

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
    consts: Consts,
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
                alpha_ngp: args.alpha_ngp,
                delta_ngp: args.delta_ngp,
                k: args.k,
                l_ncp: args.l_ncp,
                r_0_1: args.r_0_1,
                r_0_2: args.r_0_2,
                theta_sun: args.theta_sun,
                u_sun: args.u_sun,
                u_sun_standard: args.u_sun_standard,
                v_sun_standard: args.v_sun_standard,
                w_sun_standard: args.w_sun_standard,
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
