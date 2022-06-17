//! Construct records from the model

use crate::model::io::output::coords::{Record, Records};
use crate::model::Model;

use itertools::izip;
use num::Float;

impl<F: Float> From<&Model<F>> for Records<F> {
    fn from(model: &Model<F>) -> Self {
        izip!(
            model.names.iter().cloned(),
            &model.coords.x,
            &model.coords.y,
            &model.coords.z,
            model.obj_types.iter().cloned(),
        )
        .fold(Self::default(), |mut acc, (name, &x, &y, &z, obj_type)| {
            acc.push(Record {
                name,
                x,
                y,
                z,
                obj_type,
            });
            acc
        })
    }
}
