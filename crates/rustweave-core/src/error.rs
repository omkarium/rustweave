// SPDX-License-Identifier: MIT OR Apache-2.0
//! Error type returned by parsing, evaluation, and JSON operations.

use std::fmt;

/// An error produced while parsing or evaluating a script.
///
/// The [`Display`](fmt::Display) implementation writes only the message.
#[derive(Clone, Debug)]
pub struct DwError {
    /// Human-readable description of the problem.
    pub message: String,
    /// A short error class name used by `dw::Runtime` results.
    pub kind: String,
}

impl DwError {
    /// Creates an error from any value that can become a string.
    pub fn new(message: impl Into<String>) -> Self {
        let message = message.into();
        let kind = classify(&message).to_string();
        DwError { message, kind }
    }

    /// Creates an error with an explicit error class name.
    pub fn with_kind(kind: impl Into<String>, message: impl Into<String>) -> Self {
        DwError {
            message: message.into(),
            kind: kind.into(),
        }
    }
}

/// Returns a default error class name for an error message.
fn classify(message: &str) -> &'static str {
    if message.contains("division by zero") {
        "DivisionByZeroException"
    } else {
        "RuntimeException"
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
