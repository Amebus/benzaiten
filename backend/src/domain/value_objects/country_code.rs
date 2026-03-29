use serde::{Deserialize, Serialize};
use std::fmt;

/// Codice paese ISO 3166-1 alpha-2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn new(code: &str) -> Result<Self, String> {
        if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(CountryCode(code.to_uppercase()))
        } else {
            Err(format!("Codice paese non valido: {}", code))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
