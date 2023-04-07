//! Galactic heliocentric spherical coordinates

use super::{Object, Params};

use core::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Object<F> {
    /// Convert the equatorial coordinates to spherical heliocentric Galactic coordinates
    ///
    /// Angles must be in radians, then radians are returned.
    ///
    /// Source: [Wikipedia](https://en.wikipedia.org/wiki/Galactic_coordinate_system#Conversion_between_equatorial_and_galactic_coordinates)
    pub fn compute_l_b<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug,
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
    pub fn compute_r_h_nominal(&mut self)
    where
        F: Float + Debug + Default,
    {
        let par = self.par.unwrap();
        self.r_h = Some(1. / par);
    }
    /// Compute the heliocentric distance
    pub fn compute_r_h(&mut self)
    where
        F: Float + Debug,
    {
        // Unpack the data
        let par = self.par.unwrap();
        let par_p = self.par_p.unwrap();
        let par_m = self.par_m.unwrap();
        // Compute the heliocentric distance
        let r_h = 1. / par;
        let r_h_p = 1. / par_p;
        let r_h_m = 1. / par_m;
        self.r_h = Some(r_h);
        self.r_h_ep = Some(r_h_p - r_h);
        self.r_h_em = Some(r_h - r_h_m);
        self.r_h_p = Some(r_h_p);
        self.r_h_m = Some(r_h_m);
    }
    /// Compute the heliocentric velocity in distance
    pub fn compute_v_r<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let l = self.l.unwrap();
        let b = self.b.unwrap();
        let v_lsr = self.v_lsr.unwrap();
        // Get the parameters
        let u_sun_standard: F = params.u_sun_standard.into();
        let v_sun_standard: F = params.v_sun_standard.into();
        let w_sun_standard: F = params.w_sun_standard.into();
        // Compute the heliocentric velocity
        self.v_r = Some(
            v_lsr
                - (u_sun_standard * l.cos() + v_sun_standard * l.sin()) * b.cos()
                - w_sun_standard * b.sin(),
        );
    }
    /// Compute the velocities in longitude and
    /// latitude with the specific values
    fn compute_v_l_v_b_with<F2>(&self, r_h: F, params: &Params<F2>) -> (F, F)
    where
        F: Float + Debug,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let mu_l_cos_b = self.mu_l_cos_b.unwrap();
        let mu_b = self.mu_b.unwrap();
        // Get the parameters
        let k: F = params.k.into();
        // Compute the heliocentric velocity
        let v_l = k * r_h * mu_l_cos_b;
        let v_b = k * r_h * mu_b;
        (v_l, v_b)
    }
    /// Compute the velocities in longitude and latitude
    /// (nominal values only)
    #[allow(clippy::similar_names)]
    pub fn compute_v_l_v_b_nominal<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug + Default,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.unwrap();
        // Compute the heliocentric velocity
        let (v_l, v_b) = self.compute_v_l_v_b_with(r_h, params);
        self.v_l = Some(v_l);
        self.v_b = Some(v_b);
    }
    /// Compute the velocities in longitude and latitude
    #[allow(clippy::similar_names)]
    pub fn compute_v_l_v_b<F2>(&mut self, params: &Params<F2>)
    where
        F: Float + Debug,
        F2: Float + Debug + Into<F>,
    {
        // Unpack the data
        let r_h = self.r_h.unwrap();
        let r_h_p = self.r_h_p.unwrap();
        let r_h_m = self.r_h_m.unwrap();
        // Compute the heliocentric velocity
        let (v_l, v_b) = self.compute_v_l_v_b_with(r_h, params);
        let (v_l_p, v_b_p) = self.compute_v_l_v_b_with(r_h_p, params);
        let (v_l_m, v_b_m) = self.compute_v_l_v_b_with(r_h_m, params);
        self.v_l = Some(v_l);
        self.v_l_p = Some(v_l_p);
        self.v_l_m = Some(v_l_m);
        self.v_l_ep = Some(v_l_p - v_l);
        self.v_l_em = Some(v_l - v_l_m);
        self.v_b = Some(v_b);
        self.v_b_p = Some(v_b_p);
        self.v_b_m = Some(v_b_m);
        self.v_b_ep = Some(v_b_p - v_b);
        self.v_b_em = Some(v_b - v_b_m);
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
        struct Record<F> {
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
