//! Command-line interface

use super::Goal;
use crate::utils::{dms2rad, hms2rad, str2vec};

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::{
    builder::{EnumValueParser, TypedValueParser},
    Parser,
};

/// Parser of angles in the hours-minutes-seconds form
#[derive(Clone)]
struct HMSParser;

#[allow(clippy::indexing_slicing)]
impl TypedValueParser for HMSParser {
    type Value = f64;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // If the string can be split into a vector of floats
            if let Ok(vec) = str2vec(string) {
                // If there are at least three float numbers
                if vec.len() >= 3 {
                    // Convert these to the radians
                    return Ok(hms2rad(vec[0], vec[1], vec[2]));
                }
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the HMS form from the string",
        ))
    }
}

/// Parser of angles in the degrees-minutes-seconds form
#[derive(Clone)]
struct DMSParser;

#[allow(clippy::indexing_slicing)]
impl TypedValueParser for DMSParser {
    type Value = f64;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // If the string can be split into a vector of floats
            if let Ok(vec) = str2vec(string) {
                // If there are at least three float numbers
                if vec.len() >= 3 {
                    // Convert these to the radians
                    return Ok(dms2rad(vec[0], vec[1], vec[2]));
                }
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the DMS form from the string",
        ))
    }
}

/// Parser of angles in the decimal degrees form
#[derive(Clone)]
struct DecParser;

impl TypedValueParser for DecParser {
    type Value = f64;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // If the string can be parsed as a float
            if let Ok(decimal) = string.parse::<f64>() {
                // Convert it to radians
                return Ok(decimal.to_radians());
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the decimal degrees form from the string",
        ))
    }
}

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Output directory
    #[clap(short, required = true)]
    pub output: PathBuf,
    /// Computation goals
    #[clap(long, multiple_values = true, required = true, value_parser = EnumValueParser::<Goal>::new())]
    pub goals: Vec<Goal>,
    /// Input files
    #[clap(short, multiple_values = true, required = true, validator = Self::validate_input)]
    pub inputs: Vec<PathBuf>,
    /// The right ascension of the north galactic pole (HMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[clap(long, value_parser = HMSParser {}, default_value = "12:51:26.2817", help_heading = "CONSTANTS")]
    pub alpha_ngp: f64,
    /// The declination of the north galactic pole (DMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[clap(long, value_parser = DMSParser {}, default_value = "27:07:42.013", help_heading = "CONSTANTS")]
    pub delta_ngp: f64,
    /// Linear velocities units conversion coefficient
    ///
    /// Sources: Gromov, Nikiforov (2016)
    #[clap(long, default_value_t = 4.7406, help_heading = "CONSTANTS")]
    pub k: f64,
    /// The longitude of the north celestial pole (decimal degrees angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[clap(long, value_parser = DecParser {}, default_value_t = 122.932, help_heading = "CONSTANTS")]
    pub l_ncp: f64,
    /// Galactocentric distance to the Sun (kpc) [coords]
    #[clap(long, default_value_t = 8., help_heading = "CONSTANTS")]
    pub r_0_1: f64,
    /// Galactocentric distance to the Sun (kpc) [rotcurve]
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 8.15, help_heading = "CONSTANTS")]
    pub r_0_2: f64,
    /// Full circular velocity of the Sun (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 247., help_heading = "CONSTANTS")]
    pub theta_sun: f64,
    /// Peculiar motion locally toward GC (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 10.7, help_heading = "CONSTANTS")]
    pub u_sun: f64,
    /// Standard Solar Motion toward GC (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 10.3, help_heading = "CONSTANTS")]
    pub u_sun_standard: f64,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 15.3, help_heading = "CONSTANTS")]
    pub v_sun_standard: f64,
    /// Standard Solar Motion toward NGP (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[clap(long, default_value_t = 7.7, help_heading = "CONSTANTS")]
    pub w_sun_standard: f64,
}

impl Args {
    /// Check if the path to the input file is valid
    fn validate_input(s: &str) -> Result<()> {
        if Path::new(s).is_file() {
            Ok(())
        } else {
            Err(anyhow!("Input must be an existing file"))
        }
    }
}

/// Parse the arguments
pub fn parse() -> Args {
    // Parse the arguments
    let mut args = Args::parse();
    // Sort and dedup the goals
    args.goals.sort();
    args.goals.dedup();
    args
}
