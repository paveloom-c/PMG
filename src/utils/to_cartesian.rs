//! Convert heliocentric Galactic coordinates from
//! the spherical to the Cartesian coordinate system

use num::Float;
use numeric_literals::replace_float_literals;

/// Convert the heliocentric Galactic coordinates
/// from the spherical to the Cartesian coordinate system
///
/// Angles must be in radians.
#[allow(clippy::many_single_char_names)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn to_cartesian<F: Float>(l: F, b: F, par: F) -> (F, F, F) {
    // Compute the distance in `kpc`
    let d = 1. / par;
    // Convert to the Galactic heliocentric Cartesian system
    let x = d * F::cos(b) * F::cos(l);
    let y = d * F::cos(b) * F::sin(l);
    let z = d * F::sin(b);
    (x, y, z)
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use super::to_spherical;

        use std::path::Path;

        use anyhow::{ensure, Context, Result};
        use csv::ReaderBuilder;
        use itertools::izip;
        use serde::Deserialize;

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
            #[allow(dead_code)]
            l: F,
            /// Latitude
            #[allow(dead_code)]
            b: F,
            /// Parallax
            #[allow(dead_code)]
            par: F,
            /// Uncertainty in `par`
            #[allow(dead_code)]
            e_par: F,
        }


        /// Coordinates record
        #[derive(Deserialize)]
        struct CoordsRecord<F: Float> {
            /// X coordinate
            x: F,
            /// Y coordinate
            y: F,
            /// Z coordinate
            z: F,
            /// Parallax
            #[allow(dead_code)]
            par: F,
            /// Uncertainty in `par`
            #[allow(dead_code)]
            e_par: F,
            /// Name of the object
            #[allow(dead_code)]
            name: String,
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
    let coords_path = tests_path.join("coords.dat");
    // Create two CSV readers
    let mut data_rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .comment(Some(b'#'))
        .from_path(&data_path)
        .with_context(|| format!("Couldn't create a reader for {data_path:?}"))?;
    let mut coords_rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .comment(Some(b'#'))
        .from_path(&coords_path)
        .with_context(|| format!("Couldn't create a reader for {coords_path:?}"))?;
    // Prepare a counter
    let mut i = 1;
    // For each pair of records
    for (data_record, coords_record) in izip!(data_rdr.deserialize(), coords_rdr.deserialize()) {
        // Deserialize the data
        let data: DataRecord<f64> = data_record
            .with_context(|| format!("Couldn't deserialize a record from {data_path:?}"))?;
        let coords: CoordsRecord<f64> = coords_record
            .with_context(|| format!("Couldn't deserialize a record from {coords_path:?}"))?;
        // Compute the Galactic spherical coordinates
        let (gs_l, gs_b) = to_spherical(data.alpha.to_radians(), data.delta.to_radians());
        // Compare the data
        let a = (coords.x, coords.y, coords.z);
        let b = to_cartesian(gs_l, gs_b, data.par);
        ensure!(
            izip!([a.0, a.1, a.2], [b.0, b.1, b.2])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon() * 10.)),
            "Cartesian coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
