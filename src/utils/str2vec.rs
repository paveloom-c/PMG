//! Parse a string for a vector of floats

use core::fmt::Debug;
use core::str::FromStr;
use std::error::Error;

use anyhow::{Context, Result};
use num::Float;

/// Parse a string for a vector of floats, splitting by colons
pub fn str2vec<F>(str: &str) -> Result<Vec<F>>
where
    F: Float + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    // Split a string by colons
    str.split(':')
        // For each slice
        .map(|s| {
            // Parse the slice as a floating point number
            s.parse::<F>()
                .with_context(|| format!("Couldn't parse the string slice {s:?}"))
        })
        // Collect the values in an array
        .collect()
}

#[cfg(test)]
use anyhow::ensure;

#[test]
fn test() -> Result<()> {
    let a = vec![-5., 35., 5.108];
    let b = str2vec("-05:35:05.108").with_context(|| "Couldn't parse the string")?;
    ensure!(a == b, "The result should be the same: {a:?} vs. {b:?}");
    Ok(())
}
