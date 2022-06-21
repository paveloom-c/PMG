//! Command-line interface

use super::Goal;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::{builder::EnumValueParser, Parser};

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Output directory
    #[clap(short, required = true, validator = Self::validate_output)]
    pub output: PathBuf,
    /// Computation goals
    #[clap(long, multiple_values = true, required = true, value_parser = EnumValueParser::<Goal>::new())]
    pub goals: Vec<Goal>,
    /// Input files
    #[clap(short, multiple_values = true, required = true, validator = Self::validate_input)]
    pub inputs: Vec<PathBuf>,
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
    /// Check if the path to the output directory is valid
    fn validate_output(s: &str) -> Result<()> {
        if Path::new(s).is_dir() {
            Ok(())
        } else {
            Err(anyhow!("Output must be an existing directory"))
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
