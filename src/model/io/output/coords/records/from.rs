//! Construct records from the model

use crate::model::io::output::coords::{Record, Records};
use crate::model::Model;

use anyhow::{Context, Result};
use num::Float;

impl<'a, F: Float> TryFrom<&'a Model<F>> for Records<'a, F> {
    type Error = anyhow::Error;

    fn try_from(model: &'a Model<F>) -> Result<Self> {
        model
            .objects
            .iter()
            .map(|object| {
                Record::try_from(object)
                    .with_context(|| "Couldn't construct a record from the object")
            })
            .collect()
    }
}
