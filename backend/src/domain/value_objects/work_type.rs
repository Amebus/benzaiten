use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo di opera nel catalogo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum WorkType {
    Manga,
    Anime,
    Movie,
    Music,
}

impl fmt::Display for WorkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkType::Manga => write!(f, "MANGA"),
            WorkType::Anime => write!(f, "ANIME"),
            WorkType::Movie => write!(f, "MOVIE"),
            WorkType::Music => write!(f, "MUSIC"),
        }
    }
}

impl std::str::FromStr for WorkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MANGA" => Ok(WorkType::Manga),
            "ANIME" => Ok(WorkType::Anime),
            "MOVIE" => Ok(WorkType::Movie),
            "MUSIC" => Ok(WorkType::Music),
            _ => Err(format!("Tipo opera sconosciuto: {}", s)),
        }
    }
}
