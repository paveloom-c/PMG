//! Convert from equatorial coordinates to
//! Galactic heliocentric Cartesian coordinates

use super::{to_cartesian, to_spherical};
use std::fmt::Debug;

use num::Float;

/// Convert from equatorial coordinates to
/// Galactic heliocentric Cartesian coordinates
///
/// Angles must be in radians.
#[allow(clippy::many_single_char_names)]
pub fn to_galactic<F>(alpha: F, delta: F, par: F) -> (F, F, F)
where
    F: Float + Debug,
{
    // Convert to the spherical coordinate system
    let (l, b) = to_spherical(alpha, delta);
    // Convert to the Cartesian coordinate system
    let (x, y, z) = to_cartesian(l, b, par);
    (x, y, z)
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
        // Compare the data
        let a = (coords.x, coords.y, coords.z);
        let b = to_galactic(data.alpha.to_radians(), data.delta.to_radians(), data.par);
        ensure!(
            izip!([a.0, a.1, a.2], [b.0, b.1, b.2])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon() * 10.)),
            "Cartesian coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
