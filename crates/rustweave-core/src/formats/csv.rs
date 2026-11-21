// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

use csv::{ReaderBuilder, Terminator};

use super::{bool_option, char_option, object_entries, option, string_option, validate_encoding};
use crate::error::DwError;
use crate::value::Value;

#[derive(Clone)]
struct CsvOptions {
    header: bool,
    separator: u8,
    quote: u8,
    escape: u8,
    escape_explicit: bool,
    quote_values: bool,
    record_separator: String,
    skip_empty_line: bool,
    null_value: Option<String>,
    header_line_number: usize,
    body_start_line_number: Option<usize>,
}

impl CsvOptions {
    fn read(options: Option<&Value>) -> Result<Self, DwError> {
        validate_encoding(options)?;
        Ok(CsvOptions {
            header: bool_option(options, "header", true)?,
            separator: char_option(options, "separator", ',')?,
            quote: char_option(options, "quote", '"')?,
            escape: char_option(options, "escape", '\\')?,
            escape_explicit: option(options, "escape").is_some(),
            quote_values: bool_option(options, "quoteValues", true)?,
            record_separator: string_option(options, "recordSeparator", Some("\n"))?
                .unwrap_or_else(|| "\n".to_string()),
            skip_empty_line: bool_option(options, "skipEmptyLine", true)?,
            null_value: string_option(options, "nullValue", None)?,
            header_line_number: line_number(options, "headerLineNumber", 1)?,
            body_start_line_number: match option(options, "bodyStartLineNumber") {
                Some(_) => Some(line_number(options, "bodyStartLineNumber", 1)?),
                None => None,
            },
        })
    }

    fn read_terminator(&self) -> Result<Terminator, DwError> {
        match self.record_separator.as_str() {
            "\n" | "\r\n" => Ok(Terminator::CRLF),
            text if text.len() == 1 && text.is_ascii() => Ok(Terminator::Any(text.as_bytes()[0])),
            _ => Err(DwError::new(
                "CSV recordSeparator must be one ASCII character or a line break",
            )),
        }
    }

    fn write_line_ending(&self) -> Result<&str, DwError> {
        match self.record_separator.as_str() {
            "\r\n" => Ok("\r\n"),
            text if text.len() == 1 && text.is_ascii() => Ok(text),
            _ => Err(DwError::new(
                "CSV recordSeparator must be one ASCII character or a line break",
            )),
        }
    }

    fn field(&self, text: &str) -> Value {
        match &self.null_value {
            Some(null_value) if text == null_value => Value::Null,
            _ => Value::String(text.to_string()),
        }
    }
}

/// Reads a non-negative line number from a writer or reader option.
fn line_number(options: Option<&Value>, key: &str, default: usize) -> Result<usize, DwError> {
    match option(options, key) {
        None => Ok(default),
        Some(Value::Number(number)) => number
            .to_i64()
            .filter(|value| *value >= 1)
            .map(|value| value as usize)
            .ok_or_else(|| DwError::new(format!("option '{key}' must be a positive integer"))),
        Some(other) => Err(DwError::new(format!(
            "option '{key}' must be a positive integer, found {}",
            other.type_name()
        ))),
    }
}

fn preserve_empty_csv_records(text: &str, options: &CsvOptions) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut start = 0;
    let mut index = 0;
    let mut in_quotes = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_quotes {
            if byte == options.escape && options.escape != options.quote {
                index = (index + 2).min(bytes.len());
                continue;
            }
            if byte == options.quote {
                if bytes.get(index + 1) == Some(&options.quote) {
                    index += 2;
                    continue;
                }
                in_quotes = false;
            }
            index += 1;
            continue;
        }
        if byte == options.quote {
            in_quotes = true;
            index += 1;
            continue;
        }
        let terminator_len = if byte == b'\r' {
            if bytes.get(index + 1) == Some(&b'\n') {
                2
            } else {
                1
            }
        } else if byte == b'\n'
            || (options.record_separator.len() == 1
                && byte == options.record_separator.as_bytes()[0])
        {
            1
        } else {
            0
        };
        if terminator_len == 0 {
            index += 1;
            continue;
        }
        let record = &bytes[start..index];
        if record.is_empty() {
            out.push(options.quote);
            out.push(options.quote);
        } else {
            out.extend_from_slice(record);
        }
        out.extend_from_slice(&bytes[index..index + terminator_len]);
        index += terminator_len;
        start = index;
    }
    out.extend_from_slice(&bytes[start..]);
    String::from_utf8_lossy(&out).into_owned()
}

