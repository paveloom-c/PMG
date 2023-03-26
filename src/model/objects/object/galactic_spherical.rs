//! Galactic heliocentric spherical coordinates

use super::{Measurement, Object};
use crate::model::Params;

use core::fmt::Debug;

use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F>
where
    F: Float + FloatConst + Default + Debug,
{
    /// Convert the equatorial coordinates to spherical heliocentric Galactic coordinates
    ///
    /// Angles must be in radians, then radians are returned.
    ///
    /// Source: [Wikipedia](https://en.wikipedia.org/wiki/Galactic_coordinate_system#Conversion_between_equatorial_and_galactic_coordinates)
    pub fn compute_l_b<F2>(&mut self, params: &Params<F2>)
    where
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let alpha = self.alpha.unwrap();
        let delta = self.delta.unwrap();
        // Get the parameters
        let alpha_ngp: F = params.alpha_ngp.into();
        let delta_ngp: F = params.delta_ngp.into();
        let l_ncp: F = params.l_ncp.into();
        // Compute the angles
        let phi = F::atan2(
            F::cos(delta) * F::sin(alpha - alpha_ngp),
            F::cos(delta_ngp) * F::sin(delta)
                - F::sin(delta_ngp) * F::cos(delta) * F::cos(alpha - alpha_ngp),
        );
        self.l = Some(l_ncp - phi);
        self.b = Some(F::asin(
            F::sin(delta_ngp) * F::sin(delta)
                + F::cos(delta_ngp) * F::cos(delta) * F::cos(alpha - alpha_ngp),
        ));
    }
    /// Compute the heliocentric distance (nominal value only)
    pub fn compute_r_h_nominal(&mut self) {
        let par = self.par.as_ref().unwrap();
        self.r_h = Some(Measurement {
            v: 1. / par.v,
            ..Default::default()
        });
    }
    /// Compute the heliocentric distance
    pub fn compute_r_h(&mut self) {
        // Unpack the data
        let par = self.par.as_ref().unwrap();
        // Compute the heliocentric distance
        let r_h = 1. / par.v;
        let r_h_u = 1. / par.v_u;
        let r_h_l = 1. / par.v_l;
        self.r_h = Some(Measurement {
            v: r_h,
            v_u: r_h_u,
            v_l: r_h_l,
            e_p: r_h_u - r_h,
            e_m: r_h - r_h_l,
        });
    }
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use std::path::Path;

        use anyhow::{ensure, Context, Result};
        use csv::ReaderBuilder;
        use itertools::izip;
        use serde::Deserialize;

        /// Data record
        #[derive(Deserialize)]
        struct Record<F: Float + Debug> {
            /// Name of the object
            #[allow(dead_code)]
            name: String,
            /// Right ascension (degrees)
            alpha: F,
            /// Declination (degrees)
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
        let record: Record<f64> = data_record
            .with_context(|| format!("Couldn't deserialize a record from {data_path:?}"))?;
        // Create an object
        let mut object = Object {
            alpha: Some(record.alpha.to_radians()),
            delta: Some(record.delta.to_radians()),
            ..Default::default()
        };
        // Compute the spherical coordinates
        object.compute_l_b(&params);
        // Compare the data
        let a = (record.l.to_radians(), record.b.to_radians());
        let b = (object.l.unwrap(), object.b.unwrap());
        ensure!(
            izip!([a.0, a.1], [b.0, b.1])
                .all(|(v1, v2)| (v1 - v2).abs() < f64::from(f32::epsilon())),
            "Spherical coordinates don't match at the record #{i}: {a:?} vs. {b:?}"
        );
        i += 1;
    }
    Ok(())
}
