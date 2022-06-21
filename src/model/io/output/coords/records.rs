//! Output data records

mod from;

use super::Record;

/// Output data records
pub(in crate::model) type Records<'a, F> = Vec<Record<'a, F>>;
