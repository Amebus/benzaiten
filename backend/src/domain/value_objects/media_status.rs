use serde::{Deserialize, Serialize};
use std::fmt;

/// Stato di rilascio/distribuzione di un media
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaStatus {
    Ongoing,
    Completed,
    Dropped,
    Announced,
}

impl fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaStatus::Ongoing => write!(f, "ONGOING"),
            MediaStatus::Completed => write!(f, "COMPLETED"),
            MediaStatus::Dropped => write!(f, "DROPPED"),
            MediaStatus::Announced => write!(f, "ANNOUNCED"),
        }
    }
}

impl std::str::FromStr for MediaStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONGOING" => Ok(MediaStatus::Ongoing),
            "COMPLETED" => Ok(MediaStatus::Completed),
            "DROPPED" => Ok(MediaStatus::Dropped),
            "ANNOUNCED" => Ok(MediaStatus::Announced),
            _ => Err(format!("Stato media sconosciuto: {}", s)),
        }
    }
}
