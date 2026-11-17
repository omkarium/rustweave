//! Error type returned by parsing, evaluation, and JSON operations.

use std::fmt;

/// An error produced while parsing or evaluating a script.
///
/// The [`Display`](fmt::Display) implementation writes only the message.
#[derive(Clone, Debug)]
pub struct DwError {
    /// Human-readable description of the problem.
    pub message: String,
}

impl DwError {
    /// Creates an error from any value that can become a string.
    pub fn new(message: impl Into<String>) -> Self {
        DwError {
            message: message.into(),
        }
    }
}

impl fmt::Display for DwError {
    /// Writes the bare error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DwError {}

impl From<String> for DwError {
    /// Creates an error from an owned message.
    fn from(value: String) -> Self {
        DwError::new(value)
    }
}

impl From<&str> for DwError {
    /// Creates an error from a borrowed message.
    fn from(value: &str) -> Self {
        DwError::new(value)
    }
}
