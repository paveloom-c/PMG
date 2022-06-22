//! Write the model data to files inside the directory

use super::super::Model;
use crate::model::io::output;
use crate::Goal;

use std::fs::create_dir_all;
use std::path::Path;

use anyhow::{bail, Context, Result};
use indoc::indoc;
use num::Float;
use serde::Serialize;

/// Header of the `coords.dat` file
const COORDS_CSV_HEADER: &str = indoc! {"
    # Galactic coordinates of the objects
    #
    # Descriptions:
    #
    # name: Name of the object
    # l: Longitude [deg]
    # b: Latitude [deg]
    # x: X coordinate [kpc]
    # y: Y coordinate [kpc]
    # z: Z coordinate [kpc]
    # r_h: Heliocentric distance [kpc]
    # r_g: Galactocentric distance [kpc]
    # obj_type: Type of the object
    # source: Source of the data
    #\n
"};

impl<F: Float> Model<F> {
    /// Write the model data to files inside the directory
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
        match goals[..] {
            [Goal::Coords] => Model::<F>::serialize_to(
                dat_dir,
                bin_dir,
                "coords",
                COORDS_CSV_HEADER,
                output::coords::Records::try_from(self)
                    .with_context(|| "Couldn't construct records from objects")?,
            )
            .with_context(|| "Couldn't write the Galactic coordinates to a file")?,
            _ => bail!("This combination of goals wasn't expected."),
        };
        Ok(())
    }
}
