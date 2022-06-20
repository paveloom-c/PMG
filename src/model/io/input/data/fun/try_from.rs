//! Get data from the file with the given path

use crate::model::coordinates::Equatorial;
use crate::model::io::input::{Data, Record};
use crate::model::{Names, ObjTypes, Sources};

use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use num::Float;
use serde::de::DeserializeOwned;

impl<F> TryFrom<&Path> for Data<F>
where
    F: Float + Debug + FromStr + DeserializeOwned,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        // Create a reader
        let mut rdr = ReaderBuilder::default()
            .delimiter(b' ')
            .comment(Some(b'#'))
            .from_path(path)
            .with_context(|| format!("Couldn't read from the file {path:?}"))?;
        // Prepare storage
        let mut names = Names::default();
        let mut coords = Equatorial::<F>::default();
        let mut obj_types = ObjTypes::default();
        let mut sources = Sources::default();
        // For each record in the reader
        for result in rdr.deserialize() {
            // Try to deserialize the record
            let record: Record<F> = result.with_context(|| "Couldn't deserialize the record")?;
            // Push the data to the according storage
            names.push(record.name.clone());
            coords.push(&record)?;
            obj_types.push(record.obj_type.clone());
            sources.push(record.source.clone());
        }
        Ok(Data {
            names,
            coords,
            obj_types,
            sources,
        })
    }
}
