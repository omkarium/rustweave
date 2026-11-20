// SPDX-License-Identifier: MIT OR Apache-2.0

use yaml_rust2::{Yaml, YamlLoader};

use super::{bool_option, option, validate_encoding};

use crate::error::DwError;
use crate::number::Number;
use crate::value::Value;

struct YamlOptions {
    indent: usize,
    write_document_start: bool,
}

impl YamlOptions {
    fn read(options: Option<&Value>) -> Result<Self, DwError> {
        validate_encoding(options)?;
        let indent = match option(options, "indent") {
            None => 2,
            Some(Value::Number(number)) => number
                .to_i64()
                .filter(|value| (1..=10).contains(value))
                .map(|value| value as usize)
                .ok_or_else(|| DwError::new("YAML indent must be an integer from 1 to 10"))?,
            Some(other) => {
                return Err(DwError::new(format!(
                    "option 'indent' must be a Number, found {}",
                    other.type_name()
                )))
            }
        };
        Ok(YamlOptions {
            indent,
            write_document_start: bool_option(options, "writeDocumentStart", false)?,
        })
    }
}

fn yaml_key(key: &Yaml) -> Result<String, DwError> {
    match key {
        Yaml::String(text) => Ok(text.clone()),
        Yaml::Integer(value) => Ok(value.to_string()),
        Yaml::Real(value) => Ok(value.clone()),
        Yaml::Boolean(value) => Ok(value.to_string()),
        Yaml::Null => Ok("null".to_string()),
        _ => Err(DwError::new("complex YAML keys are not supported")),
    }
}

fn yaml_value(value: &Yaml) -> Result<Value, DwError> {
    match value {
        Yaml::Null => Ok(Value::Null),
        Yaml::Boolean(value) => Ok(Value::Bool(*value)),
        Yaml::Integer(value) => Ok(Value::Number(Number::from_i64(*value))),
        Yaml::Real(value) => Number::parse(value)
            .map(Value::Number)
            .ok_or_else(|| DwError::new(format!("invalid YAML number '{value}'"))),
        Yaml::String(text) => Ok(Value::String(text.clone())),
        Yaml::Array(items) => Ok(Value::Array(
            items
                .iter()
                .map(yaml_value)
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Yaml::Hash(entries) => {
            let mut out = Vec::with_capacity(entries.len());
            for (key, item) in entries {
                out.push((yaml_key(key)?, yaml_value(item)?));
            }
            Ok(Value::Object(out))
        }
        Yaml::Alias(_) | Yaml::BadValue => Err(DwError::new("invalid YAML value")),
    }
}

pub(crate) fn read_yaml(text: &str, properties: Option<&Value>) -> Result<Value, DwError> {
    let _options = YamlOptions::read(properties)?;
    let documents = YamlLoader::load_from_str(text)
        .map_err(|error| DwError::new(format!("invalid YAML: {error}")))?;
    let values = documents
        .iter()
        .map(yaml_value)
        .collect::<Result<Vec<_>, _>>()?;
    match values.len() {
        0 => Ok(Value::Null),
        1 => Ok(values.into_iter().next().unwrap_or(Value::Null)),
        _ => Ok(Value::Array(values)),
    }
}

fn yaml_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other if other.is_control() => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out
}

fn yaml_needs_quotes(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    if text.trim() != text {
        return true;
    }
    let lower = text.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "null" | "~" | "true" | "false" | "yes" | "no" | "on" | "off"
    ) || Number::parse(text).is_some()
    {
        return true;
    }
    if text
        .chars()
        .any(|character| character.is_control() || ":#,[]{}&*!|>'\"%@`\\".contains(character))
    {
        return true;
    }
    text.starts_with('-') || text.starts_with('?') || text.contains(" #")
}

fn yaml_string(text: &str) -> String {
    if yaml_needs_quotes(text) {
        format!("\"{}\"", yaml_escape(text))
    } else {
        text.to_string()
    }
}

fn yaml_scalar(value: &Value) -> Result<String, DwError> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Number(value) => Ok(value.to_string()),
        Value::String(text) => Ok(yaml_string(text)),
        Value::Array(_) | Value::Object(_) => Err(DwError::new("expected a YAML scalar value")),
        other => Ok(yaml_string(&other.key_string())),
    }
}

fn write_yaml_multiline(out: &mut String, indent: usize, text: &str, width: usize) {
    let (body, indicator) = match text.strip_suffix('\n') {
        Some(body) => (body, "|"),
        None => (text, "|-"),
    };
    out.push(' ');
    out.push_str(indicator);
    out.push('\n');
    let pad = " ".repeat(indent * width);
    for line in body.split('\n') {
        out.push_str(&pad);
        out.push_str(line);
        out.push('\n');
    }
}

fn write_yaml_object(
    out: &mut String,
    entries: &[(String, Value)],
    indent: usize,
    width: usize,
) -> Result<(), DwError> {
    let pad = " ".repeat(indent * width);
    if entries.is_empty() {
        out.push_str(&pad);
        out.push_str("{}\n");
        return Ok(());
    }
    for (key, value) in entries {
        out.push_str(&pad);
        out.push_str(&yaml_string(key));
        out.push(':');
        match value {
            Value::Object(children) if !children.is_empty() => {
                out.push('\n');
                write_yaml_object(out, children, indent + 1, width)?;
            }
            Value::Object(_) => out.push_str(" {}\n"),
            Value::Array(items) if !items.is_empty() => {
                out.push('\n');
                write_yaml_array(out, items, indent + 1, width)?;
            }
            Value::Array(_) => out.push_str(" []\n"),
            Value::String(text) if text.contains('\n') => {
                write_yaml_multiline(out, indent + 1, text, width);
            }
            other => {
                out.push(' ');
                out.push_str(&yaml_scalar(other)?);
                out.push('\n');
            }
        }
    }
    Ok(())
}

fn write_yaml_array(
    out: &mut String,
    items: &[Value],
    indent: usize,
    width: usize,
) -> Result<(), DwError> {
    let pad = " ".repeat(indent * width);
    if items.is_empty() {
        out.push_str(&pad);
        out.push_str("[]\n");
        return Ok(());
    }
    for item in items {
        out.push_str(&pad);
        out.push('-');
        match item {
            Value::Object(children) if !children.is_empty() => {
                out.push('\n');
                write_yaml_object(out, children, indent + 1, width)?;
            }
            Value::Object(_) => out.push_str(" {}\n"),
            Value::Array(children) if !children.is_empty() => {
                out.push('\n');
                write_yaml_array(out, children, indent + 1, width)?;
            }
            Value::Array(_) => out.push_str(" []\n"),
            Value::String(text) if text.contains('\n') => {
                write_yaml_multiline(out, indent + 1, text, width);
            }
            other => {
                out.push(' ');
                out.push_str(&yaml_scalar(other)?);
                out.push('\n');
            }
        }
    }
    Ok(())
}

pub(crate) fn write_yaml(value: &Value, properties: Option<&Value>) -> Result<String, DwError> {
    let options = YamlOptions::read(properties)?;
    let mut out = String::new();
    if options.write_document_start {
        out.push_str("---\n");
    }
    match value {
        Value::Object(entries) => write_yaml_object(&mut out, entries, 0, options.indent)?,
        Value::Array(items) => write_yaml_array(&mut out, items, 0, options.indent)?,
        Value::String(text) if text.contains('\n') => {
            write_yaml_multiline(&mut out, 1, text, options.indent);
            if out.starts_with(' ') {
                out.remove(0);
            }
        }
        other => {
            out.push_str(&yaml_scalar(other)?);
            out.push('\n');
        }
    }
    Ok(out)
}
