// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runtime values produced by evaluating a script.

use std::cmp::Ordering;
use std::fmt;

use crate::number::Number;
use crate::temporal::{Date, DateTime, LocalDateTime, LocalTime, Period, Time, Zone};
use crate::types::TypeValue;

/// A runtime value.
///
/// Objects preserve insertion order and may contain duplicate keys.
#[derive(Clone, PartialEq)]
pub enum Value {
    /// The `null` value.
    Null,
    /// A boolean value.
    Bool(bool),
    /// An exact decimal number.
    Number(Number),
    /// A string value.
    String(String),
    /// An ordered array.
    Array(Vec<Value>),
    /// An ordered list of key/value entries.
    Object(Vec<(String, Value)>),
    Date(Date),
    DateTime(DateTime),
    LocalDateTime(LocalDateTime),
    LocalTime(LocalTime),
    Time(Time),
    TimeZone(Zone),
    Period(Period),
    Binary(Vec<u8>),
    Regex(String),
    Uri(String),
    Key(String),
    Type(Box<TypeValue>),
}

impl Value {
    /// Returns the DataWeave type name for this value.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "Null",
            Value::Bool(_) => "Boolean",
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Array(_) => "Array",
            Value::Object(_) => "Object",
            Value::Date(_) => "Date",
            Value::DateTime(_) => "DateTime",
            Value::LocalDateTime(_) => "LocalDateTime",
            Value::LocalTime(_) => "LocalTime",
            Value::Time(_) => "Time",
            Value::TimeZone(_) => "TimeZone",
            Value::Period(_) => "Period",
            Value::Binary(_) => "Binary",
            Value::Regex(_) => "Regex",
            Value::Uri(_) => "Uri",
            Value::Key(_) => "Key",
            Value::Type(_) => "Type",
        }
    }

    /// Reports whether this is [`Value::Null`].
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the number inside this value, if it is a number.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the text inside this value, if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the bytes inside this value, if it is binary.
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Value::Binary(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Returns the elements inside this value, if it is an array.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(items) => Some(items),
            _ => None,
        }
    }

    /// Returns the entries inside this value, if it is an object.
    pub fn as_object(&self) -> Option<&[(String, Value)]> {
        match self {
            Value::Object(entries) => Some(entries),
            _ => None,
        }
    }

    /// Looks up `key` in an object, returning the last value for duplicate keys.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(entries) => entries.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Reads a temporal component by name.
    pub fn temporal_field(&self, key: &str) -> Option<Value> {
        use chrono::Datelike;
        let number = |value: i64| Value::Number(Number::from_i64(value));
        let (date, time, zone) = match self {
            Value::Date(date) => (Some(*date), None, None),
            Value::DateTime(value) => (Some(value.date), Some(&value.time), Some(&value.zone)),
            Value::LocalDateTime(value) => (Some(value.date), Some(&value.time), None),
            Value::Time(value) => (None, Some(&value.time), Some(&value.zone)),
            Value::LocalTime(value) => (None, Some(value), None),
            _ => (None, None, None),
        };
        if let Some(date) = date {
            if let Some(naive) = chrono::NaiveDate::from_ymd_opt(date.year, date.month, date.day) {
                match key {
                    "year" => return Some(number(naive.year() as i64)),
                    "month" => return Some(number(naive.month() as i64)),
                    "day" => return Some(number(naive.day() as i64)),
                    "quarter" => return Some(number(((naive.month() - 1) / 3 + 1) as i64)),
                    "dayOfWeek" => {
                        return Some(number(naive.weekday().num_days_from_monday() as i64 + 1))
                    }
                    "dayOfYear" => return Some(number(naive.ordinal() as i64)),
                    _ => {}
                }
            }
        }
        if let Some(time) = time {
            match key {
                "hour" => return Some(number(time.hour as i64)),
                "minutes" | "minute" => return Some(number(time.minute as i64)),
                "seconds" | "second" => return Some(number(time.second as i64)),
                "milliseconds" => return Some(number((time.nano / 1_000_000) as i64)),
                "nanoseconds" => return Some(number(time.nano as i64)),
                "offsetSeconds" => {
                    return zone.map(|zone| number(zone.seconds() as i64));
                }
                _ => {}
            }
        }
        if let Value::Period(period) = self {
            match key {
                "years" => return Some(number(period.years)),
                "months" => return Some(number(period.months)),
                "days" => return Some(number(period.days)),
                "hours" => {
                    let seconds = period.time_seconds.to_f64().unwrap_or(0.0);
                    return Some(number(((seconds / 3600.0).trunc() as i64) % 24));
                }
                "minutes" => {
                    let seconds = period.time_seconds.to_f64().unwrap_or(0.0);
                    return Some(number(((seconds % 3600.0) / 60.0).trunc() as i64));
                }
                "seconds" | "secs" => {
                    let seconds = period.time_seconds.to_f64().unwrap_or(0.0);
                    return Some(Value::Number(
                        Number::from_f64(seconds % 60.0).unwrap_or_else(Number::zero),
                    ));
                }
                "nanos" => {
                    let seconds = period.time_seconds.to_f64().unwrap_or(0.0);
                    return Some(number((seconds.fract() * 1_000_000_000.0).round() as i64));
                }
                _ => {}
            }
        }
        None
    }

    /// Returns the element at `index`, or `null` when out of range.
    ///
    /// Negative indexes count from the end. Arrays and strings are indexable.
    pub fn get_index(&self, index: i64) -> Option<Value> {
        match self {
            Value::Array(items) => {
                let len = items.len() as i64;
                let index = if index < 0 { len + index } else { index };
                if index >= 0 && index < len {
                    items.get(index as usize).cloned()
                } else {
                    Some(Value::Null)
                }
            }
            Value::String(text) => {
                let chars: Vec<char> = text.chars().collect();
                let len = chars.len() as i64;
                let index = if index < 0 { len + index } else { index };
                if index >= 0 && index < len {
                    Some(Value::String(chars[index as usize].to_string()))
                } else {
                    Some(Value::Null)
                }
            }
            Value::Null => Some(Value::Null),
            _ => None,
        }
    }

    /// Inserts or replaces an object entry.
    ///
    /// Replaces the first entry with `key`, or appends a new entry. Does
    /// nothing when the value is not an object.
    pub fn insert(&mut self, key: String, value: Value) {
        if let Value::Object(entries) = self {
            if let Some(slot) = entries.iter_mut().find(|(k, _)| *k == key) {
                slot.1 = value;
            } else {
                entries.push((key, value));
            }
        }
    }

    /// Creates an object from ordered key/value entries.
    pub fn object(entries: Vec<(String, Value)>) -> Self {
        Value::Object(entries)
    }

    /// Returns the length of an array or object, or the character count of a string.
    pub fn len(&self) -> Option<usize> {
        match self {
            Value::Array(items) => Some(items.len()),
            Value::Object(entries) => Some(entries.len()),
            Value::String(text) => Some(text.chars().count()),
            Value::Binary(bytes) => Some(bytes.len()),
            _ => None,
        }
    }

    /// Reports whether the value is null or an empty array, object, or string.
    pub fn is_empty(&self) -> bool {
        match self {
            Value::Null => true,
            Value::String(text) => text.is_empty(),
            Value::Array(items) => items.is_empty(),
            Value::Object(entries) => entries.is_empty(),
            Value::Binary(bytes) => bytes.is_empty(),
            _ => false,
        }
    }

    /// Renders the value as text for object keys and string interpolation.
    pub fn key_string(&self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(value) => value.to_string(),
            Value::Number(number) => number.to_string(),
            Value::String(text) => text.clone(),
            Value::Date(date) => crate::temporal::format_date(date),
            Value::DateTime(date) => crate::temporal::format_date_time(date),
            Value::LocalDateTime(date) => crate::temporal::format_local_date_time(date),
            Value::LocalTime(time) => crate::temporal::format_local_time(time),
            Value::Time(time) => crate::temporal::format_time(time),
            Value::TimeZone(zone) => zone.label(),
            Value::Period(period) => crate::temporal::format_period(period),
            Value::Binary(bytes) => crate::temporal::base64_encode(bytes),
            Value::Regex(pattern) => pattern.clone(),
            Value::Uri(text) | Value::Key(text) => text.clone(),
            Value::Type(value) => value.display(),
            _ => crate::json::write(self),
        }
    }

    /// Returns whether the value is truthy in a condition.
    pub fn truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(value) => *value,
            Value::Number(number) => !number.is_zero(),
            Value::String(text) => !text.is_empty(),
            Value::Array(items) => !items.is_empty(),
            Value::Object(entries) => !entries.is_empty(),
            Value::Binary(bytes) => !bytes.is_empty(),
            Value::Period(period) => {
                period.years != 0
                    || period.months != 0
                    || period.days != 0
                    || !period.time_seconds.is_zero()
            }
            _ => true,
        }
    }

    /// Compares values structurally, with numbers compared by numeric value.
    pub fn deep_eq(&self, other: &Value) -> bool {
        self == other
    }

    /// Orders two values when they are comparable.
    ///
    /// Returns `None` for values that have no ordering, such as mixed types.
    pub fn compare(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
            (Value::Number(a), Value::Number(b)) => Some(a.cmp(b)),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::Array(a), Value::Array(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    match x.compare(y)? {
                        Ordering::Equal => {}
                        other => return Some(other),
                    }
                }
                Some(a.len().cmp(&b.len()))
            }
            (Value::Object(a), Value::Object(b)) => Some(a.len().cmp(&b.len())),
            (Value::Date(a), Value::Date(b)) => a.partial_cmp(b),
            (Value::LocalTime(a), Value::LocalTime(b)) => a.partial_cmp(b),
            (Value::Time(a), Value::Time(b)) => a.partial_cmp(b),
            (Value::LocalDateTime(a), Value::LocalDateTime(b)) => a.partial_cmp(b),
            (Value::DateTime(a), Value::DateTime(b)) => a.partial_cmp(b),
            (Value::Null, _) => Some(Ordering::Less),
            (_, Value::Null) => Some(Ordering::Greater),
            _ => None,
        }
    }

    /// Returns the boolean inside this value, if it is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

