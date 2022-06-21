//! Construct a record from the object

use super::super::Record;
use crate::model::Object;

use anyhow::Result;
use num::Float;

impl<'a, F: Float> TryFrom<&'a Object<F>> for Record<'a, F> {
    type Error = anyhow::Error;

    fn try_from(object: &'a Object<F>) -> Result<Self> {
        Ok(Self {
            name: object.name()?,
            l: object.galactic_s()?.l.to_degrees(),
            b: object.galactic_s()?.b.to_degrees(),
            x: object.galactic_c()?.x,
            y: object.galactic_c()?.y,
            z: object.galactic_c()?.z,
            obj_type: object.obj_type()?,
            source: object.source()?,
        })
    }
}
