//! Input data record

use num::Float;
use serde::Deserialize;

/// Input data record
#[derive(Deserialize)]
pub(in crate::model) struct Record<F: Float> {
    /// Name
    pub(in crate::model) name: String,
    /// Right ascension (string in the HMS form)
    pub(in crate::model) alpha: String,
    /// Declination (string in the DMS form)
    pub(in crate::model) delta: String,
    /// Parallax
    pub(in crate::model) par: F,
    /// Standard error of the parallax
    #[allow(dead_code)]
    pub(in crate::model) sigma_par: F,
    /// Radial velocity
    #[allow(dead_code)]
    pub(in crate::model) v: F,
    /// Standard error of the radial velocity
    #[allow(dead_code)]
    pub(in crate::model) sigma_v: F,
    /// Proper motion in the X coordinate
    #[allow(dead_code)]
    pub(in crate::model) mu_x: F,
    /// Standard error of the proper motion in the X coordinate
    #[allow(dead_code)]
    pub(in crate::model) sigma_mu_x: F,
    /// Proper motion in the Y coordinate
    #[allow(dead_code)]
    pub(in crate::model) mu_y: F,
    /// Standard error of the proper motion in the Y coordinate
    #[allow(dead_code)]
    pub(in crate::model) sigma_mu_y: F,
    /// Type of the object
    #[allow(dead_code)]
    pub(in crate::model) obj_type: String,
    /// Source reference
    #[allow(dead_code)]
    pub(in crate::model) reference: String,
}
