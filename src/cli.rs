//! Command-line interface

use crate::utils;

use core::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{builder::TypedValueParser, Parser};

/// Computation goal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Goal {
    /// Perform per-object computations
    Objects,
    /// Fit the model of the Galaxy to the data
    Fit,
}

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
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(author, version, about)]
#[command(help_template(
    "{before-help}{name} {version}\n\
    {author-with-newline}{about-with-newline}\n\
    {usage-heading} {usage}\n\n\
    {all-args}{after-help}"
))]
pub struct Args {
    /// Input file
    #[arg(short, required = true, value_parser = PathBufParser)]
    pub input: PathBuf,
    /// Output directory
    #[arg(short, required = true)]
    pub output_dir: PathBuf,
    /// Computation goal
    #[arg(long, required = true)]
    pub goal: Goal,
    /// Optimal degree of the polynomial of the rotation curve
    ///
    /// Maximum supported value is 8.
    #[arg(long, default_value_t = 1)]
    pub n_best: usize,
    /// Maximum degree of the polynomial of the rotation curve
    ///
    /// Maximum supported value is 8.
    #[arg(long, default_value_t = 6)]
    pub n_max: usize,
    /// Try to define the confidence intervals (fit goal only)
    #[arg(long)]
    pub with_errors: bool,
    /// Try to compute conditional profiles (fit goal only)
    #[arg(long)]
    pub with_conditional_profiles: bool,
    /// Disable the inner optimization (fit goal only)
    #[arg(long)]
    pub disable_inner: bool,
    /// Disable checks for outliers
    #[arg(long)]
    pub disable_outliers: bool,
    /// Tolerance of the L-BFGS algorithm
    #[arg(long, default_value_t = 1e-15)]
    pub lbfgs_tolerance: f64,
    /// Galactocentric distance to the Sun (kpc)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 8.15, help_heading = "Parameters")]
    pub r_0: f64,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    #[arg(long, default_value_t = 28., help_heading = "Parameters")]
    pub omega_0: f64,
    /// Oort's A constant (km/s/kpc)
    #[arg(long, default_value_t = 17., help_heading = "Parameters")]
    pub a: f64,
    /// Residual motion of the Sun toward GC (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 10.7, help_heading = "Parameters")]
    pub u_sun: f64,
    /// Residual motion of the Sun toward l = 90 degrees (km/s)
    ///
    /// This is only used for fitting.
    ///
    /// Sources: Rastorguev et al. (2017)
    #[arg(long, default_value_t = 19.0, help_heading = "Parameters")]
    pub v_sun: f64,
    /// Residual motion of the Sun toward NGP (km/s)
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 7.7, help_heading = "Parameters")]
    pub w_sun: f64,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 12., help_heading = "Parameters")]
    pub sigma_r_g: f64,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 6., help_heading = "Parameters")]
    pub sigma_theta: f64,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    #[arg(long, default_value_t = 3., help_heading = "Parameters")]
    pub sigma_z: f64,
    /// Linear rotation velocity of the Sun (km/s)
    ///
    /// This is only used for computing per-object data.
    ///
    /// Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 247., help_heading = "Parameters")]
    pub theta_sun: f64,
    /// The right ascension of the north galactic pole (HMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = HMSParser {}, default_value = "12:51:26.2817", help_heading = "Parameters")]
    pub alpha_ngp: f64,
    /// The declination of the north galactic pole (DMS angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = DMSParser {}, default_value = "27:07:42.013", help_heading = "Parameters")]
    pub delta_ngp: f64,
    /// Linear velocities units conversion coefficient
    ///
    /// Sources: Gromov, Nikiforov (2016)
    #[arg(long, default_value_t = 4.7406, help_heading = "Parameters")]
    pub k: f64,
    /// The longitude of the north celestial pole (decimal degrees angle -> radians)
    ///
    /// Source: Reid et al. (2009)
    #[arg(long, value_parser = DecParser {}, default_value_t = 122.932, help_heading = "Parameters")]
    pub l_ncp: f64,
    /// Standard Solar Motion toward GC (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 10.3, help_heading = "Parameters")]
    pub u_sun_standard: f64,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 15.3, help_heading = "Parameters")]
    pub v_sun_standard: f64,
    /// Standard Solar Motion toward NGP (km/s)
    ///
    /// Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
    #[arg(long, default_value_t = 7.7, help_heading = "Parameters")]
    pub w_sun_standard: f64,
}

/// Parse the arguments
pub fn parse() -> Args {
    Args::parse()
}