pub(crate) fn read_csv(text: &str, options: Option<&Value>) -> Result<Value, DwError> {
    let options = CsvOptions::read(options)?;
    let terminator = options.read_terminator()?;
    let prepared;
    let text = if options.skip_empty_line {
        text
    } else {
        prepared = preserve_empty_csv_records(text, &options);
        prepared.as_str()
    };
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(options.separator)
        .quote(options.quote)
        .escape(Some(options.escape))
        .terminator(terminator)
        .from_reader(text.as_bytes());

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|error| DwError::new(format!("invalid CSV: {error}")))?;
        rows.push(record.iter().map(str::to_string).collect::<Vec<_>>());
    }

    if !options.header {
        let start = options
            .body_start_line_number
            .unwrap_or(1)
            .saturating_sub(1);
        return Ok(Value::Array(
            rows.into_iter()
                .skip(start)
                .map(|row| {
                    Value::Object(
                        row.iter()
                            .enumerate()
                            .map(|(index, field)| (format!("column_{index}"), options.field(field)))
                            .collect(),
                    )
                })
                .collect(),
        ));
    }

    let header_index = options.header_line_number.saturating_sub(1);
    let Some(headers) = rows.get(header_index).cloned() else {
        return Ok(Value::Array(Vec::new()));
    };
    let mut seen = HashSet::new();
    for header in &headers {
        if !seen.insert(header.clone()) {
            return Err(DwError::new(format!("duplicate CSV header '{header}'")));
        }
    }

    let body_start = options
        .body_start_line_number
        .unwrap_or(options.header_line_number + 1)
        .saturating_sub(1);
    let mut out = Vec::new();
    for row in rows.into_iter().skip(body_start) {
        if row.len() > headers.len() {
            return Err(DwError::new("CSV row has more fields than the header row"));
        }
        let mut entries = Vec::with_capacity(headers.len());
        for (index, header) in headers.iter().enumerate() {
            let field = row.get(index).map(String::as_str).unwrap_or("");
            entries.push((header.clone(), options.field(field)));
        }
        out.push(Value::Object(entries));
    }
    Ok(Value::Array(out))
}

fn csv_cell(value: &Value) -> Result<String, DwError> {
    match value {
        Value::Null => Ok(String::new()),
        Value::Array(_) | Value::Object(_) => Err(DwError::new(
            "CSV cells cannot contain nested arrays or objects",
        )),
        other => Ok(other.key_string()),
    }
}

/// Formats one field using either quoting or escape-character rules.
fn format_field(field: &str, options: &CsvOptions, escape_mode: bool) -> String {
    let special = |byte: u8| {
        byte == options.separator || byte == options.quote || byte == b'\r' || byte == b'\n'
    };
    if escape_mode {
        let mut out = String::with_capacity(field.len());
        for character in field.chars() {
            let byte = if character.is_ascii() {
                character as u8
            } else {
                0
            };
            if special(byte) || byte == options.escape {
                out.push(options.escape as char);
            }
            out.push(character);
        }
        return out;
    }
    if !field.bytes().any(special) {
        return field.to_string();
    }
    let mut out = String::with_capacity(field.len() + 2);
    out.push(options.quote as char);
    for character in field.chars() {
        if character.is_ascii() && character as u8 == options.quote {
            if options.escape == options.quote {
                out.push(options.quote as char);
            } else {
                out.push(options.escape as char);
            }
        }
        out.push(character);
    }
    out.push(options.quote as char);
    out
}

fn format_record(record: &[String], options: &CsvOptions, escape_mode: bool) -> String {
    record
        .iter()
        .map(|field| format_field(field, options, escape_mode))
        .collect::<Vec<_>>()
        .join(&(options.separator as char).to_string())
}

pub(crate) fn write_csv(value: &Value, options: Option<&Value>) -> Result<String, DwError> {
    let options = CsvOptions::read(options)?;
    let rows = match value {
        Value::Array(rows) => rows,
        other => {
            return Err(DwError::new(format!(
                "CSV output requires an array, found {}",
                other.type_name()
            )))
        }
    };
    if rows.is_empty() {
        return Ok(String::new());
    }

    let mut records: Vec<Vec<String>> = Vec::new();
    match &rows[0] {
        Value::Object(first) => {
            let mut headers = Vec::new();
            let mut seen = HashSet::new();
            for (key, _) in first {
                if seen.insert(key.clone()) {
                    headers.push(key.clone());
                }
            }
            if options.header {
                records.push(headers.clone());
            }
            for row in rows {
                let entries = object_entries(row)?;
                if entries.is_empty() {
                    records.push(Vec::new());
                    continue;
                }
                let mut record = Vec::with_capacity(headers.len());
                for header in &headers {
                    let cell = entries
                        .iter()
                        .rev()
                        .find(|(key, _)| key == header)
                        .map(|(_, value)| value)
                        .unwrap_or(&Value::Null);
                    record.push(csv_cell(cell)?);
                }
                records.push(record);
            }
        }
        Value::Array(_) => {
            for row in rows {
                let cells = match row {
                    Value::Array(cells) => cells,
                    other => {
                        return Err(DwError::new(format!(
                            "CSV rows must all be arrays, found {}",
                            other.type_name()
                        )))
                    }
                };
                records.push(cells.iter().map(csv_cell).collect::<Result<Vec<_>, _>>()?);
            }
        }
        other => {
            return Err(DwError::new(format!(
                "CSV rows must be objects or arrays, found {}",
                other.type_name()
            )))
        }
    }

    let line_ending = options.write_line_ending()?;
    let escape_mode = !options.quote_values || options.escape_explicit;
    let mut out = String::new();
    for record in &records {
        out.push_str(&format_record(record, &options, escape_mode));
        out.push_str(line_ending);
    }
    Ok(out)
}
