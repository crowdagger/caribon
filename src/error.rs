use std::error;
use std::result;
use std::fmt;

#[derive(Debug)]
/// Caribon error type (just a String for now)
pub struct Error {
    pub content: String
}

impl Error {
    pub fn new(s: &str) -> Error {
        Error { content: s.to_string() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.content)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.content
    }
}

/// Result from Caribon functions
pub type Result<T> = result::Result<T, Error>;
