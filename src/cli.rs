//! Command-line interface

use super::Goal;
use crate::utils;

use core::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{builder::TypedValueParser, Parser};

/// Parser of angles in the hours-minutes-seconds form
#[derive(Clone)]
struct HMSParser;

#[allow(clippy::indexing_slicing)]
impl TypedValueParser for HMSParser {
    type Value = f64;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // If the string can be split into a vector of floats
            if let Ok(vec) = utils::str2vec(string) {
                // If there are at least three float numbers
                if vec.len() >= 3 {
                    // Convert these to the radians
                    return Ok(utils::hms2rad(vec[0], vec[1], vec[2]));
                }
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the HMS form from the string\n",
        )
        .with_cmd(cmd))
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
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // If the string can be split into a vector of floats
            if let Ok(vec) = utils::str2vec(string) {
                // If there are at least three float numbers
                if vec.len() >= 3 {
                    // Convert these to the radians
                    return Ok(utils::dms2rad(vec[0], vec[1], vec[2]));
                }
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the DMS form from the string\n",
        )
        .with_cmd(cmd))
    }
}

/// Parser of angles in the decimal degrees form
#[derive(Clone)]
struct DecParser;

impl TypedValueParser for DecParser {
    type Value = f64;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
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
            clap::error::ErrorKind::InvalidValue,
            "Couldn't parse an angle in the decimal degrees form from the string\n",
        )
        .with_cmd(cmd))
    }
}

/// Parser of ranges
#[derive(Clone)]
struct RangeParser;

impl TypedValueParser for RangeParser {
    type Value = Range<f64>;

    #[allow(clippy::indexing_slicing)]
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        // If the OS string can be converted to a regular string
        if let Some(string) = value.to_str() {
            // Split the string by `..`
            let substrings: Vec<&str> = string.split("..").collect();
            // If the number of substrings is exactly two
            if substrings.len() == 2 {
                // If both of these substrings can be converted to floats
                if let Ok(begin) = substrings[0].parse::<f64>() {
                    if let Ok(end) = substrings[1].parse::<f64>() {
                        // If the first number is smaller than the second one
                        if begin < end {
                            // Return the range
                            return Ok(begin..end);
                        }
                    }
                }
            }
        }
        // Otherwise, return an error
        Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "Couldn't parse a range from the string\n",
        )
        .with_cmd(cmd))
    }
}

/// Parser of paths
#[derive(Clone)]
pub struct PathBufParser;

impl TypedValueParser for PathBufParser {
    type Value = std::path::PathBuf;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, clap::Error> {
        if let Some(string) = value.to_str() {
            if Path::new(string).is_file() {
                return Ok(Self::Value::from(value));
            }
        }
        Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "Input must be an existing file\n",
        )
        .with_cmd(cmd))
    }
}

/// Command-line interface arguments
#[derive(Parser)]
#[command(author, version, about)]
#[command(help_template(
    "{before-help}{name} {version}\n\
    {author-with-newline}{about-with-newline}\n\
    {usage-heading} {usage}\n\n\
    {all-args}{after-help}"
))]
pub struct Args {
    /// Output directory
    #[arg(short, required = true)]
    pub output: PathBuf,
    /// Computation goal
    #[arg(long, required = true)]
    pub goal: Goal,
    /// Input files
    #[arg(short, required = true, value_parser = PathBufParser)]
    pub inputs: Vec<PathBuf>,
    /// The right ascension of the north galactic pole (HMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = HMSParser {}, default_value = "12:51:26.2817", help_heading = "PARAMETERS")]
    pub alpha_ngp: f64,
    /// The declination of the north galactic pole (DMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = DMSParser {}, default_value = "27:07:42.013", help_heading = "PARAMETERS")]
    pub delta_ngp: f64,
    /// Linear velocities units conversion coefficient
    ///
    /// Sources: Gromov, Nikiforov (2016)
    #[arg(long, default_value_t = 4.7406, help_heading = "PARAMETERS")]
    pub k: f64,
    /// The longitude of the north celestial pole (decimal degrees angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = DecParser {}, default_value_t = 122.932, help_heading = "PARAMETERS")]
    pub l_ncp: f64,
    /// Galactocentric distance to the Sun (kpc)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 8.15, help_heading = "PARAMETERS")]
    pub r_0: f64,
    /// Full circular velocity of the Sun (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 247., help_heading = "PARAMETERS")]
    pub theta_sun: f64,
    /// Peculiar motion locally toward GC (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 10.7, help_heading = "PARAMETERS")]
    pub u_sun: f64,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    #[arg(long, default_value_t = 28., help_heading = "PARAMETERS")]
    pub omega_0: f64,
    /// Oort's A constant (km/s/kpc)
    #[arg(long, default_value_t = 17., help_heading = "PARAMETERS")]
    pub a: f64,
    /// Standard Solar Motion toward GC (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 10.3, help_heading = "PARAMETERS")]
    pub u_sun_standard: f64,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 15.3, help_heading = "PARAMETERS")]
    pub v_sun_standard: f64,
    /// Standard Solar Motion toward NGP (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 7.7, help_heading = "PARAMETERS")]
    pub w_sun_standard: f64,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 6., help_heading = "PARAMETERS")]
    pub sigma_r: f64,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 12., help_heading = "PARAMETERS")]
    pub sigma_theta: f64,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 3., help_heading = "PARAMETERS")]
    pub sigma_z: f64,
    /// Galactocentric distance to the Sun (kpc)
    #[arg(long, value_parser = RangeParser, default_value = "7.0..9.0", help_heading = "BOUNDS")]
    pub r_0_bounds: Range<f64>,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    #[arg(long, value_parser = RangeParser, default_value = "1.0..35.0", help_heading = "BOUNDS")]
    pub omega_0_bounds: Range<f64>,
    /// Oort's A constant (km/s/kpc)
    #[arg(long, value_parser = RangeParser, default_value = "10.0..20.0", help_heading = "BOUNDS")]
    pub a_bounds: Range<f64>,
    /// Standard Solar Motion toward GC (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "10.2..10.4", help_heading = "BOUNDS")]
    pub u_sun_standard_bounds: Range<f64>,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "15.2..15.4", help_heading = "BOUNDS")]
    pub v_sun_standard_bounds: Range<f64>,
    /// Standard Solar Motion toward NGP (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "7.6..7.8", help_heading = "BOUNDS")]
    pub w_sun_standard_bounds: Range<f64>,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "1.0..25.0", help_heading = "BOUNDS")]
    pub sigma_r_bounds: Range<f64>,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "1.0..25.0", help_heading = "BOUNDS")]
    pub sigma_theta_bounds: Range<f64>,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, value_parser = RangeParser, default_value = "1.0..25.0", help_heading = "BOUNDS")]
    pub sigma_z_bounds: Range<f64>,
}

/// Parse the arguments
pub fn parse() -> Args {
    Args::parse()
}
