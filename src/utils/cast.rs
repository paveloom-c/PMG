//! Try to cast the number to a generic floating-point number

#![allow(clippy::module_name_repetitions)]

use anyhow::{anyhow, Result};
use num::{Float, ToPrimitive};

use core::fmt::Debug;

/// Try to cast the number to a generic floating-point number
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn cast<X, F>(x: X) -> Result<F>
where
    X: ToPrimitive,
    F: Float + Debug,
{
    F::from(x).ok_or_else(|| anyhow!("Couldn't cast a value to a floating-point number"))
}
