//! Galactic heliocentric Cartesian coordinates

use super::Object;

use core::fmt::Debug;

use num::Float;

#[allow(clippy::unwrap_used)]
#[allow(clippy::many_single_char_names)]
impl<F> Object<F> {
    /// Compute the coordinates with the specific values
    fn compute_x_y_z_with(&self, r_h: F) -> (F, F, F)
    where
        F: Float + Debug,
    {
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
    pub fn compute_x_y_z(&mut self)
    where
        F: Float + Debug,
    {
        // Unpack the data
        let r_h = self.r_h.unwrap();
        let r_h_p = self.r_h_p.unwrap();
        let r_h_m = self.r_h_m.unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = self.compute_x_y_z_with(r_h);
        let (x_p, y_p, z_p) = self.compute_x_y_z_with(r_h_p);
        let (x_m, y_m, z_m) = self.compute_x_y_z_with(r_h_m);
        self.x = Some(x);
        self.x_ep = Some(x_p - x);
        self.x_em = Some(x - x_m);
        self.x_p = Some(x_p);
        self.x_m = Some(x_m);
        self.y = Some(y);
        self.y_ep = Some(y_p - y);
        self.y_em = Some(y - y_m);
        self.y_p = Some(y_p);
        self.y_m = Some(y_m);
        self.z = Some(z);
        self.z_ep = Some(z_p - z);
        self.z_em = Some(z - z_m);
        self.z_p = Some(z_p);
        self.z_m = Some(z_m);
    }
    /// Compute the velocities in the Galactic
    /// heliocentric Cartesian coordinates system
    /// (with the specific values)
    pub fn compute_u_v_w_with(&self, v_l: F, v_b: F) -> (F, F, F)
    where
        F: Float + Debug,
    {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        let v_r = self.v_r.unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let aux_vel = v_r * b.cos() - v_b * b.sin();
        let u = aux_vel * l.cos() - v_l * l.sin();
        let v = aux_vel * l.sin() + v_l * l.cos();
        let w = v_b * b.cos() + v_r * b.sin();
        (u, v, w)
    }
    /// Compute the velocities in the Galactic
    /// heliocentric Cartesian coordinates system
    /// (nominal values only)
    pub fn compute_u_v_w_nominal(&mut self)
    where
        F: Float + Debug + Default,
    {
        // Unpack the data
        let v_l = self.v_l.unwrap();
        let v_b = self.v_b.unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (u, v, w) = self.compute_u_v_w_with(v_l, v_b);
        self.u = Some(u);
        self.v = Some(v);
        self.w = Some(w);
    }
    /// Compute the velocities in the Galactic
    /// heliocentric Cartesian coordinates system
    #[allow(clippy::similar_names)]
    pub fn compute_u_v_w(&mut self)
    where
        F: Float + Debug,
    {
        // Unpack the data
        let v_l = self.v_l.unwrap();
        let v_l_p = self.v_l_p.unwrap();
        let v_l_m = self.v_l_m.unwrap();
        let v_b = self.v_b.unwrap();
        let v_b_p = self.v_b_p.unwrap();
        let v_b_m = self.v_b_m.unwrap();
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (u, v, w) = self.compute_u_v_w_with(v_l, v_b);
        let (u_p, v_p, w_p) = self.compute_u_v_w_with(v_l_p, v_b_p);
        let (u_m, v_m, w_m) = self.compute_u_v_w_with(v_l_m, v_b_m);
        self.u = Some(u);
        self.u_ep = Some(u_p - u);
        self.u_em = Some(u - u_m);
        self.u_p = Some(u_p);
        self.u_m = Some(u_m);
        self.v = Some(v);
        self.v_ep = Some(v_p - v);
        self.v_em = Some(v - v_m);
        self.v_p = Some(v_p);
        self.v_m = Some(v_m);
        self.w = Some(w);
        self.w_ep = Some(w_p - w);
        self.w_em = Some(w - w_m);
        self.w_p = Some(w_p);
        self.w_m = Some(w_m);
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
        struct DataRecord<F> {
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
        struct CoordsRecord<F> {
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
        object.par = Some(data.par);
        object.compute_l_b(&params);
        object.compute_r_h();
        object.compute_x_y_z();
        let x = object.x.unwrap();
        let y = object.y.unwrap();
        let z = object.z.unwrap();
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
