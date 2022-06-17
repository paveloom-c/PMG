//! Output data records

use super::Record;
use crate::model::Model;

use itertools::izip;
use num::Float;

/// Output data records
pub(in crate::model) type Records<F> = Vec<Record<F>>;

impl<F: Float> From<&Model<F>> for Records<F> {
    fn from(model: &Model<F>) -> Self {
        izip!(
            model.names.iter().cloned(),
            &model.coords.x,
            &model.coords.y,
            &model.coords.z,
            model.obj_types.iter().cloned(),
        )
        .fold(Self::new(), |mut acc, (name, &x, &y, &z, obj_type)| {
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
