//! Construct records from the model

use crate::model::io::output::coords::{Record, Records};
use crate::model::Model;

use num::Float;

impl<F: Float> From<&Model<F>> for Records<F> {
    fn from(model: &Model<F>) -> Self {
        model
            .objects
            .iter()
            .map(|object| { Record::from(object) }).collect()
    }
}