impl fmt::Debug for Value {
    /// Formats a debug representation of the value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(text) => write!(f, "{text:?}"),
            Value::Array(items) => f.debug_list().entries(items).finish(),
            Value::Object(entries) => f
                .debug_map()
                .entries(entries.iter().map(|(k, v)| (k, v)))
                .finish(),
            Value::Date(date) => f.write_str(&crate::temporal::format_date(date)),
            Value::DateTime(date) => f.write_str(&crate::temporal::format_date_time(date)),
            Value::LocalDateTime(date) => {
                f.write_str(&crate::temporal::format_local_date_time(date))
            }
            Value::LocalTime(time) => f.write_str(&crate::temporal::format_local_time(time)),
            Value::Time(time) => f.write_str(&crate::temporal::format_time(time)),
            Value::TimeZone(zone) => f.write_str(&zone.label()),
            Value::Period(period) => f.write_str(&crate::temporal::format_period(period)),
            Value::Binary(bytes) => write!(f, "{:?}", crate::temporal::base64_encode(bytes)),
            Value::Regex(pattern) => write!(f, "/{pattern}/"),
            Value::Uri(text) | Value::Key(text) => write!(f, "{text:?}"),
            Value::Type(value) => write!(f, "{value}"),
        }
    }
}

impl From<bool> for Value {
    /// Converts a boolean to a value.
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    /// Converts an owned string to a value.
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    /// Converts a borrowed string to a value.
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<i64> for Value {
    /// Converts a signed integer to a numeric value.
    fn from(value: i64) -> Self {
        Value::Number(Number::from_i64(value))
    }
}

impl From<usize> for Value {
    /// Converts an unsigned size to a numeric value.
    fn from(value: usize) -> Self {
        Value::Number(Number::from_usize(value))
    }
}

impl From<Vec<Value>> for Value {
    /// Converts a vector to an array value.
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}
