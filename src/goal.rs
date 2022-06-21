//! Computation goal

use std::cmp::Ord;

/// Computation goal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Goal {
    /// Convert equatorial coordinates to the Galactic
    /// heliocentric spherical and Cartesian coordinates
    Coords,
}

impl clap::ValueEnum for Goal {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Coords]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        match self {
            &Self::Coords => Some(clap::PossibleValue::new("coords")),
        }
    }
}
