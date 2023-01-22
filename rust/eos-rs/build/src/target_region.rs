use std::env;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum TargetRegionError {
    Unknown(String),
    Missing,
}

impl Display for TargetRegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetRegionError::Unknown(reg) => {
                write!(f, "The game region '{reg}' is unknown.")
            }
            TargetRegionError::Missing => {
                write!(f, "The game region could not be determined from the target name. Make sure the target name ends in -na, -eu or -ja.")
            }
        }
    }
}

impl Error for TargetRegionError {}

pub enum TargetRegion {
    Eu,
    Na,
    Ja,
}

impl TargetRegion {
    /// Build from a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(string: S) -> Result<Self, TargetRegionError> {
        match string.as_ref().to_lowercase().as_str() {
            "eu" => Ok(Self::Eu),
            "na" => Ok(Self::Na),
            "ja" => Ok(Self::Ja),
            x => Err(TargetRegionError::Unknown(x.to_string())),
        }
    }

    /// Try to interpret from the target string in env variable TARGET
    pub fn from_target_env() -> Result<Self, TargetRegionError> {
        match env::var("TARGET") {
            Ok(target_str) => Self::from_target_string(target_str),
            Err(_) => Err(TargetRegionError::Missing),
        }
    }

    /// Try to interpret from the target string
    pub fn from_target_string<S: AsRef<str>>(target_str: S) -> Result<Self, TargetRegionError> {
        if let Some(last) = target_str.as_ref().split('-').last() {
            Self::from_str(last)
        } else {
            Err(TargetRegionError::Missing)
        }
    }

    pub fn as_str_lower(&self) -> &'static str {
        match self {
            TargetRegion::Eu => "eu",
            TargetRegion::Na => "na",
            TargetRegion::Ja => "ja",
        }
    }

    pub fn as_str_upper(&self) -> &'static str {
        match self {
            TargetRegion::Eu => "EU",
            TargetRegion::Na => "NA",
            TargetRegion::Ja => "JA",
        }
    }

    /// Returns the canonical target names as they are used in this repository.
    pub fn target_str(&self) -> &'static str {
        match self {
            TargetRegion::Eu => "armv5te-none-ndseoseabi-eu",
            TargetRegion::Na => "armv5te-none-ndseoseabi-na",
            TargetRegion::Ja => "armv5te-none-ndseoseabi-ja",
        }
    }
}
