//! Input related

use serde::Deserialize;

/// Input data record
#[derive(Deserialize)]
pub(in crate::model) struct Record<F> {
    /// Name
    pub(in crate::model) name: String,
    /// Right ascension (string in the HMS form)
    pub(in crate::model) alpha: String,
    /// Declination (string in the DMS form)
    pub(in crate::model) delta: String,
    /// Parallax
    pub(in crate::model) par: F,
    /// Uncertainty in `par`
    pub(in crate::model) e_par: F,
    /// Local Standard of Rest velocity
    pub(in crate::model) v_lsr: F,
    /// Uncertainty in `v_lsr`
    pub(in crate::model) e_v_lsr: F,
    /// Eastward proper motion
    pub(in crate::model) mu_x: F,
    /// Uncertainty in `mu_x`
    pub(in crate::model) e_mu_x: F,
    /// Northward proper motion
    pub(in crate::model) mu_y: F,
    /// Uncertainty in `mu_y`
    pub(in crate::model) e_mu_y: F,
    /// Type of the object
    #[serde(rename = "type")]
    pub(in crate::model) obj_type: String,
    /// Sources of the data
    pub(in crate::model) source: String,
    /// Reference(s)
    #[allow(dead_code)]
    pub(in crate::model) reference: String,
}
