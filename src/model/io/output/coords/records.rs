//! Output data records

mod fun;

use super::Record;

/// Output data records
pub(in crate::model) type Records<F> = Vec<Record<F>>;
