// SPDX-License-Identifier: MIT OR Apache-2.0
//! JSON input parsing and pretty-printed output.
//!
//! Input accepts standard JSON. Output uses two-space indentation, one element
//! per line, and a space after each colon.

use crate::error::DwError;
use crate::number::Number;
use crate::value::Value;

/// Parses a JSON document into a [`Value`].
///
/// # Errors
///
/// Returns [`DwError`] when the text is not a complete, valid JSON document.
///
/// ```
/// let value = rustweave_core::json::parse(r#"{"a":1}"#).unwrap();
/// assert_eq!(rustweave_core::json::write(&value), "{\n  \"a\": 1\n}");
/// ```
pub fn parse(text: &str) -> Result<Value, DwError> {
    let mut parser = JsonParser {
        input: text.as_bytes(),
        pos: 0,
    };
    parser.skip_whitespace();
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    if parser.pos != parser.input.len() {
        return Err(DwError::new("invalid JSON: trailing characters"));
    }
    Ok(value)
}

/// Writes a [`Value`] as pretty-printed JSON.
///
/// ```
/// assert_eq!(rustweave_core::json::write(&rustweave_core::Value::Array(vec![rustweave_core::Value::Null])), "[\n  null\n]");
/// ```
pub fn write(value: &Value) -> String {
    let mut out = String::new();
    write_value(value, 0, &mut out);
    out
}

