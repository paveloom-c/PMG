//! Convert a degrees-minutes-seconds angle to radians

use num::Float;
use numeric_literals::replace_float_literals;

/// Convert a degrees-minutes-seconds angle to radians
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn dms2rad<F: Float>(degrees: F, minutes: F, seconds: F) -> F {
    (degrees.signum() * F::abs(degrees + minutes / 60. + seconds / 3600.)).to_radians()
}

#[cfg(test)]
use anyhow::{ensure, Result};

#[test]
fn test() -> Result<()> {
    let a = 3.97.to_radians();
    let b = dms2rad(3., 58., 12.);
    ensure!(
        (a - b) < f64::epsilon(),
        "The result should be the same: {a:?} vs. {b:?}"
    );
    Ok(())
}
