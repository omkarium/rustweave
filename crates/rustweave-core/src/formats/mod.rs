// SPDX-License-Identifier: MIT OR Apache-2.0
//! CSV, XML, and YAML readers and writers.

use crate::error::DwError;
use crate::value::Value;

mod csv;
mod xml;
mod yaml;

pub(crate) use csv::{read_csv, write_csv};
pub(crate) use xml::{read_xml, write_xml};
pub(crate) use yaml::{read_yaml, write_yaml};

/// Returns the primary MIME type in lower case, without parameters.
pub(crate) fn normalize_mime(text: &str) -> String {
    let primary = text.split(';').next().unwrap_or(text);
    primary
        .split_whitespace()
        .next()
        .unwrap_or(primary)
        .trim()
        .to_ascii_lowercase()
}

/// Maps a format ID such as `json` to its MIME type.
///
/// Other text is treated as a MIME type and only normalized.
pub(crate) fn resolve_format(text: &str) -> String {
    match normalize_mime(text).as_str() {
        "json" => "application/json".to_string(),
        "csv" => "application/csv".to_string(),
        "xml" => "application/xml".to_string(),
        "dw" => "application/dw".to_string(),
        "text" => "text/plain".to_string(),
        other => other.to_string(),
    }
}

/// Maps a writer name from an `output ... with writer` clause to its MIME type.
pub(crate) fn resolve_writer(text: &str) -> Result<String, DwError> {
    match normalize_mime(text).as_str() {
        "json" => Ok("application/json".to_string()),
        "csv" => Ok("application/csv".to_string()),
        "xml" => Ok("application/xml".to_string()),
        "dw" => Ok("application/dw".to_string()),
        "text" => Ok("text/plain".to_string()),
        other => Err(DwError::new(format!("unknown writer '{other}'"))),
    }
}

/// Splits a `prefix#name` key when the prefix is a declared namespace.
pub(crate) fn namespace_key<'a>(
    key: &'a str,
    namespaces: &[(String, String)],
) -> Option<(&'a str, &'a str)> {
    let (prefix, local) = key.split_once('#')?;
    namespaces
        .iter()
        .any(|(declared, _)| declared == prefix)
        .then_some((prefix, local))
}

/// Returns a copy of a value with declared namespace prefixes removed from keys.
pub(crate) fn strip_namespace_keys(value: &Value, namespaces: &[(String, String)]) -> Value {
    if namespaces.is_empty() {
        return value.clone();
    }
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| strip_namespace_keys(item, namespaces))
                .collect(),
        ),
        Value::Object(entries) => Value::Object(
            entries
                .iter()
                .map(|(key, item)| {
                    let key = namespace_key(key, namespaces)
                        .map(|(_, local)| local.to_string())
                        .unwrap_or_else(|| key.clone());
                    (key, strip_namespace_keys(item, namespaces))
                })
                .collect(),
        ),
        Value::Meta(inner, entries) => Value::Meta(
            Box::new(strip_namespace_keys(inner, namespaces)),
            entries.clone(),
        ),
        Value::Formatted(inner, display) => Value::Formatted(
            Box::new(strip_namespace_keys(inner, namespaces)),
            display.clone(),
        ),
        other => other.clone(),
    }
}

pub(super) fn object_entries(value: &Value) -> Result<&[(String, Value)], DwError> {
    match value {
        Value::Object(entries) => Ok(entries),
        other => Err(DwError::new(format!(
            "expected Object, found {}",
            other.type_name()
        ))),
    }
}

pub(super) fn option<'a>(options: Option<&'a Value>, key: &str) -> Option<&'a Value> {
    match options {
        Some(Value::Object(entries)) => entries
            .iter()
            .rev()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value),
        _ => None,
    }
}

pub(super) fn bool_option(
    options: Option<&Value>,
    key: &str,
    default: bool,
) -> Result<bool, DwError> {
    match option(options, key) {
        None => Ok(default),
        Some(Value::Bool(value)) => Ok(*value),
        Some(other) => Err(DwError::new(format!(
            "option '{key}' must be a Boolean, found {}",
            other.type_name()
        ))),
    }
}

pub(super) fn string_option(
    options: Option<&Value>,
    key: &str,
    default: Option<&str>,
) -> Result<Option<String>, DwError> {
    match option(options, key) {
        None => Ok(default.map(str::to_string)),
        Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(other) => Err(DwError::new(format!(
            "option '{key}' must be a String, found {}",
            other.type_name()
        ))),
    }
}

pub(super) fn char_option(
    options: Option<&Value>,
    key: &str,
    default: char,
) -> Result<u8, DwError> {
    let text = string_option(options, key, None)?.unwrap_or_else(|| default.to_string());
    let mut chars = text.chars();
    let character = chars
        .next()
        .ok_or_else(|| DwError::new(format!("option '{key}' must not be empty")))?;
    if chars.next().is_some() || !character.is_ascii() {
        return Err(DwError::new(format!(
            "option '{key}' must be a single ASCII character"
        )));
    }
    Ok(character as u8)
}

pub(super) fn validate_encoding(options: Option<&Value>) -> Result<(), DwError> {
    if let Some(value) = option(options, "encoding") {
        let text = match value {
            Value::Null => return Ok(()),
            Value::String(text) => text,
            other => {
                return Err(DwError::new(format!(
                    "option 'encoding' must be a String, found {}",
                    other.type_name()
                )))
            }
        };
        if !text.eq_ignore_ascii_case("utf-8") && !text.eq_ignore_ascii_case("utf8") {
            return Err(DwError::new("only UTF-8 encoding is supported"));
        }
    }
    Ok(())
}
