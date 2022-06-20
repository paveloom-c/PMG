//! Construct a record from the object

use super::super::Record;
use crate::model::Object;

use num::Float;

impl<F: Float> From<&Object<F>> for Record<F> {
    fn from(object: &Object<F>) -> Self {
        Self {
            name: object.name.clone(),
            x: object.galactic_c.x,
            y: object.galactic_c.y,
            z: object.galactic_c.z,
            obj_type: object.obj_type.clone(),
            source: object.source.clone(),
        }
    }
}
