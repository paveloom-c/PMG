//! Command-line interface

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Output directory
    #[clap(short, validator = Self::validate_output)]
    pub output: PathBuf,
    /// Input files
    #[clap(multiple_values = true, required = true, validator = Self::validate_input)]
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
    Args::parse()
}
