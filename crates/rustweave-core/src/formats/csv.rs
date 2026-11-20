// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

use csv::{ReaderBuilder, Terminator, WriterBuilder};

use super::{bool_option, char_option, object_entries, string_option, validate_encoding};
use crate::error::DwError;
use crate::value::Value;

#[derive(Clone)]
struct CsvOptions {
    header: bool,
    separator: u8,
    quote: u8,
    escape: u8,
    record_separator: String,
    skip_empty_line: bool,
    null_value: Option<String>,
}

impl CsvOptions {
    fn read(options: Option<&Value>) -> Result<Self, DwError> {
        validate_encoding(options)?;
        Ok(CsvOptions {
            header: bool_option(options, "header", true)?,
            separator: char_option(options, "separator", ',')?,
            quote: char_option(options, "quote", '"')?,
            escape: char_option(options, "escape", '"')?,
            record_separator: string_option(options, "recordSeparator", Some("\n"))?
                .unwrap_or_else(|| "\n".to_string()),
            skip_empty_line: bool_option(options, "skipEmptyLine", true)?,
            null_value: string_option(options, "nullValue", None)?,
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

    fn write_terminator(&self) -> Result<Terminator, DwError> {
        match self.record_separator.as_str() {
            "\r\n" => Ok(Terminator::CRLF),
            text if text.len() == 1 && text.is_ascii() => Ok(Terminator::Any(text.as_bytes()[0])),
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
        return Ok(Value::Array(
            rows.into_iter()
                .map(|row| Value::Array(row.iter().map(|field| options.field(field)).collect()))
                .collect(),
        ));
    }

    let mut rows = rows.into_iter();
    let headers = rows.next().unwrap_or_default();
    let mut seen = HashSet::new();
    for header in &headers {
        if !seen.insert(header.clone()) {
            return Err(DwError::new(format!("duplicate CSV header '{header}'")));
        }
    }

    let mut out = Vec::new();
    for row in rows {
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

    let terminator = options.write_terminator()?;
    let mut builder = WriterBuilder::new();
    builder
        .has_headers(false)
        .flexible(true)
        .delimiter(options.separator)
        .quote(options.quote)
        .escape(options.escape)
        .double_quote(options.escape == options.quote)
        .terminator(terminator);
    let mut writer = builder.from_writer(Vec::new());
    for record in records {
        writer
            .write_record(record)
            .map_err(|error| DwError::new(format!("cannot write CSV: {error}")))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| DwError::new(format!("cannot finish CSV: {error}")))?;
    String::from_utf8(bytes).map_err(|_| DwError::new("CSV output is not valid UTF-8"))
}
