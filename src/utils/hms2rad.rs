//! Convert an hours-minutes-seconds angle to radians

use core::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// Convert an hours-minutes-seconds angle to radians
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn hms2rad<F: Float + Debug>(hours: F, minutes: F, seconds: F) -> F {
    (hours * 15. + minutes / 4. + seconds / 240.).to_radians()
}

#[cfg(test)]
use anyhow::{ensure, Result};

#[test]
fn test() -> Result<()> {
    let a = super::dms2rad(187., 13., 57.750);
    let b = hms2rad(12., 28., 55.85);
    ensure!(
        (a - b).abs() < f64::epsilon(),
        "The result should be the same: {a:?} vs. {b:?}"
    );
    Ok(())
}
