//! Convert equatorial coordinates to spherical Galactic coordinates

use crate::utils::{dms2rad, hms2rad};

use lazy_static::lazy_static;
use num::Float;

lazy_static! {
    /// The right ascension of the north galactic pole (radians)
    ///
    /// Source: Reid et al. (2009)
    static ref ALPHA_NGP: f64 = hms2rad(12., 51., 26.2817);
    /// The declination of the north galactic pole (radians)
    ///
    /// Source: Reid et al. (2009)
    static ref DELTA_NGP: f64 = dms2rad(27., 7., 42.013);
    /// The longitude of the north celestial pole (radians)
    ///
    /// Source: Reid et al. (2009)
    static ref L_NCP: f64 = 122.932.to_radians();
}

/// The right ascension of the north galactic pole (radians)
#[allow(clippy::unwrap_used)]
#[inline]
fn alpha_ngp<F: Float>() -> F {
    F::from(*ALPHA_NGP).unwrap()
}

/// The declination of the north galactic pole (radians)
#[allow(clippy::unwrap_used)]
#[inline]
fn delta_ngp<F: Float>() -> F {
    F::from(*DELTA_NGP).unwrap()
}

/// The longitude of the north celestial pole (radians)
#[allow(clippy::unwrap_used)]
#[inline]
fn l_ncp<F: Float>() -> F {
    F::from(*L_NCP).unwrap()
}

/// Convert the equatorial coordinates to spherical heliocentric Galactic coordinates
///
/// Angles must be in radians, then radians returned.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/Galactic_coordinate_system#Conversion_between_equatorial_and_galactic_coordinates)
pub fn to_spherical<F: Float>(alpha: F, delta: F) -> (F, F) {
    let phi = F::atan2(
        F::cos(delta) * F::sin(alpha - alpha_ngp()),
        F::cos(delta_ngp()) * F::sin(delta)
            - F::sin(delta_ngp()) * F::cos(delta) * F::cos(alpha - alpha_ngp()),
    );
    let l = l_ncp::<F>() - phi;
    let b = F::asin(
        F::sin(delta_ngp()) * F::sin(delta)
            + F::cos(delta_ngp()) * F::cos(delta) * F::cos(alpha - alpha_ngp()),
    );
    (l, b)
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use anyhow::{ensure, Context, Result};
        use csv::ReaderBuilder;
        use itertools::izip;
        use serde::Deserialize;
        use std::path::Path;

        /// Data record
        #[derive(Deserialize)]
        struct DataRecord<F: Float> {
            /// Name of the object
            #[allow(dead_code)]
            name: String,
            /// Right ascension
            alpha: F,
            /// Declination
            delta: F,
            /// Longitude
            l: F,
            /// Latitude
            b: F,
            /// Parallax
            #[allow(dead_code)]
            par: F,
            /// Uncertainty in `par`
            #[allow(dead_code)]
            e_par: F,
        }
    } else {}
}

#[test]
#[allow(clippy::unwrap_used)]
fn test() -> Result<()> {
    // Define the path to the data files
    let current_file = Path::new(file!());
    let tests_path = current_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests");
    let data_path = tests_path.join("data.dat");
    // Create two CSV readers
    let mut data_rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .comment(Some(b'#'))
        .from_path(&data_path)
        .with_context(|| format!("Couldn't create a reader for {data_path:?}"))?;
    // Prepare a counter
    let mut i = 1;
    // For each pair of records
    for data_record in data_rdr.deserialize() {
        // Deserialize the data
        let data: DataRecord<f64> = data_record
            .with_context(|| format!("Couldn't deserialize a record from {data_path:?}"))?;
        // Compare the data
        let a = (data.l.to_radians(), data.b.to_radians());
        let b = to_spherical(data.alpha.to_radians(), data.delta.to_radians());
        ensure!(
            izip!([a.0, a.1], [b.0, b.1])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon())),
            "Spherical coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