/// Writes one value at the requested indentation level.
fn write_value(value: &Value, level: usize, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => out.push_str(&number.to_string()),
        Value::String(text) => write_string(text, out),
        Value::Array(items) => {
            out.push('[');
            out.push('\n');
            if items.is_empty() {
                indent(level + 1, out);
                out.push('\n');
            } else {
                for (index, item) in items.iter().enumerate() {
                    indent(level + 1, out);
                    write_value(item, level + 1, out);
                    if index + 1 < items.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
            }
            indent(level, out);
            out.push(']');
        }
        Value::Object(entries) => {
            out.push('{');
            out.push('\n');
            if entries.is_empty() {
                indent(level + 1, out);
                out.push('\n');
            } else {
                for (index, (key, item)) in entries.iter().enumerate() {
                    indent(level + 1, out);
                    write_string(key, out);
                    out.push_str(": ");
                    write_value(item, level + 1, out);
                    if index + 1 < entries.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
            }
            indent(level, out);
            out.push('}');
        }
    }
}

/// Appends two spaces per indentation level.
fn indent(level: usize, out: &mut String) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

/// Appends a JSON-escaped string literal.
fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Cursor over the bytes of a JSON document.
struct JsonParser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    /// Advances past spaces, tabs, carriage returns, and newlines.
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.get(self.pos) {
            if matches!(c, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Parses the JSON value at the cursor.
    fn parse_value(&mut self) -> Result<Value, DwError> {
        self.skip_whitespace();
        match self.input.get(self.pos).copied() {
            Some(b'n') => {
                self.expect_word(b"null")?;
                Ok(Value::Null)
            }
            Some(b't') => {
                self.expect_word(b"true")?;
                Ok(Value::Bool(true))
            }
            Some(b'f') => {
                self.expect_word(b"false")?;
                Ok(Value::Bool(false))
            }
            Some(b'"') => Ok(Value::String(self.parse_string()?)),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            Some(b'-') | Some(b'0'..=b'9') => self.parse_number(),
            _ => Err(DwError::new("invalid JSON value")),
        }
    }

    /// Consumes the exact ASCII literal `word`.
    fn expect_word(&mut self, word: &[u8]) -> Result<(), DwError> {
        if self.input.len() >= self.pos + word.len()
            && &self.input[self.pos..self.pos + word.len()] == word
        {
            self.pos += word.len();
            Ok(())
        } else {
            Err(DwError::new("invalid JSON literal"))
        }
    }

    /// Parses a JSON number into an exact decimal.
    fn parse_number(&mut self) -> Result<Value, DwError> {
        let start = self.pos;
        if self.input.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        match self.input.get(self.pos) {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(DwError::new("invalid JSON number")),
        }
        if self.input.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            if !matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                return Err(DwError::new("invalid JSON number"));
            }
            while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.input.get(self.pos), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.input.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                return Err(DwError::new("invalid JSON number"));
            }
            while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        let text = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|_| DwError::new("invalid JSON number"))?;
        Number::parse(text)
            .map(Value::Number)
            .ok_or_else(|| DwError::new("invalid JSON number"))
    }

    /// Parses a quoted JSON string, including escape sequences.
    fn parse_string(&mut self) -> Result<String, DwError> {
        if self.input.get(self.pos) != Some(&b'"') {
            return Err(DwError::new("expected string"));
        }
        self.pos += 1;
        let mut out = String::new();
        loop {
            let c = self
                .input
                .get(self.pos)
                .copied()
                .ok_or_else(|| DwError::new("unterminated string"))?;
            self.pos += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    let escape = self
                        .input
                        .get(self.pos)
                        .copied()
                        .ok_or_else(|| DwError::new("unterminated escape"))?;
                    self.pos += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let first = self.parse_hex4()?;
                            if (0xD800..=0xDBFF).contains(&first) {
                                if self.input.get(self.pos) == Some(&b'\\')
                                    && self.input.get(self.pos + 1) == Some(&b'u')
                                {
                                    self.pos += 2;
                                    let second = self.parse_hex4()?;
                                    if (0xDC00..=0xDFFF).contains(&second) {
                                        let code = 0x10000
                                            + (((first - 0xD800) as u32) << 10)
                                            + (second - 0xDC00) as u32;
                                        if let Some(ch) = char::from_u32(code) {
                                            out.push(ch);
                                            continue;
                                        }
                                    }
                                }
                                return Err(DwError::new("invalid unicode escape"));
                            }
                            if (0xDC00..=0xDFFF).contains(&first) {
                                return Err(DwError::new("invalid unicode escape"));
                            }
                            if let Some(ch) = char::from_u32(first as u32) {
                                out.push(ch);
                            } else {
                                return Err(DwError::new("invalid unicode escape"));
                            }
                        }
                        _ => return Err(DwError::new("invalid escape")),
                    }
                }
                c if c < 0x20 => return Err(DwError::new("control character in string")),
                c if c < 0x80 => out.push(c as char),
                _ => {
                    self.pos -= 1;
                    let rest = &self.input[self.pos..];
                    let text =
                        std::str::from_utf8(rest).map_err(|_| DwError::new("invalid UTF-8"))?;
                    let ch = text
                        .chars()
                        .next()
                        .ok_or_else(|| DwError::new("invalid UTF-8"))?;
                    out.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
    }

    /// Reads four hexadecimal digits for a `\u` escape.
    fn parse_hex4(&mut self) -> Result<u16, DwError> {
        let mut value: u16 = 0;
        for _ in 0..4 {
            let c = self
                .input
                .get(self.pos)
                .copied()
                .ok_or_else(|| DwError::new("invalid unicode escape"))?;
            self.pos += 1;
            let digit = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => return Err(DwError::new("invalid unicode escape")),
            };
            value = value * 16 + digit as u16;
        }
        Ok(value)
    }

    /// Parses a JSON array.
    fn parse_array(&mut self) -> Result<Value, DwError> {
        self.pos += 1;
        self.skip_whitespace();
        let mut items = Vec::new();
        if self.input.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Value::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.input.get(self.pos).copied() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Value::Array(items));
                }
                _ => return Err(DwError::new("invalid JSON array")),
            }
        }
    }

    /// Parses a JSON object, keeping the last value for duplicate keys.
    fn parse_object(&mut self) -> Result<Value, DwError> {
        self.pos += 1;
        self.skip_whitespace();
        let mut entries = Vec::new();
        if self.input.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Value::Object(entries));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.input.get(self.pos) != Some(&b':') {
                return Err(DwError::new("invalid JSON object"));
            }
            self.pos += 1;
            let value = self.parse_value()?;
            if let Some(slot) = entries.iter_mut().find(|entry| entry.0 == key) {
                slot.1 = value;
            } else {
                entries.push((key, value));
            }
            self.skip_whitespace();
            match self.input.get(self.pos).copied() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Value::Object(entries));
                }
                _ => return Err(DwError::new("invalid JSON object")),
            }
        }
    }
}
