//! Galactic heliocentric Cartesian coordinates

use super::{Measurement, Object};

use core::fmt::{Debug, Display};

use num::{traits::FloatConst, Float};

#[allow(clippy::many_single_char_names)]
impl<F> Object<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Compute the coordinates with a specific galactocentric distance
    #[allow(clippy::unwrap_used)]
    fn compute_x_y_z_with(&self, r_h: F) -> (F, F, F) {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let x = r_h * F::cos(b) * F::cos(l);
        let y = r_h * F::cos(b) * F::sin(l);
        let z = r_h * F::sin(b);
        (x, y, z)
    }
    /// Convert the galactic heliocentric spherical coordinates
    /// to Galactic heliocentric Cartesian coordinates
    #[allow(clippy::unwrap_used)]
    pub fn compute_x_y_z(&mut self) {
        // Unpack the data
        let r_h = self.r_h.as_ref().unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = self.compute_x_y_z_with(r_h.v);
        let (x_u, y_u, z_u) = self.compute_x_y_z_with(r_h.v_u);
        let (x_l, y_l, z_l) = self.compute_x_y_z_with(r_h.v_l);
        self.x = Some(Measurement {
            v: x,
            v_u: x_u,
            v_l: x_l,
            e_p: x_u - x,
            e_m: x - x_l,
        });
        self.y = Some(Measurement {
            v: y,
            v_u: y_u,
            v_l: y_l,
            e_p: y_u - y,
            e_m: y - y_l,
        });
        self.z = Some(Measurement {
            v: z,
            v_u: z_u,
            v_l: z_l,
            e_p: z_u - z,
            e_m: z - z_l,
        });
    }
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::model::{Params};

        use std::path::Path;

        use anyhow::{ensure, Context, Result};
        use csv::ReaderBuilder;
        use itertools::izip;
        use serde::Deserialize;

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
            #[allow(dead_code)]
            l: F,
            /// Latitude
            #[allow(dead_code)]
            b: F,
            /// Parallax
            par: F,
            /// Uncertainty in `par`
            #[allow(dead_code)]
            e_par: F,
        }


        /// Coordinates record
        #[derive(Deserialize)]
        struct CoordsRecord<F: Float + Debug> {
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
#[allow(clippy::many_single_char_names)]
#[allow(clippy::unwrap_used)]
fn test() -> Result<()> {
    // Initialize a new parameters struct
    let params = Params {
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
        let mut object = Object {
            alpha: Some(data.alpha.to_radians()),
            delta: Some(data.delta.to_radians()),
            ..Default::default()
        };
        object.par = Some(Measurement {
            v: data.par,
            ..Default::default()
        });
        object.compute_l_b(&params);
        object.compute_r_h();
        object.compute_x_y_z();
        let x = object.x.unwrap().v;
        let y = object.y.unwrap().v;
        let z = object.z.unwrap().v;
        // Compare the data
        let a = (coords.x, coords.y, coords.z);
        let b = (x, y, z);
        ensure!(
            izip!([a.0, a.1, a.2], [b.0, b.1, b.2])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon() * 10.)),
            "Cartesian coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
