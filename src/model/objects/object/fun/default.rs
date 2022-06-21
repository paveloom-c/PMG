//! Get default value

use super::super::Object;

use num::Float;

impl<F: Float> Default for Object<F> {
    fn default() -> Self {
        Self {
            name: Option::default(),
            equatorial_s: Option::default(),
            galactic_s: Option::default(),
            galactic_c: Option::default(),
            obj_type: Option::default(),
            source: Option::default(),
        }
    }
}
