//! Convert a degrees-minutes-seconds angle to radians

use std::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Convert a degrees-minutes-seconds angle to radians
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn dms2rad<F: Float + Debug>(degrees: F, minutes: F, seconds: F) -> F {
    (degrees.signum() * (F::abs(degrees) + minutes / 60. + seconds / 3600.)).to_radians()
}

#[cfg(test)]
use anyhow::{ensure, Result};

#[test]
fn test_1() -> Result<()> {
    let a = 3.97.to_radians();
    let b = dms2rad(3., 58., 12.);
    ensure!(
        (a - b).abs() < f64::epsilon(),
        "The result should be the same: {a:?} vs. {b:?}"
    );
    Ok(())
}

#[test]
fn test_2() -> Result<()> {
    let a = -0.087_584_107_675_442_64;
    let b = dms2rad(-5., 1., 5.519);
    ensure!(
        (a - b).abs() < f64::epsilon(),
        "The result should be the same: {a:?} vs. {b:?}"
    );
    Ok(())
}
