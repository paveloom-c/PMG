//! Convert equatorial coordinates to spherical Galactic coordinates

use crate::model::Consts;

use std::fmt::Debug;

use num::Float;

/// Convert the equatorial coordinates to spherical heliocentric Galactic coordinates
///
/// Angles must be in radians, then radians returned.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/Galactic_coordinate_system#Conversion_between_equatorial_and_galactic_coordinates)
pub fn to_spherical<F: Float + Debug>(alpha: F, delta: F, consts: &Consts) -> (F, F) {
    // Get the constants
    let alpha_ngp: F = consts.alpha_ngp();
    let delta_ngp: F = consts.delta_ngp();
    let l_ncp: F = consts.l_ncp();
    // Compute the angles
    let phi = F::atan2(
        F::cos(delta) * F::sin(alpha - alpha_ngp),
        F::cos(delta_ngp) * F::sin(delta)
            - F::sin(delta_ngp) * F::cos(delta) * F::cos(alpha - alpha_ngp),
    );
    let l = l_ncp - phi;
    let b = F::asin(
        F::sin(delta_ngp) * F::sin(delta)
            + F::cos(delta_ngp) * F::cos(delta) * F::cos(alpha - alpha_ngp),
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
        struct DataRecord<F: Float + Debug> {
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
    // Initialize a new constants struct
    let consts = Consts {
        alpha_ngp: 3.366_033_392_377_493,
        delta_ngp: 0.473_478_800_270_973_6,
        l_ncp: 2.145_568_156_061_669_3,
        ..Default::default()
    };
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
        let b = to_spherical(data.alpha.to_radians(), data.delta.to_radians(), &consts);
        ensure!(
            izip!([a.0, a.1], [b.0, b.1])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon())),
            "Spherical coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
