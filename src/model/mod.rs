//! Model of the Galaxy

mod coordinates;
mod io;

use crate::model::io::input::Data;
use crate::model::io::output::coords::Record;
use coordinates::Galactic;

use std::error::Error;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use csv::WriterBuilder;
use itertools::izip;
use num::Float;
use serde::{de::DeserializeOwned, Serialize};

/// Type for names of the objects
pub type Names = Vec<String>;

/// Type for types of the objects
pub type ObjTypes = Vec<String>;

/// Model of the Galaxy
#[derive(Debug)]
pub struct Model<F: Float> {
    /// Names of the objects
    names: Names,
    /// Coordinates (in the Galactic heliocentric Cartesian system)
    coords: Galactic<F>,
    /// Types of the objects
    obj_types: ObjTypes,
}

impl<F: Float> Model<F> {
    /// Create a new instance of the struct
    fn new() -> Self {
        Self {
            names: Names::new(),
            coords: Galactic::new(),
            obj_types: ObjTypes::new(),
        }
    }
    /// Extend the model by parsing and appending the data
    /// from the file, doing conversions where necessary
    ///
    /// # Errors
    ///
    /// Will return `Err` if deserializing data from the file wasn't successful
    pub fn extend(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Debug + FromStr + DeserializeOwned,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Parse the data from the file
        let data = Data::try_from(path).with_context(|| "Couldn't parse the data")?;
        // Extend the model
        self.names.extend(data.names);
        self.coords.extend(Galactic::from(data.coords));
        self.obj_types.extend(data.obj_types);
        Ok(())
    }
    /// Write the model data to files inside the directory
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the path isn't a directory;
    /// - couldn't write to the file;
    /// - couldn't serialize a record
    pub fn write_to(&self, dir: &Path) -> Result<()>
    where
        F: Serialize,
    {
        // Create a writer for the coordinates
        let path = &dir.join("coords.dat");
        let mut wtr = WriterBuilder::new()
            .delimiter(b' ')
            .from_path(path)
            .with_context(|| format!("Couldn't write to the file {path:?}"))?;
        // For each object
        for (name, x, y, z, obj_type) in izip!(
            &self.names,
            &self.coords.x,
            &self.coords.y,
            &self.coords.z,
            &self.obj_types
        ) {
            // Serialize and write a record
            wtr.serialize(Record {
                name: name.clone(),
                x: *x,
                y: *y,
                z: *z,
                obj_type: obj_type.clone(),
            })
            .with_context(|| {
                format!("Couldn't write serialize a record while writing to {path:?}")
            })?;
        }
        Ok(())
    }
}

impl<F> TryFrom<Vec<PathBuf>> for Model<F>
where
    F: Float + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(paths: Vec<PathBuf>) -> Result<Self> {
        // Initialize an empty model
        let mut model = Model::new();
        // Extend it using the data from the files
        for path in paths {
            model
                .extend(&path)
                .with_context(|| format!("Couldn't load the data from the file {path:?}"))?;
        }
        // Return the result
        Ok(model)
    }
}
