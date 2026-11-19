// SPDX-License-Identifier: MIT OR Apache-2.0
//! Standard-library functions for arrays, objects, strings, numbers, and core operations.
//!
//! [`builtin_name`] reports which names are available, [`lambda_positions`] lists
//! the arguments that accept a function, and [`call`] dispatches by name.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::{SystemTime, UNIX_EPOCH};

use num_bigint::BigUint;
use regex::Regex;

use crate::error::DwError;
use crate::eval;
use crate::number::Number;
use crate::runtime::RValue;
use crate::value::Value;

const NAMES: &[&str] = &[
    "map",
    "filter",
    "reduce",
    "groupBy",
    "sizeOf",
    "isEmpty",
    "sum",
    "mapObject",
    "pluck",
    "filterObject",
    "flatMap",
    "entriesOf",
    "namesOf",
    "keysOf",
    "valuesOf",
    "flatten",
    "distinctBy",
    "orderBy",
    "find",
    "indexOf",
    "lastIndexOf",
    "contains",
    "to",
    "zip",
    "unzip",
    "upper",
    "lower",
    "trim",
    "startsWith",
    "endsWith",
    "splitBy",
    "with",
    "replace",
    "joinBy",
    "abs",
    "ceil",
    "floor",
    "round",
    "pow",
    "sqrt",
    "mod",
    "isInteger",
    "isDecimal",
    "isOdd",
    "isEven",
    "isBlank",
    "minBy",
    "maxBy",
    "max",
    "min",
    "avg",
    "then",
    "onNull",
    "matches",
    "match",
    "scan",
    "typeOf",
    "random",
    "randomInt",
    "uuid",
    "log",
    "countBy",
    "sumBy",
    "some",
    "every",
    "take",
    "drop",
    "slice",
    "indexWhere",
    "splitAt",
    "takeWhile",
    "dropWhile",
    "partition",
    "firstWith",
    "divideBy",
    "splitWhere",
    "join",
    "leftJoin",
    "outerJoin",
    "appendIfMissing",
    "camelize",
    "capitalize",
    "charCode",
    "charCodeAt",
    "collapse",
    "countCharactersBy",
    "countMatches",
    "dasherize",
    "everyCharacter",
    "first",
    "fromCharCode",
    "hammingDistance",
    "isAlpha",
    "isAlphanumeric",
    "isLowerCase",
    "isNumeric",
    "isUpperCase",
    "isWhitespace",
    "last",
    "leftPad",
    "levenshteinDistance",
    "lines",
    "mapString",
    "ordinalize",
    "pluralize",
    "prependIfMissing",
    "remove",
    "repeat",
    "replaceAll",
    "reverse",
    "rightPad",
    "singularize",
    "someCharacter",
    "substring",
    "substringAfter",
    "substringAfterLast",
    "substringBefore",
    "substringBeforeLast",
    "substringBy",
    "substringEvery",
    "underscore",
    "unwrap",
    "withMaxSize",
    "words",
    "wrapIfMissing",
    "wrapWith",
    "entrySet",
    "nameSet",
    "keySet",
    "valueSet",
    "everyEntry",
    "someEntry",
    "mergeWith",
    "toHex",
    "toBinary",
    "fromHex",
    "fromBinary",
    "fromRadixNumber",
    "toRadixNumber",
    "atBeginningOfDay",
    "atBeginningOfHour",
    "atBeginningOfMonth",
    "atBeginningOfWeek",
    "atBeginningOfYear",
    "date",
    "dateTime",
    "localDateTime",
    "localTime",
    "time",
    "today",
    "tomorrow",
    "yesterday",
    "between",
    "days",
    "duration",
    "hours",
    "minutes",
    "months",
    "period",
    "seconds",
    "years",
    "daysBetween",
    "isLeapYear",
    "now",
    "asExpressionString",
    "filterArrayLeafs",
    "filterObjectLeafs",
    "filterTree",
    "isArrayType",
    "isAttributeType",
    "isObjectType",
    "mapLeafValues",
    "nodeExists",
    "read",
    "write",
    "readUrl",
    "concatWith",
    "fromBase64",
    "fromHex",
    "readLinesWith",
    "toBase64",
    "toHex",
    "writeLinesWith",
];

/// Returns the canonical built-in name for a function name.
pub fn builtin_name(name: &str) -> Option<&'static str> {
    NAMES.iter().copied().find(|candidate| *candidate == name)
}

/// Returns local name aliases for functions from an imported module.
pub fn module_aliases(module: &str) -> &'static [(&'static str, &'static str)] {
    match module {
        "dw::core::Binaries" => &[("fromHex", "binariesFromHex"), ("toHex", "binariesToHex")],
        _ => &[],
    }
}

/// Returns the value of a built-in constant, if the name is one.
pub fn constant(name: &str) -> Option<Value> {
    match name {
        "ARRAY_TYPE" => Some(Value::String("Array".to_string())),
        "ATTRIBUTE_TYPE" => Some(Value::String("Attribute".to_string())),
        "OBJECT_TYPE" => Some(Value::String("Object".to_string())),
        _ => None,
    }
}

/// Returns the argument positions that accept a lambda.
pub fn lambda_positions(name: &str) -> &'static [usize] {
    match name {
        "map" | "filter" | "reduce" | "groupBy" | "mapObject" | "pluck" | "filterObject"
        | "flatMap" | "distinctBy" | "orderBy" | "find" | "minBy" | "maxBy" | "then" | "onNull"
        | "scan" | "countBy" | "sumBy" | "some" | "every" | "indexWhere" | "takeWhile"
        | "dropWhile" | "partition" | "firstWith" | "splitWhere" | "everyEntry" | "someEntry"
        | "countCharactersBy" | "everyCharacter" | "someCharacter" | "mapString"
        | "substringBy" | "filterArrayLeafs" | "filterObjectLeafs" | "filterTree"
        | "mapLeafValues" | "nodeExists" | "sort" => &[1],
        "join" | "leftJoin" | "outerJoin" => &[2, 3],
        _ => &[],
    }
}

/// Dispatches a standard-library function by name.
pub fn call(name: &str, args: &[RValue]) -> Result<Value, DwError> {
    match name {
        "map" => map(args),
        "filter" => filter(args),
        "reduce" => reduce(args),
        "groupBy" => group_by(args),
        "sizeOf" => size_of(args),
        "isEmpty" => Ok(Value::Bool(value(args, 0)?.is_empty())),
        "sum" => sum(args),
        "mapObject" => map_object(args),
        "pluck" => pluck(args),
        "filterObject" => filter_object(args),
        "flatMap" => flat_map(args),
        "entriesOf" => entries_of(args, true),
        "entrySet" => entries_of(args, true),
        "namesOf" | "keysOf" | "nameSet" | "keySet" => names_of(args),
        "valuesOf" | "valueSet" => values_of(args),
        "flatten" => Ok(Value::Array(flatten(value(args, 0)?))),
        "distinctBy" => distinct_by(args),
        "orderBy" => order_by(args),
        "find" => find(args),
        "indexOf" => index_of(args),
        "lastIndexOf" => last_index_of(args),
        "contains" => contains(args),
        "to" => range(args),
        "zip" => zip(args),
        "unzip" => unzip(args),
        "upper" => text_unary(args, |text| text.to_uppercase()),
        "lower" => text_unary(args, |text| text.to_lowercase()),
        "trim" => text_unary(args, |text| text.trim().to_string()),
        "startsWith" => Ok(Value::Bool(string(args, 0)?.starts_with(&string(args, 1)?))),
        "endsWith" => Ok(Value::Bool(string(args, 0)?.ends_with(&string(args, 1)?))),
        "splitBy" => split_by(args),
        "with" => with_helper(args),
        "replace" => replace_all(args),
        "replaceAll" => replace_all_literal(args),
        "joinBy" => join_by(args),
        "abs" => number_unary(args, |n| n.abs()),
        "ceil" => number_unary(args, |n| n.ceil()),
        "floor" => number_unary(args, |n| n.floor()),
        "round" => number_unary(args, |n| n.round()),
        "pow" => pow(args),
        "sqrt" => sqrt(args),
        "mod" => modulo(args),
        "isInteger" => Ok(Value::Bool(
            number_like(value(args, 0)?)
                .map(|n| n.is_integer())
                .unwrap_or(false),
        )),
        "isDecimal" => Ok(Value::Bool(
            number_like(value(args, 0)?)
                .map(|n| !n.is_integer())
                .unwrap_or(false),
        )),
        "isOdd" => Ok(Value::Bool(
            number_like(value(args, 0)?)
                .and_then(|n| n.to_i64())
                .map(|n| n % 2 != 0)
                .ok_or_else(|| DwError::new("expected an integer"))?,
        )),
        "isEven" => Ok(Value::Bool(
            number_like(value(args, 0)?)
                .and_then(|n| n.to_i64())
                .map(|n| n % 2 == 0)
                .ok_or_else(|| DwError::new("expected an integer"))?,
        )),
        "isBlank" => Ok(Value::Bool(is_blank(value(args, 0)?))),
        "minBy" => min_max_by(args, true),
        "maxBy" => min_max_by(args, false),
        "max" => min_max(args, false),
        "min" => min_max(args, true),
        "avg" => average(args),
        "then" => then(args),
        "onNull" => on_null(args),
        "matches" => matches_text(args),
        "match" => match_text(args),
        "scan" => scan(args),
        "typeOf" => Ok(Value::String(value(args, 0)?.type_name().to_string())),
        "random" => Ok(Value::Number(random_number())),
        "randomInt" => random_int(args),
        "uuid" => Ok(Value::String(uuid())),
        "log" => log(args),
        "atBeginningOfDay" => at_beginning(args, "day"),
        "atBeginningOfHour" => at_beginning(args, "hour"),
        "atBeginningOfMonth" => at_beginning(args, "month"),
        "atBeginningOfWeek" => at_beginning(args, "week"),
        "atBeginningOfYear" => at_beginning(args, "year"),
        "date" => date(args),
        "dateTime" => date_time(args),
        "localDateTime" => local_date_time(args),
        "localTime" => local_time(args),
        "time" => time(args),
        "today" => Ok(Value::Date(crate::temporal::today())),
        "tomorrow" => Ok(Value::Date(crate::temporal::today().add_days(1)?)),
        "yesterday" => Ok(Value::Date(crate::temporal::today().add_days(-1)?)),
        "between" => between(args),
        "days" => make_days(args),
        "duration" => duration(args),
        "hours" => make_duration(args, 3_600),
        "minutes" => make_duration(args, 60),
        "months" => make_date_period(args, "months"),
        "period" => period(args),
        "seconds" => make_duration(args, 1),
        "years" => make_date_period(args, "years"),
        "daysBetween" => days_between(args),
        "isLeapYear" => is_leap_year(args),
        "now" => Ok(Value::DateTime(crate::temporal::now())),
        "asExpressionString" => as_expression_string(args),
        "filterArrayLeafs" => filter_array_leafs(args),
        "filterObjectLeafs" => filter_object_leafs(args),
        "filterTree" => filter_tree(args),
        "isArrayType" => path_kind(args, "Array"),
        "isAttributeType" => path_kind(args, "Attribute"),
        "isObjectType" => path_kind(args, "Object"),
        "mapLeafValues" => map_leaf_values(args),
        "nodeExists" => node_exists(args),
        "read" => read_value(args),
        "write" => write_value(args),
        "readUrl" => read_url(args),
        "concatWith" => concat_with(args),
        "fromBase64" => Ok(Value::Binary(crate::temporal::base64_decode(&string(
            args, 0,
        )?)?)),
        "fromHex" => radix_number(args, 16, false),
        "binariesFromHex" => Ok(Value::Binary(crate::temporal::hex_decode(&string(
            args, 0,
        )?)?)),
        "binariesToHex" => Ok(Value::String(crate::temporal::hex_encode(
            value(args, 0)?
                .as_binary()
                .ok_or_else(|| DwError::new("expected Binary"))?,
        ))),
        "readLinesWith" => read_lines_with(args),
        "toBase64" => Ok(Value::String(crate::temporal::base64_encode(
            value(args, 0)?
                .as_binary()
                .ok_or_else(|| DwError::new("expected Binary"))?,
        ))),
        "toHex" => match value(args, 0)? {
            Value::Binary(bytes) => Ok(Value::String(crate::temporal::hex_encode(bytes))),
            _ => radix_string(args, 16, false),
        },
        "writeLinesWith" => write_lines_with(args),
        "countBy" => count_by(args),
        "sumBy" => sum_by(args),
        "some" => some(args),
        "every" => every(args),
        "take" => take(args),
        "drop" => drop_items(args),
        "slice" => slice(args),
        "indexWhere" => index_where(args),
        "splitAt" => split_at(args),
        "takeWhile" => take_while(args),
        "dropWhile" => drop_while(args),
        "partition" => partition(args),
        "firstWith" => first_with(args),
        "divideBy" => divide_by(args),
        "splitWhere" => split_where(args),
        "join" | "leftJoin" | "outerJoin" => join_arrays(name, args),
        "appendIfMissing" => {
            let text = string(args, 0)?;
            let suffix = string(args, 1)?;
            Ok(Value::String(if text.ends_with(&suffix) {
                text
            } else {
                format!("{text}{suffix}")
            }))
        }
        "prependIfMissing" => {
            let text = string(args, 0)?;
            let prefix = string(args, 1)?;
            Ok(Value::String(if text.starts_with(&prefix) {
                text
            } else {
                format!("{prefix}{text}")
            }))
        }
        "camelize" => text_unary(args, |text| camelize(&text)),
        "capitalize" => text_unary(args, |text| capitalize(&text)),
        "charCode" => {
            let text = string(args, 0)?;
            let ch = text
                .chars()
                .next()
                .ok_or_else(|| DwError::new("empty string"))?;
            Ok(Value::Number(Number::from_u64(ch as u64)))
        }
        "charCodeAt" => char_code_at(args),
        "collapse" => collapse(args),
        "countCharactersBy" => count_characters_by(args),
        "countMatches" => count_matches(args),
        "dasherize" => text_unary(args, |text| separate(&text, '-')),
        "everyCharacter" => every_character(args),
        "first" => first_or_last(args, true),
        "last" => first_or_last(args, false),
        "fromCharCode" => from_char_code(args),
        "hammingDistance" => hamming_distance(args),
        "isAlpha" => character_class(args, char::is_alphabetic),
        "isAlphanumeric" => character_class(args, char::is_alphanumeric),
        "isNumeric" => character_class(args, |c: char| c.is_ascii_digit()),
        "isWhitespace" => Ok(Value::Bool(
            string(args, 0)?.chars().all(char::is_whitespace),
        )),
        "isLowerCase" => Ok(Value::Bool(
            string(args, 0)?
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_lowercase())
                && string(args, 0)?.chars().any(|c| c.is_alphabetic()),
        )),
        "isUpperCase" => Ok(Value::Bool(
            string(args, 0)?
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
                && string(args, 0)?.chars().any(|c| c.is_alphabetic()),
        )),
        "leftPad" => pad(args, true),
        "rightPad" => pad(args, false),
        "levenshteinDistance" => levenshtein(args),
        "lines" => Ok(Value::Array(split_lines(&string(args, 0)?))),
        "mapString" => map_string(args),
        "ordinalize" => ordinalize(args),
        "pluralize" => text_unary(args, |text| pluralize(&text)),
        "singularize" => text_unary(args, |text| singularize(&text)),
        "remove" => {
            let text = string(args, 0)?;
            let target = string(args, 1)?;
            Ok(Value::String(text.replace(&target, "")))
        }
        "repeat" => repeat(args),
        "reverse" => text_unary(args, |text| text.chars().rev().collect()),
        "someCharacter" => some_character(args),
        "substring" => substring(args),
        "substringAfter" => substring_after(args, false, false),
        "substringAfterLast" => substring_after(args, true, false),
        "substringBefore" => substring_after(args, false, true),
        "substringBeforeLast" => substring_after(args, true, true),
        "substringBy" => substring_by(args),
        "substringEvery" => substring_every(args),
        "underscore" => text_unary(args, |text| separate(&text, '_')),
        "unwrap" => unwrap(args),
        "withMaxSize" => with_max_size(args),
        "words" => Ok(Value::Array(
            string(args, 0)?
                .split_whitespace()
                .map(|word| Value::String(word.to_string()))
                .collect(),
        )),
        "wrapIfMissing" => wrap_if_missing(args),
        "wrapWith" => wrap_with(args),
        "everyEntry" => every_entry(args),
        "someEntry" => some_entry(args),
        "mergeWith" => merge_objects(args),
        "toBinary" => radix_string(args, 2, false),
        "fromBinary" => radix_number(args, 2, false),
        "fromRadixNumber" => radix_number(args, 0, true),
        "toRadixNumber" => radix_string(args, 0, true),
        other => Err(DwError::new(format!("unknown function '{other}'"))),
    }
}

/// Returns argument `index` as a value.
fn value(args: &[RValue], index: usize) -> Result<&Value, DwError> {
    match args.get(index) {
        Some(RValue::Val(value)) => Ok(value),
        Some(_) => Err(DwError::new("expected a value")),
        None => Err(DwError::new("missing argument")),
    }
}

/// Returns argument `index` if it is present.
fn optional_value(args: &[RValue], index: usize) -> Option<&Value> {
    match args.get(index) {
        Some(RValue::Val(value)) => Some(value),
        _ => None,
    }
}

/// Returns argument `index` as a callable function.
fn function(args: &[RValue], index: usize) -> Result<&RValue, DwError> {
    match args.get(index) {
        Some(value @ RValue::Func(_)) | Some(value @ RValue::Builtin(_)) => Ok(value),
        Some(_) => Err(DwError::new("expected a function")),
        None => Err(DwError::new("missing function argument")),
    }
}

/// Returns argument `index` as text.
fn string(args: &[RValue], index: usize) -> Result<String, DwError> {
    match value(args, index)? {
        Value::String(text) => Ok(text.clone()),
        other => Err(DwError::new(format!(
            "expected String, found {}",
            other.type_name()
        ))),
    }
}

/// Returns argument `index` as an array of values.
fn array(args: &[RValue], index: usize) -> Result<Vec<Value>, DwError> {
    match value(args, index)? {
        Value::Array(items) => Ok(items.clone()),
        other => Err(DwError::new(format!(
            "expected Array, found {}",
            other.type_name()
        ))),
    }
}

/// Returns argument `index` as ordered object entries.
fn object(args: &[RValue], index: usize) -> Result<Vec<(String, Value)>, DwError> {
    match value(args, index)? {
        Value::Object(entries) => Ok(entries.clone()),
        other => Err(DwError::new(format!(
            "expected Object, found {}",
            other.type_name()
        ))),
    }
}

/// Converts a numeric value or numeric string to a number.
fn number_like(value: &Value) -> Option<Number> {
    match value {
        Value::Number(number) => Some(number.clone()),
        Value::String(text) => Number::parse(text),
        _ => None,
    }
}

/// Returns argument `index` as a number.
fn number(args: &[RValue], index: usize) -> Result<Number, DwError> {
    match value(args, index)? {
        Value::Number(number) => Ok(number.clone()),
        other => Err(DwError::new(format!(
            "expected Number, found {}",
            other.type_name()
        ))),
    }
}

/// Returns argument `index` as an `i64` integer.
fn integer(args: &[RValue], index: usize) -> Result<i64, DwError> {
    number(args, index)?
        .to_i64()
        .ok_or_else(|| DwError::new("expected an integer"))
}

/// Calls a function with owned data values.
fn call_fn(function: &RValue, args: Vec<Value>) -> Result<Value, DwError> {
    let args: Vec<RValue> = args.into_iter().map(RValue::Val).collect();
    eval::invoke(function, &args)
}

/// Calls a function with borrowed data values.
fn call_simple(function: &RValue, args: &[Value]) -> Result<Value, DwError> {
    let args: Vec<RValue> = args.iter().cloned().map(RValue::Val).collect();
    eval::invoke(function, &args)
}

/// Implements the `map` standard-library operation.
fn map(args: &[RValue]) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::with_capacity(items.len());
    for (index, item) in items.into_iter().enumerate() {
        out.push(call_fn(
            function,
            vec![item, Value::Number(Number::from_usize(index))],
        )?);
    }
    Ok(Value::Array(out))
}

/// Implements the `filter` standard-library operation.
fn filter(args: &[RValue]) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let keep = call_fn(
            function,
            vec![item.clone(), Value::Number(Number::from_usize(index))],
        )?;
        if keep.truthy() {
            out.push(item);
        }
    }
    Ok(Value::Array(out))
}

/// Implements the `reduce` standard-library operation.
fn reduce(args: &[RValue]) -> Result<Value, DwError> {
    if args.len() >= 3 {
        let mut acc = value(args, 0)?.clone();
        let items = array(args, 1)?;
        let function = function(args, 2)?;
        for item in items {
            acc = call_simple(function, &[item, acc])?;
        }
        return Ok(acc);
    }

    let function = function(args, 1)?;
    let initial = eval::default_value(function, 1)?;
    match value(args, 0)? {
        Value::Array(items) => {
            let mut iter = items.iter().cloned();
            let mut acc = match initial {
                Some(value) => value,
                None => match iter.next() {
                    Some(value) => value,
                    None => return Ok(Value::Null),
                },
            };
            for item in iter {
                acc = call_simple(function, &[item, acc])?;
            }
            Ok(acc)
        }
        Value::String(text) => {
            let mut iter = text.chars().map(|c| Value::String(c.to_string()));
            let mut acc = match initial {
                Some(value) => value,
                None => match iter.next() {
                    Some(value) => value,
                    None => return Ok(Value::String(String::new())),
                },
            };
            for item in iter {
                acc = call_simple(function, &[item, acc])?;
            }
            Ok(acc)
        }
        Value::Null => Ok(Value::Null),
        other => Err(DwError::new(format!("cannot reduce {}", other.type_name()))),
    }
}

/// Groups array items by a selector.
fn group_by(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut groups: Vec<(String, Value)> = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let key = call_fn(
            function,
            vec![item.clone(), Value::Number(Number::from_usize(index))],
        )?
        .key_string();
        if let Some(slot) = groups.iter_mut().find(|(k, _)| *k == key) {
            if let Value::Array(items) = &mut slot.1 {
                items.push(item);
            }
        } else {
            groups.push((key, Value::Array(vec![item])));
        }
    }
    Ok(Value::Object(groups))
}

/// Returns the size of a value.
fn size_of(args: &[RValue]) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let size = match value(args, 0)? {
        Value::Null => 0,
        Value::String(text) => text.chars().count(),
        Value::Array(items) => items.len(),
        Value::Object(entries) => entries.len(),
        Value::Number(number) => number.to_string().chars().count(),
        Value::Bool(value) => value.to_string().len(),
        other => crate::json::write(other).chars().count(),
    };
    Ok(Value::Number(Number::from_usize(size)))
}

/// Sums an array of numbers.
fn sum(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let mut total = Number::zero();
    for item in items {
        match item {
            Value::Number(number) => total = total.add(&number),
            other => return Err(DwError::new(format!("cannot sum {}", other.type_name()))),
        }
    }
    Ok(Value::Number(total))
}

/// Maps each object entry to a new object.
fn map_object(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let function = function(args, 1)?;
    let mut out: Vec<(String, Value)> = Vec::new();
    for (index, (key, item)) in entries.into_iter().enumerate() {
        let result = call_fn(
            function,
            vec![
                item,
                Value::String(key),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        match result {
            Value::Object(new_entries) => {
                for (new_key, new_value) in new_entries {
                    if let Some(slot) = out.iter_mut().find(|(k, _)| *k == new_key) {
                        slot.1 = new_value;
                    } else {
                        out.push((new_key, new_value));
                    }
                }
            }
            other => {
                return Err(DwError::new(format!(
                    "mapObject callback must return an Object, found {}",
                    other.type_name()
                )))
            }
        }
    }
    Ok(Value::Object(out))
}

/// Extracts values from object entries by key.
fn pluck(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::with_capacity(entries.len());
    for (index, (key, item)) in entries.into_iter().enumerate() {
        out.push(call_fn(
            function,
            vec![
                item,
                Value::String(key),
                Value::Number(Number::from_usize(index)),
            ],
        )?);
    }
    Ok(Value::Array(out))
}

/// Filters object entries by a predicate.
fn filter_object(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::new();
    for (index, (key, item)) in entries.into_iter().enumerate() {
        let keep = call_fn(
            function,
            vec![
                item.clone(),
                Value::String(key.clone()),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if keep.truthy() {
            out.push((key, item));
        }
    }
    Ok(Value::Object(out))
}

/// Maps each item and flattens the results.
fn flat_map(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        match call_fn(
            function,
            vec![item, Value::Number(Number::from_usize(index))],
        )? {
            Value::Array(nested) => out.extend(nested),
            other => out.push(other),
        }
    }
    Ok(Value::Array(out))
}

/// Converts object entries to an array.
fn entries_of(args: &[RValue], with_attributes: bool) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let out = entries
        .into_iter()
        .map(|(key, value)| {
            let mut entry = vec![
                ("key".to_string(), Value::String(key)),
                ("value".to_string(), value),
            ];
            if with_attributes {
                entry.push(("attributes".to_string(), Value::Object(Vec::new())));
            }
            Value::Object(entry)
        })
        .collect();
    Ok(Value::Array(out))
}

/// Returns the keys of an object as text.
fn names_of(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    Ok(Value::Array(
        entries
            .into_iter()
            .map(|(key, _)| Value::String(key))
            .collect(),
    ))
}

/// Returns the values of an object.
fn values_of(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    Ok(Value::Array(
        entries.into_iter().map(|(_, value)| value).collect(),
    ))
}

/// Recursively flattens nested arrays.
fn flatten(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(items) => {
            let mut out = Vec::new();
            for item in items {
                out.extend(flatten(item));
            }
            out
        }
        other => vec![other.clone()],
    }
}

/// Removes duplicate values using a selector.
fn distinct_by(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let key = call_fn(
            function,
            vec![item.clone(), Value::Number(Number::from_usize(index))],
        )?
        .key_string();
        if seen.insert(key) {
            out.push(item);
        }
    }
    Ok(Value::Array(out))
}

/// Sorts values using an optional selector.
fn order_by(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::Array(items) => {
            let mut items = items.clone();
            match args.get(1) {
                Some(function @ RValue::Func(_)) | Some(function @ RValue::Builtin(_)) => {
                    let mut keyed = Vec::with_capacity(items.len());
                    for (index, item) in items.into_iter().enumerate() {
                        let key = call_simple(
                            function,
                            &[item.clone(), Value::Number(Number::from_usize(index))],
                        )?;
                        keyed.push((key, item));
                    }
                    keyed.sort_by(|(a, _), (b, _)| a.compare(b).unwrap_or(Ordering::Equal));
                    Ok(Value::Array(
                        keyed.into_iter().map(|(_, item)| item).collect(),
                    ))
                }
                _ => {
                    items.sort_by(|a, b| a.compare(b).unwrap_or(Ordering::Equal));
                    Ok(Value::Array(items))
                }
            }
        }
        Value::Object(entries) => {
            let function = function(args, 1)?;
            let mut keyed = Vec::with_capacity(entries.len());
            for (key, item) in entries.iter().cloned() {
                let sort_key = call_simple(function, &[item.clone(), Value::String(key.clone())])?;
                keyed.push((sort_key, key, item));
            }
            keyed.sort_by(|(a, _, _), (b, _, _)| a.compare(b).unwrap_or(Ordering::Equal));
            Ok(Value::Object(
                keyed
                    .into_iter()
                    .map(|(_, key, item)| (key, item))
                    .collect(),
            ))
        }
        Value::Null => Ok(Value::Null),
        other => Err(DwError::new(format!("cannot order {}", other.type_name()))),
    }
}

/// Finds regex or text matches with their positions.
fn find(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::Array(items) => {
            let mut out = Vec::new();
            match args.get(1) {
                Some(function @ RValue::Func(_)) | Some(function @ RValue::Builtin(_)) => {
                    for (index, item) in items.iter().cloned().enumerate() {
                        if call_fn(
                            function,
                            vec![item, Value::Number(Number::from_usize(index))],
                        )?
                        .truthy()
                        {
                            out.push(Value::Number(Number::from_usize(index)));
                        }
                    }
                }
                Some(RValue::Val(needle)) => {
                    for (index, item) in items.iter().enumerate() {
                        if item == needle {
                            out.push(Value::Number(Number::from_usize(index)));
                        }
                    }
                }
                _ => return Err(DwError::new("find requires a value or function")),
            }
            Ok(Value::Array(out))
        }
        Value::String(text) => match args.get(1) {
            Some(RValue::Regex(pattern)) => {
                let regex =
                    Regex::new(pattern).map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
                let mut out = Vec::new();
                for captures in regex.captures_iter(text) {
                    let mut starts = Vec::new();
                    if let Some(full) = captures.get(0) {
                        starts.push(Value::Number(Number::from_usize(char_position(
                            text,
                            full.start(),
                        ))));
                    }
                    for group in captures.iter().skip(1) {
                        if let Some(group) = group {
                            starts.push(Value::Number(Number::from_usize(char_position(
                                text,
                                group.start(),
                            ))));
                        } else {
                            starts.push(Value::Number(Number::from_i64(-1)));
                        }
                    }
                    out.push(Value::Array(starts));
                }
                Ok(Value::Array(out))
            }
            Some(RValue::Val(Value::String(needle))) => {
                let mut out = Vec::new();
                if !needle.is_empty() {
                    let mut search_from = 0usize;
                    while let Some(relative) = text[search_from..].find(needle) {
                        let byte = search_from + relative;
                        out.push(Value::Number(Number::from_usize(char_position(text, byte))));
                        search_from = byte + needle.len();
                    }
                }
                Ok(Value::Array(out))
            }
            _ => Err(DwError::new("find requires a string or regular expression")),
        },
        Value::Null => Ok(Value::Null),
        other => Err(DwError::new(format!("cannot search {}", other.type_name()))),
    }
}

/// Returns argument `index` as a regex pattern string.
fn regex_pattern(args: &[RValue], index: usize) -> Result<String, DwError> {
    match args.get(index) {
        Some(RValue::Regex(pattern)) => Ok(pattern.clone()),
        Some(RValue::Val(Value::String(pattern))) => Ok(pattern.clone()),
        _ => Err(DwError::new("expected a regular expression")),
    }
}

/// Returns the character position for a byte offset.
fn char_position(text: &str, byte: usize) -> usize {
    text[..byte].chars().count()
}

/// Returns the first index of a value or substring.
fn index_of(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::String(text) => {
            let needle = string(args, 1)?;
            let index = text
                .find(&needle)
                .map(|byte_index| text[..byte_index].chars().count())
                .map(|index| index as i64)
                .unwrap_or(-1);
            Ok(Value::Number(Number::from_i64(index)))
        }
        Value::Array(items) => {
            let needle = value(args, 1)?;
            let index = items
                .iter()
                .position(|item| item == needle)
                .map(|index| index as i64)
                .unwrap_or(-1);
            Ok(Value::Number(Number::from_i64(index)))
        }
        other => Err(DwError::new(format!("cannot search {}", other.type_name()))),
    }
}

/// Returns the last index of a value or substring.
fn last_index_of(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::String(text) => {
            let needle = string(args, 1)?;
            let index = text
                .rfind(&needle)
                .map(|byte_index| text[..byte_index].chars().count())
                .map(|index| index as i64)
                .unwrap_or(-1);
            Ok(Value::Number(Number::from_i64(index)))
        }
        Value::Array(items) => {
            let needle = value(args, 1)?;
            let index = items
                .iter()
                .rposition(|item| item == needle)
                .map(|index| index as i64)
                .unwrap_or(-1);
            Ok(Value::Number(Number::from_i64(index)))
        }
        other => Err(DwError::new(format!("cannot search {}", other.type_name()))),
    }
}

/// Tests whether an array or text contains a value.
fn contains(args: &[RValue]) -> Result<Value, DwError> {
    let second = args
        .get(1)
        .ok_or_else(|| DwError::new("missing argument"))?;
    let result = match value(args, 0)? {
        Value::String(text) => match second {
            RValue::Val(Value::String(target)) => text.contains(target),
            RValue::Regex(pattern) => Regex::new(pattern)
                .map(|regex| regex.is_match(text))
                .unwrap_or(false),
            _ => false,
        },
        Value::Array(items) => match second {
            RValue::Val(needle) => items.iter().any(|item| item == needle),
            _ => false,
        },
        Value::Object(entries) => {
            let key = match second {
                RValue::Val(value) => value.key_string(),
                _ => return Ok(Value::Bool(false)),
            };
            entries.iter().any(|(k, _)| *k == key)
        }
        _ => false,
    };
    Ok(Value::Bool(result))
}

/// Builds an inclusive numeric range.
fn range(args: &[RValue]) -> Result<Value, DwError> {
    let start = integer(args, 0)?;
    let end = integer(args, 1)?;
    let mut out = Vec::new();
    if start <= end {
        for value in start..=end {
            out.push(Value::Number(Number::from_i64(value)));
        }
    } else {
        let mut value = start;
        while value >= end {
            out.push(Value::Number(Number::from_i64(value)));
            value -= 1;
        }
    }
    Ok(Value::Array(out))
}

/// Zips two arrays into pairs.
fn zip(args: &[RValue]) -> Result<Value, DwError> {
    let left = array(args, 0)?;
    let right = array(args, 1)?;
    let size = left.len().min(right.len());
    let mut out = Vec::with_capacity(size);
    for index in 0..size {
        out.push(Value::Array(vec![
            left[index].clone(),
            right[index].clone(),
        ]));
    }
    Ok(Value::Array(out))
}

/// Splits an array of pairs into two arrays.
fn unzip(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let mut rows: Vec<Vec<Value>> = Vec::with_capacity(items.len());
    let mut width: Option<usize> = None;
    for item in items {
        match item {
            Value::Array(row) => {
                width = Some(match width {
                    Some(current) => current.min(row.len()),
                    None => row.len(),
                });
                rows.push(row);
            }
            _ => return Err(DwError::new("unzip expects an array of arrays")),
        }
    }
    let width = width.unwrap_or(0);
    let mut out = vec![Vec::with_capacity(rows.len()); width];
    for row in &rows {
        for (index, value) in row.iter().take(width).enumerate() {
            out[index].push(value.clone());
        }
    }
    Ok(Value::Array(out.into_iter().map(Value::Array).collect()))
}

/// Applies a text transform to argument zero.
fn text_unary<F>(args: &[RValue], f: F) -> Result<Value, DwError>
where
    F: FnOnce(String) -> String,
{
    Ok(Value::String(f(string(args, 0)?)))
}

/// Splits text by a separator or regular expression.
fn split_by(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let mut parts: Vec<Value> = match args.get(1) {
        Some(RValue::Regex(pattern)) => {
            let regex =
                Regex::new(pattern).map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
            regex
                .split(&text)
                .map(|part| Value::String(part.to_string()))
                .collect()
        }
        Some(RValue::Val(Value::String(separator))) => {
            if separator.is_empty() {
                text.chars().map(|c| Value::String(c.to_string())).collect()
            } else {
                text.split(separator.as_str())
                    .map(|part| Value::String(part.to_string()))
                    .collect()
            }
        }
        _ => {
            return Err(DwError::new(
                "splitBy requires a string or regular expression",
            ))
        }
    };
    while parts.len() > 1 && matches!(parts.last(), Some(Value::String(text)) if text.is_empty()) {
        parts.pop();
    }
    Ok(Value::Array(parts))
}

/// Merges two objects, or returns the first value unchanged.
fn with_helper(args: &[RValue]) -> Result<Value, DwError> {
    if args.len() >= 2
        && matches!(value(args, 0)?, Value::Object(_))
        && matches!(value(args, 1)?, Value::Object(_))
    {
        return merge_objects(args);
    }
    value(args, 0).cloned()
}

/// Implements the `merge_objects` standard-library operation.
fn merge_objects(args: &[RValue]) -> Result<Value, DwError> {
    let mut entries = object(args, 0)?;
    let extra = object(args, 1)?;
    for (key, value) in extra {
        if let Some(slot) = entries.iter_mut().find(|(k, _)| *k == key) {
            slot.1 = value;
        } else {
            entries.push((key, value));
        }
    }
    Ok(Value::Object(entries))
}

/// Replaces regex matches with text or a function result.
fn replace_all(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    match args.get(1) {
        Some(RValue::Regex(pattern)) => {
            let regex =
                Regex::new(pattern).map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
            match args.get(2) {
                Some(function @ RValue::Func(_)) | Some(function @ RValue::Builtin(_)) => {
                    let mut out = String::new();
                    let mut last = 0usize;
                    for (index, captures) in regex.captures_iter(&text).enumerate() {
                        let full = captures
                            .get(0)
                            .ok_or_else(|| DwError::new("invalid match"))?;
                        out.push_str(&text[last..full.start()]);
                        let mut groups = vec![Value::String(full.as_str().to_string())];
                        for group in captures.iter().skip(1) {
                            groups.push(match group {
                                Some(matched) => Value::String(matched.as_str().to_string()),
                                None => Value::Null,
                            });
                        }
                        let replacement = call_fn(
                            function,
                            vec![
                                Value::Array(groups),
                                Value::Number(Number::from_usize(index)),
                            ],
                        )?;
                        out.push_str(&replacement.key_string());
                        last = full.end();
                    }
                    out.push_str(&text[last..]);
                    Ok(Value::String(out))
                }
                Some(RValue::Val(Value::String(replacement))) => Ok(Value::String(
                    regex
                        .replace_all(&text, regex::NoExpand(replacement))
                        .into_owned(),
                )),
                _ => Err(DwError::new("replace requires a replacement")),
            }
        }
        Some(RValue::Val(Value::String(target))) => {
            let replacement = string(args, 2)?;
            Ok(Value::String(text.replace(target, &replacement)))
        }
        _ => Err(DwError::new(
            "replace requires a string or regular expression",
        )),
    }
}

/// Replaces literal text occurrences.
fn replace_all_literal(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let target = string(args, 1)?;
    let replacement = string(args, 2)?;
    Ok(Value::String(text.replace(&target, &replacement)))
}

/// Implements the `join_by` standard-library operation.
fn join_by(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let separator = string(args, 1)?;
    let parts: Vec<String> = items.iter().map(Value::key_string).collect();
    Ok(Value::String(parts.join(&separator)))
}

/// Applies a numeric transform to argument zero.
fn number_unary<F>(args: &[RValue], f: F) -> Result<Value, DwError>
where
    F: FnOnce(&Number) -> Number,
{
    Ok(Value::Number(f(&number(args, 0)?)))
}

/// Implements the `pow` standard-library operation.
fn pow(args: &[RValue]) -> Result<Value, DwError> {
    let base = number(args, 0)?;
    let exponent = number(args, 1)?;
    base.pow(&exponent)
        .map(Value::Number)
        .ok_or_else(|| DwError::new("invalid power operation"))
}

/// Implements the `sqrt` standard-library operation.
fn sqrt(args: &[RValue]) -> Result<Value, DwError> {
    number(args, 0)?
        .sqrt()
        .map(Value::Number)
        .ok_or_else(|| DwError::new("square root of a negative number"))
}

/// Implements the `modulo` standard-library operation.
fn modulo(args: &[RValue]) -> Result<Value, DwError> {
    let left = number(args, 0)?;
    let right = number(args, 1)?;
    left.rem(&right)
        .map(Value::Number)
        .ok_or_else(|| DwError::new("division by zero"))
}

/// Reports whether a value is null or blank text.
fn is_blank(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.trim().is_empty(),
        Value::Array(items) => items.is_empty(),
        Value::Object(entries) => entries.is_empty(),
        _ => false,
    }
}

/// Finds the minimum or maximum item using a selector.
fn min_max_by(args: &[RValue], minimum: bool) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut best: Option<(Value, Value)> = None;
    for (index, item) in items.into_iter().enumerate() {
        let key = call_simple(
            function,
            &[item.clone(), Value::Number(Number::from_usize(index))],
        )?;
        best = match best {
            None => Some((key, item)),
            Some((best_key, best_item)) => {
                let ordering = key.compare(&best_key).unwrap_or(Ordering::Equal);
                let take = if minimum {
                    ordering.is_lt()
                } else {
                    ordering.is_gt()
                };
                if take {
                    Some((key, item))
                } else {
                    Some((best_key, best_item))
                }
            }
        };
    }
    Ok(best.map(|(_, item)| item).unwrap_or(Value::Null))
}

/// Finds the minimum or maximum value.
fn min_max(args: &[RValue], minimum: bool) -> Result<Value, DwError> {
    let items = if args.len() == 1 {
        array(args, 0)?
    } else {
        args.iter()
            .map(|arg| match arg {
                RValue::Val(value) => Ok(value.clone()),
                _ => Err(DwError::new("expected a value")),
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    let mut best: Option<Value> = None;
    for item in items {
        best = match best {
            None => Some(item),
            Some(current) => {
                let ordering = item.compare(&current).unwrap_or(Ordering::Equal);
                let take = if minimum {
                    ordering.is_lt()
                } else {
                    ordering.is_gt()
                };
                Some(if take { item } else { current })
            }
        };
    }
    Ok(best.unwrap_or(Value::Null))
}

/// Computes the arithmetic mean of an array of numbers.
fn average(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    if items.is_empty() {
        return Err(DwError::new("cannot average an empty array"));
    }
    let mut total = Number::zero();
    let count = Number::from_usize(items.len());
    for item in items {
        match item {
            Value::Number(number) => total = total.add(&number),
            other => {
                return Err(DwError::new(format!(
                    "cannot average {}",
                    other.type_name()
                )))
            }
        }
    }
    total
        .div(&count)
        .map(Value::Number)
        .ok_or_else(|| DwError::new("division by zero"))
}

/// Applies a function when the value is not null.
fn then(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?.clone();
    if value.is_null() {
        return Ok(Value::Null);
    }
    let function = function(args, 1)?;
    call_simple(function, &[value])
}

/// Applies a function when the value is null.
fn on_null(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?.clone();
    if !value.is_null() {
        return Ok(value);
    }
    match args.get(1) {
        Some(function @ RValue::Func(_)) | Some(function @ RValue::Builtin(_)) => {
            call_adaptive_null(function, value)
        }
        Some(RValue::Val(fallback)) => Ok(fallback.clone()),
        Some(_) => Err(DwError::new("invalid fallback for onNull")),
        None => Err(DwError::new("missing fallback for onNull")),
    }
}

/// Calls a null-handling function with a value.
fn call_adaptive_null(function: &RValue, value: Value) -> Result<Value, DwError> {
    if let RValue::Func(closure) = function {
        if closure.params.is_empty() {
            return call_fn(function, Vec::new());
        }
    }
    call_simple(function, &[value])
}

/// Implements `matches` for text and regex patterns.
fn matches_text(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let pattern = regex_pattern(args, 1)?;
    let regex = Regex::new(&format!("^(?:{pattern})$"))
        .map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
    Ok(Value::Bool(regex.is_match(&text)))
}

/// Implements `match` capture extraction.
fn match_text(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let pattern = regex_pattern(args, 1)?;
    let regex = Regex::new(&format!("^(?:{pattern})$"))
        .map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
    match regex.captures(&text) {
        Some(captures) => {
            let mut groups = Vec::new();
            if let Some(full) = captures.get(0) {
                groups.push(Value::String(full.as_str().to_string()));
            }
            for group in captures.iter().skip(1) {
                groups.push(match group {
                    Some(matched) => Value::String(matched.as_str().to_string()),
                    None => Value::Null,
                });
            }
            Ok(Value::Array(groups))
        }
        None => Ok(Value::Array(Vec::new())),
    }
}

/// Implements `scan` capture extraction for regex patterns.
fn scan(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::String(text) => {
            let pattern = regex_pattern(args, 1)?;
            let regex =
                Regex::new(&pattern).map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
            let mut out = Vec::new();
            for captures in regex.captures_iter(text) {
                let mut groups = Vec::new();
                if let Some(full) = captures.get(0) {
                    groups.push(Value::String(full.as_str().to_string()));
                }
                for group in captures.iter().skip(1) {
                    groups.push(match group {
                        Some(matched) => Value::String(matched.as_str().to_string()),
                        None => Value::Null,
                    });
                }
                out.push(Value::Array(groups));
            }
            Ok(Value::Array(out))
        }
        Value::Null => Ok(Value::Null),
        Value::Array(items) => {
            let items = items.clone();
            let function = function(args, 1)?;
            let mut iter = items.into_iter();
            let mut acc = match iter.next() {
                Some(value) => value,
                None => return Ok(Value::Array(Vec::new())),
            };
            let mut out = vec![acc.clone()];
            for item in iter {
                acc = call_simple(function, &[item, acc])?;
                out.push(acc.clone());
            }
            Ok(Value::Array(out))
        }
        other => Err(DwError::new(format!("cannot scan {}", other.type_name()))),
    }
}

static RANDOM_STATE: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);

/// Advances the pseudo-random state and returns raw bits.
fn next_random() -> u64 {
    let mut x = RANDOM_STATE.fetch_add(0x9e3779b97f4a7c15, AtomicOrdering::Relaxed);
    if x == 0 {
        x = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0x2545f4914f6cdd1d);
    }
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// Returns a pseudo-random number between zero and one.
fn random_number() -> Number {
    let value = next_random() >> 11;
    Number::from_f64(value as f64 / ((1u64 << 53) as f64)).unwrap_or_else(Number::zero)
}

/// Returns a pseudo-random integer in the requested range.
fn random_int(args: &[RValue]) -> Result<Value, DwError> {
    let limit = integer(args, 0)?;
    if limit <= 0 {
        return Ok(Value::Number(Number::from_i64(0)));
    }
    let value = (next_random() % limit as u64) as i64;
    Ok(Value::Number(Number::from_i64(value)))
}

/// Generates a random version-4 UUID string.
fn uuid() -> String {
    let a = next_random();
    let b = next_random();
    let bytes = [
        (a >> 56) as u8,
        (a >> 48) as u8,
        (a >> 40) as u8,
        (a >> 32) as u8,
        (a >> 24) as u8,
        (a >> 16) as u8,
        (a >> 8) as u8,
        a as u8,
        (b >> 56) as u8,
        (b >> 48) as u8,
        (b >> 40) as u8,
        (b >> 32) as u8,
        (b >> 24) as u8,
        (b >> 16) as u8,
        (b >> 8) as u8,
        b as u8,
    ];
    let mut bytes = bytes;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// Writes a value to standard error and returns it.
fn log(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?.clone();
    let prefix = optional_value(args, 1).map(Value::key_string);
    match prefix {
        Some(prefix) => eprintln!("{prefix}: {}", crate::json::write(&value)),
        None => eprintln!("{}", crate::json::write(&value)),
    }
    Ok(value)
}

/// Implements the `count_by` standard-library operation.
fn count_by(args: &[RValue]) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut count = 0usize;
    for (index, item) in items.into_iter().enumerate() {
        if call_simple(function, &[item, Value::Number(Number::from_usize(index))])?.truthy() {
            count += 1;
        }
    }
    Ok(Value::Number(Number::from_usize(count)))
}

/// Implements the `sum_by` standard-library operation.
fn sum_by(args: &[RValue]) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut total = Number::zero();
    for (index, item) in items.into_iter().enumerate() {
        match call_simple(function, &[item, Value::Number(Number::from_usize(index))])? {
            Value::Number(number) => total = total.add(&number),
            other => return Err(DwError::new(format!("cannot sum {}", other.type_name()))),
        }
    }
    Ok(Value::Number(total))
}

/// Implements the `some` standard-library operation.
fn some(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    for (index, item) in items.into_iter().enumerate() {
        if call_simple(function, &[item, Value::Number(Number::from_usize(index))])?.truthy() {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

/// Implements the `every` standard-library operation.
fn every(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    for (index, item) in items.into_iter().enumerate() {
        if !call_simple(function, &[item, Value::Number(Number::from_usize(index))])?.truthy() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

/// Implements the `take` standard-library operation.
fn take(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let count = integer(args, 1)?;
    if count <= 0 {
        return Ok(Value::Array(Vec::new()));
    }
    Ok(Value::Array(
        items.into_iter().take(count as usize).collect(),
    ))
}

/// Implements the `drop_items` standard-library operation.
fn drop_items(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let count = integer(args, 1)?.max(0) as usize;
    Ok(Value::Array(items.into_iter().skip(count).collect()))
}

/// Implements the `slice` standard-library operation.
fn slice(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let start = integer(args, 1)?.max(0) as usize;
    let end = integer(args, 2)?.max(0) as usize;
    if end <= start {
        return Ok(Value::Array(Vec::new()));
    }
    Ok(Value::Array(
        items.into_iter().skip(start).take(end - start).collect(),
    ))
}

/// Implements the `index_where` standard-library operation.
fn index_where(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    for (index, item) in items.into_iter().enumerate() {
        if call_fn(
            function,
            vec![item, Value::Number(Number::from_usize(index))],
        )?
        .truthy()
        {
            return Ok(Value::Number(Number::from_usize(index)));
        }
    }
    Ok(Value::Number(Number::from_i64(-1)))
}

/// Implements the `split_at` standard-library operation.
fn split_at(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let count = integer(args, 1)?.max(0) as usize;
    let count = count.min(items.len());
    let left: Vec<Value> = items[..count].to_vec();
    let right: Vec<Value> = items[count..].to_vec();
    Ok(Value::Object(vec![
        ("l".to_string(), Value::Array(left)),
        ("r".to_string(), Value::Array(right)),
    ]))
}

/// Takes elements while a predicate is true.
fn take_while(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::Array(items) => {
            let function = function(args, 1)?;
            let mut out = Vec::new();
            for (index, item) in items.iter().cloned().enumerate() {
                if !call_fn(
                    function,
                    vec![item.clone(), Value::Number(Number::from_usize(index))],
                )?
                .truthy()
                {
                    break;
                }
                out.push(item);
            }
            Ok(Value::Array(out))
        }
        Value::Object(entries) => {
            let function = function(args, 1)?;
            let mut out = Vec::new();
            for (index, (key, item)) in entries.iter().cloned().enumerate() {
                if !call_fn(
                    function,
                    vec![
                        item.clone(),
                        Value::String(key.clone()),
                        Value::Number(Number::from_usize(index)),
                    ],
                )?
                .truthy()
                {
                    break;
                }
                out.push((key, item));
            }
            Ok(Value::Object(out))
        }
        other => Err(DwError::new(format!(
            "cannot take from {}",
            other.type_name()
        ))),
    }
}

/// Drops elements while a predicate is true.
fn drop_while(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut index = 0usize;
    while index < items.len() {
        let result = call_simple(function, &[items[index].clone()])?;
        if !result.truthy() {
            break;
        }
        index += 1;
    }
    Ok(Value::Array(items.into_iter().skip(index).collect()))
}

/// Splits an array by a predicate.
fn partition(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut success = Vec::new();
    let mut failure = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let result = call_fn(
            function,
            vec![item.clone(), Value::Number(Number::from_usize(index))],
        )?;
        if result.truthy() {
            success.push(item);
        } else {
            failure.push(item);
        }
    }
    Ok(Value::Object(vec![
        ("success".to_string(), Value::Array(success)),
        ("failure".to_string(), Value::Array(failure)),
    ]))
}

/// Implements the `first_with` standard-library operation.
fn first_with(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    for item in items {
        if call_simple(function, std::slice::from_ref(&item))?.truthy() {
            return Ok(item);
        }
    }
    Ok(Value::Null)
}

/// Splits an array or object into chunks.
fn divide_by(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::Array(items) => {
            let size = integer(args, 1)?;
            if size <= 0 {
                return Err(DwError::new("divideBy size must be positive"));
            }
            let size = size as usize;
            let mut out = Vec::new();
            let mut chunk = Vec::new();
            for item in items.iter().cloned() {
                chunk.push(item);
                if chunk.len() == size {
                    out.push(Value::Array(std::mem::take(&mut chunk)));
                }
            }
            if !chunk.is_empty() {
                out.push(Value::Array(chunk));
            }
            Ok(Value::Array(out))
        }
        Value::Object(entries) => {
            let size = integer(args, 1)?;
            if size <= 0 {
                return Err(DwError::new("divideBy size must be positive"));
            }
            let size = size as usize;
            let mut out = Vec::new();
            let mut chunk: Vec<(String, Value)> = Vec::new();
            for (key, item) in entries.iter().cloned() {
                chunk.push((key, item));
                if chunk.len() == size {
                    out.push(Value::Object(std::mem::take(&mut chunk)));
                }
            }
            if !chunk.is_empty() {
                out.push(Value::Object(chunk));
            }
            Ok(Value::Array(out))
        }
        other => Err(DwError::new(format!("cannot divide {}", other.type_name()))),
    }
}

/// Splits an array at the first matching predicate.
fn split_where(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let include_split = optional_value(args, 2)
        .map(|value| value.truthy())
        .unwrap_or(true);
    let mut split_index = None;
    for (index, item) in items.iter().enumerate() {
        if call_simple(function, std::slice::from_ref(item))?.truthy() {
            split_index = Some(index);
            break;
        }
    }
    let left_end = split_index.unwrap_or(items.len());
    let right_start = match (split_index, include_split) {
        (Some(index), true) => index,
        (Some(index), false) => index + 1,
        (None, _) => items.len(),
    };
    let left: Vec<Value> = items[..left_end].to_vec();
    let right: Vec<Value> = items[right_start..].to_vec();
    Ok(Value::Object(vec![
        ("l".to_string(), Value::Array(left)),
        ("r".to_string(), Value::Array(right)),
    ]))
}

/// Implements inner, left, and outer array joins.
fn join_arrays(name: &str, args: &[RValue]) -> Result<Value, DwError> {
    let left = array(args, 0)?;
    let right = array(args, 1)?;
    let mut out = Vec::new();

    if args.len() >= 4 {
        let left_criteria = function(args, 2)?;
        let right_criteria = function(args, 3)?;
        let mut right_keys = Vec::with_capacity(right.len());
        for item in &right {
            right_keys.push(call_simple(right_criteria, std::slice::from_ref(item))?.key_string());
        }
        let mut right_matched = vec![false; right.len()];
        for left_item in &left {
            let left_key =
                call_simple(left_criteria, std::slice::from_ref(left_item))?.key_string();
            let mut matched = false;
            for (index, right_item) in right.iter().enumerate() {
                if right_keys[index] == left_key {
                    matched = true;
                    right_matched[index] = true;
                    out.push(join_pair(Some(left_item.clone()), Some(right_item.clone())));
                }
            }
            if !matched && name != "join" {
                out.push(join_pair(Some(left_item.clone()), None));
            }
        }
        if name == "outerJoin" {
            for (index, right_item) in right.iter().enumerate() {
                if !right_matched[index] {
                    out.push(join_pair(None, Some(right_item.clone())));
                }
            }
        }
        return Ok(Value::Array(out));
    }

    let predicate = function(args, 2)?;
    let mut right_matched = vec![false; right.len()];
    for left_item in &left {
        let mut matched = false;
        for (index, right_item) in right.iter().enumerate() {
            let result = call_simple(predicate, &[left_item.clone(), right_item.clone()])?;
            if result.truthy() {
                matched = true;
                right_matched[index] = true;
                out.push(join_pair(Some(left_item.clone()), Some(right_item.clone())));
            }
        }
        if !matched && name != "join" {
            out.push(join_pair(Some(left_item.clone()), None));
        }
    }
    if name == "outerJoin" {
        for (index, right_item) in right.iter().enumerate() {
            if !right_matched[index] {
                out.push(join_pair(None, Some(right_item.clone())));
            }
        }
    }
    Ok(Value::Array(out))
}

/// Builds a left/right join result pair.
fn join_pair(left: Option<Value>, right: Option<Value>) -> Value {
    let mut entries = Vec::new();
    if let Some(value) = left {
        entries.push(("l".to_string(), value));
    }
    if let Some(value) = right {
        entries.push(("r".to_string(), value));
    }
    Value::Object(entries)
}

/// Capitalizes each word in text.
fn capitalize(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut spaced = String::new();
    for (index, c) in chars.iter().copied().enumerate() {
        if c == '_' {
            spaced.push(' ');
            continue;
        }
        if c.is_uppercase() && index > 0 {
            let previous = chars[index - 1];
            if previous.is_lowercase() && !spaced.ends_with(' ') {
                spaced.push(' ');
            }
        }
        spaced.push(c);
    }
    let mut out = String::new();
    let mut at_word_start = true;
    for c in spaced.chars() {
        if c.is_alphabetic() {
            if at_word_start {
                out.extend(c.to_uppercase());
                at_word_start = false;
            } else {
                out.extend(c.to_lowercase());
            }
        } else {
            out.push(c);
            at_word_start = true;
        }
    }
    out
}

/// Converts separated words to camel case.
fn camelize(text: &str) -> String {
    let parts: Vec<&str> = text
        .split(['_', '-', ' '])
        .filter(|part| !part.is_empty())
        .collect();
    let mut out = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index == 0 {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_lowercase());
                out.push_str(chars.as_str());
            }
        } else {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
        }
    }
    out
}

/// Inserts separators into camel-case or mixed text.
fn separate(text: &str, separator: char) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    for (index, c) in chars.iter().copied().enumerate() {
        if c == ' ' || c == '-' || c == '_' {
            if !out.ends_with(separator) && !out.is_empty() {
                out.push(separator);
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if index > 0 {
                Some(chars[index - 1])
            } else {
                None
            };
            let next = chars.get(index + 1).copied();
            let boundary = prev
                .map(|p| p.is_lowercase() || p.is_ascii_digit())
                .unwrap_or(false)
                || (prev.map(|p| p.is_uppercase()).unwrap_or(false)
                    && next.map(|n| n.is_lowercase()).unwrap_or(false));
            if boundary && !out.is_empty() && !out.ends_with(separator) {
                out.push(separator);
            }
        }
        out.extend(c.to_lowercase());
    }
    out.trim_matches(separator).to_string()
}

/// Returns the character code at a position.
fn char_code_at(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let index = integer(args, 1)?.max(0) as usize;
    let ch = text
        .chars()
        .nth(index)
        .ok_or_else(|| DwError::new("string index out of range"))?;
    Ok(Value::Number(Number::from_u64(ch as u64)))
}

/// Groups adjacent equal characters.
fn collapse(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let mut out: Vec<Value> = Vec::new();
    let mut current = String::new();
    let mut last: Option<char> = None;
    for c in text.chars() {
        if last == Some(c) {
            current.push(c);
        } else {
            if !current.is_empty() {
                out.push(Value::String(std::mem::take(&mut current)));
            }
            current.push(c);
            last = Some(c);
        }
    }
    if !current.is_empty() {
        out.push(Value::String(current));
    }
    Ok(Value::Array(out))
}

/// Counts characters matching a predicate.
fn count_characters_by(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let function = function(args, 1)?;
    let mut count = 0usize;
    for (index, c) in text.chars().enumerate() {
        let result = call_fn(
            function,
            vec![
                Value::String(c.to_string()),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if result.truthy() {
            count += 1;
        }
    }
    Ok(Value::Number(Number::from_usize(count)))
}

/// Counts matches of text or a regex.
fn count_matches(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let pattern = regex_pattern(args, 1)?;
    let regex = Regex::new(&pattern).map_err(|e| DwError::new(format!("invalid regex: {e}")))?;
    Ok(Value::Number(Number::from_usize(
        regex.find_iter(&text).count(),
    )))
}

/// Tests whether every character matches a predicate.
fn every_character(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let function = function(args, 1)?;
    for (index, c) in text.chars().enumerate() {
        let result = call_fn(
            function,
            vec![
                Value::String(c.to_string()),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if !result.truthy() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

/// Tests whether any character matches a predicate.
fn some_character(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let function = function(args, 1)?;
    for (index, c) in text.chars().enumerate() {
        let result = call_fn(
            function,
            vec![
                Value::String(c.to_string()),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if result.truthy() {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

/// Returns the first or last characters of text.
fn first_or_last(args: &[RValue], first: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let count = optional_value(args, 1)
        .map(|value| match value {
            Value::Number(number) => number.to_i64().unwrap_or(1),
            _ => 1,
        })
        .unwrap_or(1)
        .max(0) as usize;
    let chars: Vec<char> = text.chars().collect();
    let selected: String = if first {
        chars.into_iter().take(count).collect()
    } else {
        let skip = chars.len().saturating_sub(count);
        chars.into_iter().skip(skip).collect()
    };
    Ok(Value::String(selected))
}

/// Converts a character code to text.
fn from_char_code(args: &[RValue]) -> Result<Value, DwError> {
    let code = integer(args, 0)?;
    let ch = u32::try_from(code)
        .ok()
        .and_then(char::from_u32)
        .ok_or_else(|| DwError::new("invalid character code"))?;
    Ok(Value::String(ch.to_string()))
}

/// Computes the Hamming distance between equal-length text.
fn hamming_distance(args: &[RValue]) -> Result<Value, DwError> {
    let left: Vec<char> = string(args, 0)?.chars().collect();
    let right: Vec<char> = string(args, 1)?.chars().collect();
    if left.len() != right.len() {
        return Ok(Value::Null);
    }
    let distance = left
        .iter()
        .zip(right.iter())
        .filter(|(a, b)| a != b)
        .count();
    Ok(Value::Number(Number::from_usize(distance)))
}

/// Tests every character against a predicate.
fn character_class<F>(args: &[RValue], predicate: F) -> Result<Value, DwError>
where
    F: Fn(char) -> bool,
{
    let text = string(args, 0)?;
    Ok(Value::Bool(!text.is_empty() && text.chars().all(predicate)))
}

/// Pads text on the left or on the right.
fn pad(args: &[RValue], left: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let width = integer(args, 1)?.max(0) as usize;
    let pad = match optional_value(args, 2) {
        Some(Value::String(value)) => value.clone(),
        Some(_) => return Err(DwError::new("padding must be a string")),
        None => " ".to_string(),
    };
    let current = text.chars().count();
    if current >= width || pad.is_empty() {
        return Ok(Value::String(text));
    }
    let needed = width - current;
    let mut fill = String::new();
    let pad_chars: Vec<char> = pad.chars().collect();
    for index in 0..needed {
        fill.push(pad_chars[index % pad_chars.len()]);
    }
    if left {
        Ok(Value::String(format!("{fill}{text}")))
    } else {
        Ok(Value::String(format!("{text}{fill}")))
    }
}

/// Computes the Levenshtein edit distance.
fn levenshtein(args: &[RValue]) -> Result<Value, DwError> {
    let left: Vec<char> = string(args, 0)?.chars().collect();
    let right: Vec<char> = string(args, 1)?.chars().collect();
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current = vec![0usize; right.len() + 1];
    for (i, a) in left.iter().enumerate() {
        current[0] = i + 1;
        for (j, b) in right.iter().enumerate() {
            let cost = if a == b { 0 } else { 1 };
            current[j + 1] = (previous[j + 1] + 1)
                .min(current[j] + 1)
                .min(previous[j] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    Ok(Value::Number(Number::from_usize(previous[right.len()])))
}

/// Splits text on CRLF, CR, or LF.
fn split_lines(text: &str) -> Vec<Value> {
    if text.is_empty() {
        return Vec::new();
    }
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines: Vec<&str> = normalized.split('\n').collect();
    if normalized.ends_with('\n') {
        lines.pop();
    }
    lines
        .into_iter()
        .map(|line| Value::String(line.to_string()))
        .collect()
}

/// Maps a function over the characters of text.
fn map_string(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let function = function(args, 1)?;
    let mut out = String::new();
    for (index, c) in text.chars().enumerate() {
        let result = call_fn(
            function,
            vec![
                Value::String(c.to_string()),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        out.push_str(&result.key_string());
    }
    Ok(Value::String(out))
}

/// Formats a number as an ordinal string.
fn ordinalize(args: &[RValue]) -> Result<Value, DwError> {
    let number = number(args, 0)?;
    let text = number.to_string();
    let integer_part = text.split('.').next().unwrap_or("0");
    let negative = integer_part.starts_with('-');
    let digits = integer_part.trim_start_matches('-');
    let digits = if digits.is_empty() { "0" } else { digits };
    let suffix = if negative {
        "th"
    } else {
        let last_two = if digits.len() >= 2 {
            digits[digits.len() - 2..].parse::<u32>().unwrap_or(0)
        } else {
            digits.parse::<u32>().unwrap_or(0)
        };
        let last = last_two % 10;
        if (11..=13).contains(&last_two) {
            "th"
        } else {
            match last {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        }
    };
    Ok(Value::String(format!("{integer_part}{suffix}")))
}

/// Converts an English singular word to plural form.
fn pluralize(text: &str) -> String {
    let lower = text.to_lowercase();
    let irregular = match lower.as_str() {
        "person" => Some("people"),
        "man" => Some("men"),
        "woman" => Some("women"),
        "child" => Some("children"),
        "goose" => Some("geese"),
        "foot" => Some("feet"),
        "tooth" => Some("teeth"),
        "mouse" => Some("mice"),
        "sheep" => Some("sheep"),
        "fish" => Some("fish"),
        "deer" => Some("deer"),
        "ox" => Some("oxen"),
        "knife" => Some("knives"),
        "wife" => Some("wives"),
        "life" => Some("lives"),
        "leaf" => Some("leaves"),
        "wolf" => Some("wolves"),
        "half" => Some("halves"),
        "self" => Some("selves"),
        "shelf" => Some("shelves"),
        "loaf" => Some("loaves"),
        "thief" => Some("thieves"),
        "calf" => Some("calves"),
        "elf" => Some("elves"),
        _ => None,
    };
    if let Some(word) = irregular {
        return word.to_string();
    }
    let unchanged = [
        "sheep",
        "fish",
        "deer",
        "series",
        "species",
        "news",
        "information",
    ];
    if unchanged.contains(&lower.as_str()) {
        return text.to_string();
    }
    if lower.ends_with('s')
        || lower.ends_with('x')
        || lower.ends_with('z')
        || lower.ends_with("ch")
        || lower.ends_with("sh")
    {
        return format!("{text}es");
    }
    if lower.ends_with('y') && lower.len() > 1 {
        let prev = lower.chars().nth(lower.len() - 2).unwrap_or('a');
        if !matches!(prev, 'a' | 'e' | 'i' | 'o' | 'u') {
            return format!("{}ies", &text[..text.len() - 1]);
        }
    }
    if lower.ends_with("fe") {
        return format!("{}ves", &text[..text.len() - 2]);
    }
    if lower.ends_with('f') {
        return format!("{}ves", &text[..text.len() - 1]);
    }
    format!("{text}s")
}

/// Converts an English plural word to singular form.
fn singularize(text: &str) -> String {
    let lower = text.to_lowercase();
    let irregular = match lower.as_str() {
        "people" => Some("person"),
        "men" => Some("man"),
        "women" => Some("woman"),
        "children" => Some("child"),
        "geese" => Some("goose"),
        "feet" => Some("foot"),
        "teeth" => Some("tooth"),
        "mice" => Some("mouse"),
        "sheep" => Some("sheep"),
        "fish" => Some("fish"),
        "deer" => Some("deer"),
        "oxen" => Some("ox"),
        "knives" => Some("knife"),
        "wives" => Some("wife"),
        "lives" => Some("life"),
        "leaves" => Some("leaf"),
        "wolves" => Some("wolf"),
        "halves" => Some("half"),
        "selves" => Some("self"),
        "shelves" => Some("shelf"),
        "loaves" => Some("loaf"),
        "thieves" => Some("thief"),
        "calves" => Some("calf"),
        "elves" => Some("elf"),
        _ => None,
    };
    if let Some(word) = irregular {
        return word.to_string();
    }
    if lower.ends_with("ies") && lower.len() > 3 {
        return format!("{}y", &text[..text.len() - 3]);
    }
    if lower.ends_with("ves") && lower.len() > 3 {
        return format!("{}f", &text[..text.len() - 3]);
    }
    if lower.ends_with("ches")
        || lower.ends_with("shes")
        || lower.ends_with("xes")
        || lower.ends_with("zes")
        || lower.ends_with("ses")
    {
        return text[..text.len() - 2].to_string();
    }
    if lower.ends_with('s') && !lower.ends_with("ss") && lower.len() > 1 {
        return text[..text.len() - 1].to_string();
    }
    text.to_string()
}

/// Repeats text a number of times.
fn repeat(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let count = integer(args, 1)?.max(0) as usize;
    Ok(Value::String(text.repeat(count)))
}

/// Returns a substring by character positions.
fn substring(args: &[RValue]) -> Result<Value, DwError> {
    let text: Vec<char> = string(args, 0)?.chars().collect();
    let start = integer(args, 1)?.max(0) as usize;
    let end = optional_value(args, 2)
        .map(|value| match value {
            Value::Number(number) => number.to_i64().unwrap_or(0),
            _ => 0,
        })
        .map(|value| value.max(0) as usize)
        .unwrap_or(text.len());
    if start >= text.len() || start >= end {
        return Ok(Value::String(String::new()));
    }
    let end = end.min(text.len());
    Ok(Value::String(text[start..end].iter().collect()))
}

/// Returns text after a separator, optionally the last match.
fn substring_after(args: &[RValue], last: bool, before: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let separator = string(args, 1)?;
    let found = if last {
        text.rfind(&separator)
    } else {
        text.find(&separator)
    };
    match found {
        Some(index) => {
            let result = if before {
                &text[..index]
            } else {
                &text[index + separator.len()..]
            };
            Ok(Value::String(result.to_string()))
        }
        None => {
            if before {
                Ok(Value::String(text))
            } else {
                Ok(Value::String(String::new()))
            }
        }
    }
}

/// Splits text into runs selected by a predicate.
fn substring_by(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::new();
    let mut current = String::new();
    for (index, c) in text.chars().enumerate() {
        let is_separator = call_fn(
            function,
            vec![
                Value::String(c.to_string()),
                Value::Number(Number::from_usize(index)),
            ],
        )?
        .truthy();
        if is_separator {
            if !current.is_empty() {
                out.push(Value::String(std::mem::take(&mut current)));
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        out.push(Value::String(current));
    }
    Ok(Value::Array(out))
}

/// Splits text into fixed-size chunks.
fn substring_every(args: &[RValue]) -> Result<Value, DwError> {
    let text: Vec<char> = string(args, 0)?.chars().collect();
    let size = integer(args, 1)?.max(1) as usize;
    let mut out = Vec::new();
    let mut index = 0;
    while index < text.len() {
        let end = (index + size).min(text.len());
        out.push(Value::String(text[index..end].iter().collect()));
        index = end;
    }
    Ok(Value::Array(out))
}

/// Removes matching surrounding text.
fn unwrap(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let wrapper = string(args, 1)?;
    if !wrapper.is_empty() && text.starts_with(&wrapper) && text.ends_with(&wrapper) {
        let start = wrapper.len();
        let end = text.len() - wrapper.len();
        if start <= end {
            return Ok(Value::String(text[start..end].to_string()));
        }
    }
    Ok(Value::String(text))
}

/// Truncates text to a maximum size.
fn with_max_size(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let size = integer(args, 1)?.max(0) as usize;
    Ok(Value::String(text.chars().take(size).collect()))
}

/// Wraps text only when the wrapper is absent.
fn wrap_if_missing(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let wrapper = string(args, 1)?;
    let mut out = String::new();
    if !text.starts_with(&wrapper) {
        out.push_str(&wrapper);
    }
    out.push_str(&text);
    if !text.ends_with(&wrapper) {
        out.push_str(&wrapper);
    }
    Ok(Value::String(out))
}

/// Wraps text with the given wrapper.
fn wrap_with(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let wrapper = string(args, 1)?;
    Ok(Value::String(format!("{wrapper}{text}{wrapper}")))
}

/// Tests whether every object entry matches a predicate.
fn every_entry(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let function = function(args, 1)?;
    for (index, (key, item)) in entries.into_iter().enumerate() {
        let result = call_fn(
            function,
            vec![
                item,
                Value::String(key),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if !result.truthy() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

/// Tests whether any object entry matches a predicate.
fn some_entry(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let function = function(args, 1)?;
    for (index, (key, item)) in entries.into_iter().enumerate() {
        let result = call_fn(
            function,
            vec![
                item,
                Value::String(key),
                Value::Number(Number::from_usize(index)),
            ],
        )?;
        if result.truthy() {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

/// Formats a number in a radix, or returns null.
fn radix_string(args: &[RValue], default_radix: u32, explicit: bool) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let number = number(args, 0)?;
    if !number.is_integer() {
        return Err(DwError::new("radix conversion requires an integer"));
    }
    let radix = if explicit {
        integer(args, 1)? as u32
    } else {
        default_radix
    };
    if !(2..=36).contains(&radix) {
        return Err(DwError::new("radix must be between 2 and 36"));
    }
    let text = number.to_string();
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.as_str()),
    };
    let value = BigUint::parse_bytes(digits.as_bytes(), 10)
        .ok_or_else(|| DwError::new("invalid integer"))?;
    let mut out = value.to_str_radix(radix);
    if negative {
        out.insert(0, '-');
    }
    Ok(Value::String(out))
}

/// Parses a number in a radix, or returns null.
fn radix_number(args: &[RValue], default_radix: u32, explicit: bool) -> Result<Value, DwError> {
    if matches!(value(args, 0)?, Value::Null) {
        return Ok(Value::Null);
    }
    let text = string(args, 0)?;
    let radix = if explicit {
        integer(args, 1)? as u32
    } else {
        default_radix
    };
    if !(2..=36).contains(&radix) {
        return Err(DwError::new("radix must be between 2 and 36"));
    }
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.as_str()),
    };
    let value = BigUint::parse_bytes(digits.as_bytes(), radix)
        .ok_or_else(|| DwError::new("invalid number for radix"))?;
    let decimal = value.to_str_radix(10);
    let number = Number::parse(&decimal).ok_or_else(|| DwError::new("invalid number"))?;
    Ok(Value::Number(if negative { number.neg() } else { number }))
}

fn object_part(args: &[RValue], key: &str) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    entries
        .into_iter()
        .rev()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
        .ok_or_else(|| DwError::new(format!("missing '{key}' in date factory")))
}

fn part_number(args: &[RValue], key: &str) -> Result<Number, DwError> {
    match object_part(args, key)? {
        Value::Number(number) => Ok(number),
        other => Err(DwError::new(format!(
            "expected Number for '{key}', found {}",
            other.type_name()
        ))),
    }
}

fn part_i64(args: &[RValue], key: &str) -> Result<i64, DwError> {
    part_number(args, key)?
        .to_i64()
        .ok_or_else(|| DwError::new(format!("expected a whole number for '{key}'")))
}

fn part_i32(args: &[RValue], key: &str) -> Result<i32, DwError> {
    let value = part_i64(args, key)?;
    i32::try_from(value).map_err(|_| DwError::new(format!("'{key}' is out of range")))
}

fn part_u32(args: &[RValue], key: &str) -> Result<u32, DwError> {
    let value = part_i64(args, key)?;
    u32::try_from(value).map_err(|_| DwError::new(format!("'{key}' is out of range")))
}

fn optional_number(args: &[RValue], key: &str) -> Result<Option<Number>, DwError> {
    let entries = object(args, 0)?;
    match entries.into_iter().rev().find(|(name, _)| name == key) {
        Some((_, Value::Number(number))) => Ok(Some(number)),
        Some((_, other)) => Err(DwError::new(format!(
            "expected Number for '{key}', found {}",
            other.type_name()
        ))),
        None => Ok(None),
    }
}

fn validate_date(date: &crate::temporal::Date) -> Result<(), DwError> {
    date.add_days(0).map(|_| ())
}

fn make_time(
    hour: u32,
    minute: u32,
    second: u32,
    nano: u32,
) -> Result<crate::temporal::LocalTime, DwError> {
    if hour > 23 || minute > 59 || second > 59 || nano > 999_999_999 {
        return Err(DwError::new("time field is out of range"));
    }
    Ok(crate::temporal::LocalTime {
        hour,
        minute,
        second,
        nano,
    })
}

fn date(args: &[RValue]) -> Result<Value, DwError> {
    let date = crate::temporal::Date {
        year: part_i32(args, "year")?,
        month: part_u32(args, "month")?,
        day: part_u32(args, "day")?,
    };
    validate_date(&date)?;
    Ok(Value::Date(date))
}

fn zone_part(args: &[RValue]) -> Result<crate::temporal::Zone, DwError> {
    let entries = object(args, 0)?;
    for key in ["timezone", "timeZone"] {
        if let Some((_, value)) = entries.iter().rev().find(|(name, _)| name == key) {
            return match value {
                Value::TimeZone(zone) => Ok(zone.clone()),
                Value::String(text) => crate::temporal::Zone::parse(text)
                    .ok_or_else(|| DwError::new("invalid time zone")),
                other => Err(DwError::new(format!(
                    "expected TimeZone, found {}",
                    other.type_name()
                ))),
            };
        }
    }
    Ok(crate::temporal::Zone::utc())
}

fn seconds_part(args: &[RValue], key: &str) -> Result<(u32, u32), DwError> {
    let number = optional_number(args, key)?.unwrap_or_else(Number::zero);
    let text = number.to_string();
    let (whole, fraction) = match text.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (text.as_str(), None),
    };
    let whole: u32 = whole
        .parse()
        .map_err(|_| DwError::new(format!("'{key}' is out of range")))?;
    let nano = match fraction {
        Some(fraction) => crate::temporal::parse_fraction(fraction)
            .ok_or_else(|| DwError::new("invalid fraction"))?,
        None => 0,
    };
    Ok((whole, nano))
}

fn date_time(args: &[RValue]) -> Result<Value, DwError> {
    let date = crate::temporal::Date {
        year: part_i32(args, "year")?,
        month: part_u32(args, "month")?,
        day: part_u32(args, "day")?,
    };
    validate_date(&date)?;
    let (second, nano) = seconds_part(args, "seconds")?;
    let time = make_time(
        part_u32(args, "hour")?,
        part_u32(args, "minutes")?,
        second,
        nano,
    )?;
    Ok(Value::DateTime(crate::temporal::DateTime {
        date,
        time,
        zone: zone_part(args)?,
    }))
}

fn local_date_time(args: &[RValue]) -> Result<Value, DwError> {
    let date = crate::temporal::Date {
        year: part_i32(args, "year")?,
        month: part_u32(args, "month")?,
        day: part_u32(args, "day")?,
    };
    validate_date(&date)?;
    let (second, nano) = seconds_part(args, "seconds")?;
    let time = make_time(
        part_u32(args, "hour")?,
        part_u32(args, "minutes")?,
        second,
        nano,
    )?;
    Ok(Value::LocalDateTime(crate::temporal::LocalDateTime {
        date,
        time,
    }))
}

fn local_time(args: &[RValue]) -> Result<Value, DwError> {
    let (second, nano) = seconds_part(args, "seconds")?;
    Ok(Value::LocalTime(make_time(
        part_u32(args, "hour")?,
        part_u32(args, "minutes")?,
        second,
        nano,
    )?))
}

fn time(args: &[RValue]) -> Result<Value, DwError> {
    let (second, nano) = seconds_part(args, "seconds")?;
    Ok(Value::Time(crate::temporal::Time {
        time: make_time(
            part_u32(args, "hour")?,
            part_u32(args, "minutes")?,
            second,
            nano,
        )?,
        zone: zone_part(args)?,
    }))
}

fn at_beginning(args: &[RValue], part: &str) -> Result<Value, DwError> {
    let value = value(args, 0)?.clone();
    let zero_time = || crate::temporal::LocalTime {
        hour: 0,
        minute: 0,
        second: 0,
        nano: 0,
    };
    match value {
        Value::Date(date) => {
            let date = match part {
                "month" => crate::temporal::Date {
                    year: date.year,
                    month: date.month,
                    day: 1,
                },
                "week" => date.beginning_of_week()?,
                "year" => crate::temporal::Date {
                    year: date.year,
                    month: 1,
                    day: 1,
                },
                _ => date,
            };
            Ok(Value::Date(date))
        }
        Value::LocalTime(mut time) => {
            if part == "hour" {
                time.minute = 0;
                time.second = 0;
                time.nano = 0;
            }
            Ok(Value::LocalTime(time))
        }
        Value::Time(mut time) => {
            if part == "hour" {
                time.time.minute = 0;
                time.time.second = 0;
                time.time.nano = 0;
            }
            Ok(Value::Time(time))
        }
        Value::LocalDateTime(mut value) => {
            if part == "month" {
                value.date.month = 1;
                value.date.day = 1;
                value.time = zero_time();
            } else if part == "week" {
                value.date = value.date.beginning_of_week()?;
                value.time = zero_time();
            } else if part == "year" {
                value.date.month = 1;
                value.date.day = 1;
                value.time = zero_time();
            } else if part == "day" {
                value.time = zero_time();
            } else if part == "hour" {
                value.time.minute = 0;
                value.time.second = 0;
                value.time.nano = 0;
            }
            Ok(Value::LocalDateTime(value))
        }
        Value::DateTime(mut value) => {
            if part == "month" {
                value.date.month = 1;
                value.date.day = 1;
                value.time = zero_time();
            } else if part == "week" {
                value.date = value.date.beginning_of_week()?;
                value.time = zero_time();
            } else if part == "year" {
                value.date.month = 1;
                value.date.day = 1;
                value.time = zero_time();
            } else if part == "day" {
                value.time = zero_time();
            } else if part == "hour" {
                value.time.minute = 0;
                value.time.second = 0;
                value.time.nano = 0;
            }
            Ok(Value::DateTime(value))
        }
        other => Err(DwError::new(format!(
            "cannot find the beginning of a {}",
            other.type_name()
        ))),
    }
}

fn value_as_date(value: &Value) -> Result<crate::temporal::Date, DwError> {
    match value {
        Value::Date(date) => Ok(*date),
        Value::DateTime(date) => Ok(date.date),
        Value::LocalDateTime(date) => Ok(date.date),
        Value::String(text) => crate::temporal::parse_date(text)
            .or_else(|| match crate::temporal::parse_literal(text) {
                Some(Value::Date(date)) => Some(date),
                Some(Value::DateTime(date)) => Some(date.date),
                Some(Value::LocalDateTime(date)) => Some(date.date),
                _ => None,
            })
            .ok_or_else(|| DwError::new(format!("cannot read '{text}' as a Date"))),
        other => Err(DwError::new(format!(
            "expected a Date, found {}",
            other.type_name()
        ))),
    }
}

fn between(args: &[RValue]) -> Result<Value, DwError> {
    let end = value_as_date(value(args, 0)?)?;
    let start = value_as_date(value(args, 1)?)?;
    Ok(Value::Period(crate::temporal::period_between(
        &start, &end,
    )?))
}

fn make_days(args: &[RValue]) -> Result<Value, DwError> {
    let number = number(args, 0)?;
    if number.is_integer() {
        let days = number
            .to_i64()
            .ok_or_else(|| DwError::new("days is out of range"))?;
        Ok(Value::Period(crate::temporal::date_period(0, 0, days)))
    } else {
        Ok(Value::Period(crate::temporal::duration_period(
            number.mul(&Number::from_i64(86_400)),
        )))
    }
}

fn make_duration(args: &[RValue], multiplier: i64) -> Result<Value, DwError> {
    let number = number(args, 0)?;
    Ok(Value::Period(crate::temporal::duration_period(
        number.mul(&Number::from_i64(multiplier)),
    )))
}

fn make_date_period(args: &[RValue], unit: &str) -> Result<Value, DwError> {
    let number = number(args, 0)?;
    let value = number
        .to_i64()
        .ok_or_else(|| DwError::new(format!("{unit} must be a whole number")))?;
    Ok(Value::Period(match unit {
        "months" => crate::temporal::date_period(0, value, 0),
        "years" => crate::temporal::date_period(value, 0, 0),
        _ => crate::temporal::date_period(0, 0, value),
    }))
}

fn period(args: &[RValue]) -> Result<Value, DwError> {
    let years = optional_number(args, "years")?
        .map(|number| {
            number
                .to_i64()
                .ok_or_else(|| DwError::new("years must be a whole number"))
        })
        .transpose()?
        .unwrap_or(0);
    let months = optional_number(args, "months")?
        .map(|number| {
            number
                .to_i64()
                .ok_or_else(|| DwError::new("months must be a whole number"))
        })
        .transpose()?
        .unwrap_or(0);
    let days = optional_number(args, "days")?
        .map(|number| {
            number
                .to_i64()
                .ok_or_else(|| DwError::new("days must be a whole number"))
        })
        .transpose()?
        .unwrap_or(0);
    Ok(Value::Period(crate::temporal::date_period(
        years, months, days,
    )))
}

fn duration(args: &[RValue]) -> Result<Value, DwError> {
    let mut seconds = Number::zero();
    for (key, multiplier) in [
        ("days", 86_400i64),
        ("hours", 3_600),
        ("minutes", 60),
        ("seconds", 1),
    ] {
        if let Some(number) = optional_number(args, key)? {
            seconds = seconds.add(&number.mul(&Number::from_i64(multiplier)));
        }
    }
    Ok(Value::Period(crate::temporal::duration_period(seconds)))
}

fn days_between(args: &[RValue]) -> Result<Value, DwError> {
    let from = value_as_date(value(args, 0)?)?;
    let to = value_as_date(value(args, 1)?)?;
    Ok(Value::Number(Number::from_i64(
        crate::temporal::date_difference(&from, &to)?,
    )))
}

fn year_of(value: &Value) -> Result<i32, DwError> {
    Ok(value_as_date(value)?.year)
}

fn is_leap_year(args: &[RValue]) -> Result<Value, DwError> {
    let year = year_of(value(args, 0)?)?;
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    Ok(Value::Bool(leap))
}

fn path_element(kind: &str, selector: Value) -> Value {
    Value::Object(vec![
        ("kind".to_string(), Value::String(kind.to_string())),
        ("selector".to_string(), selector),
        ("namespace".to_string(), Value::Null),
    ])
}

fn path_append(path: &[Value], kind: &str, selector: Value) -> Vec<Value> {
    let mut next = path.to_vec();
    next.push(path_element(kind, selector));
    next
}

fn path_kind(args: &[RValue], kind: &str) -> Result<Value, DwError> {
    let path = array(args, 0)?;
    let matches = path
        .last()
        .and_then(Value::as_object)
        .and_then(|entries| {
            entries
                .iter()
                .rev()
                .find(|(name, _)| name == "kind")
                .map(|(_, value)| value)
        })
        .map(|value| value.key_string() == kind)
        .unwrap_or(false);
    Ok(Value::Bool(matches))
}

fn as_expression_string(args: &[RValue]) -> Result<Value, DwError> {
    let path = array(args, 0)?;
    let mut out = String::new();
    for element in path {
        let entries = element
            .as_object()
            .ok_or_else(|| DwError::new("expected a Path element"))?;
        let part = |name: &str| {
            entries
                .iter()
                .rev()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value)
        };
        let kind = part("kind").map(Value::key_string).unwrap_or_default();
        let selector = part("selector").map(Value::key_string).unwrap_or_default();
        match kind.as_str() {
            "Object" => {
                out.push('.');
                out.push_str(&selector);
            }
            "Attribute" => {
                out.push_str(".@");
                out.push_str(&selector);
            }
            "Array" => {
                out.push('[');
                out.push_str(&selector);
                out.push(']');
            }
            _ => {}
        }
    }
    Ok(Value::String(out))
}

fn tree_callback(function: &RValue, value: &Value, path: &[Value]) -> Result<Value, DwError> {
    call_fn(function, vec![value.clone(), Value::Array(path.to_vec())])
}

fn map_leaf_value(value: &Value, path: &[Value], function: &RValue) -> Result<Value, DwError> {
    match value {
        Value::Object(entries) => {
            let mut out = Vec::with_capacity(entries.len());
            for (key, child) in entries {
                let next = path_append(path, "Object", Value::String(key.clone()));
                let mapped = match child {
                    Value::Object(_) | Value::Array(_) => map_leaf_value(child, &next, function)?,
                    _ => tree_callback(function, child, &next)?,
                };
                out.push((key.clone(), mapped));
            }
            Ok(Value::Object(out))
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (index, child) in items.iter().enumerate() {
                let next = path_append(path, "Array", Value::Number(Number::from_usize(index)));
                let mapped = match child {
                    Value::Object(_) | Value::Array(_) => map_leaf_value(child, &next, function)?,
                    _ => tree_callback(function, child, &next)?,
                };
                out.push(mapped);
            }
            Ok(Value::Array(out))
        }
        _ => tree_callback(function, value, path),
    }
}

fn map_leaf_values(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let function = function(args, 1)?;
    map_leaf_value(value, &[], function)
}

fn filter_array_leaf(value: &Value, path: &[Value], function: &RValue) -> Result<Value, DwError> {
    match value {
        Value::Object(entries) => {
            let mut out = Vec::with_capacity(entries.len());
            for (key, child) in entries {
                let next = path_append(path, "Object", Value::String(key.clone()));
                out.push((key.clone(), filter_array_leaf(child, &next, function)?));
            }
            Ok(Value::Object(out))
        }
        Value::Array(items) => {
            let mut out = Vec::new();
            for (index, child) in items.iter().enumerate() {
                let next = path_append(path, "Array", Value::Number(Number::from_usize(index)));
                let keep = match child {
                    Value::Object(_) | Value::Array(_) => true,
                    _ => tree_callback(function, child, &next)?.truthy(),
                };
                if keep {
                    out.push(filter_array_leaf(child, &next, function)?);
                }
            }
            Ok(Value::Array(out))
        }
        _ => Ok(value.clone()),
    }
}

fn filter_array_leafs(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let function = function(args, 1)?;
    filter_array_leaf(value, &[], function)
}

fn filter_object_leaf(value: &Value, path: &[Value], function: &RValue) -> Result<Value, DwError> {
    match value {
        Value::Object(entries) => {
            let mut out = Vec::new();
            for (key, child) in entries {
                let next = path_append(path, "Object", Value::String(key.clone()));
                let keep = match child {
                    Value::Object(_) | Value::Array(_) => true,
                    _ => tree_callback(function, child, &next)?.truthy(),
                };
                if keep {
                    out.push((key.clone(), filter_object_leaf(child, &next, function)?));
                }
            }
            Ok(Value::Object(out))
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (index, child) in items.iter().enumerate() {
                let next = path_append(path, "Array", Value::Number(Number::from_usize(index)));
                out.push(filter_object_leaf(child, &next, function)?);
            }
            Ok(Value::Array(out))
        }
        _ => Ok(value.clone()),
    }
}

fn filter_object_leafs(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let function = function(args, 1)?;
    filter_object_leaf(value, &[], function)
}

fn filter_tree_node(
    value: &Value,
    path: &[Value],
    function: &RValue,
) -> Result<Option<Value>, DwError> {
    if !tree_callback(function, value, path)?.truthy() {
        return Ok(None);
    }
    match value {
        Value::Object(entries) => {
            let mut out = Vec::new();
            for (key, child) in entries {
                let next = path_append(path, "Object", Value::String(key.clone()));
                if let Some(value) = filter_tree_node(child, &next, function)? {
                    out.push((key.clone(), value));
                }
            }
            Ok(Some(Value::Object(out)))
        }
        Value::Array(items) => {
            let mut out = Vec::new();
            for (index, child) in items.iter().enumerate() {
                let next = path_append(path, "Array", Value::Number(Number::from_usize(index)));
                if let Some(value) = filter_tree_node(child, &next, function)? {
                    out.push(value);
                }
            }
            Ok(Some(Value::Array(out)))
        }
        _ => Ok(Some(value.clone())),
    }
}

fn filter_tree(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let function = function(args, 1)?;
    if !matches!(value, Value::Object(_) | Value::Array(_)) {
        return Ok(value.clone());
    }
    Ok(filter_tree_node(value, &[], function)?.unwrap_or(Value::Null))
}

fn node_exists_inner(value: &Value, path: &[Value], function: &RValue) -> Result<bool, DwError> {
    if tree_callback(function, value, path)?.truthy() {
        return Ok(true);
    }
    match value {
        Value::Object(entries) => {
            for (key, child) in entries {
                let next = path_append(path, "Object", Value::String(key.clone()));
                if node_exists_inner(child, &next, function)? {
                    return Ok(true);
                }
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                let next = path_append(path, "Array", Value::Number(Number::from_usize(index)));
                if node_exists_inner(child, &next, function)? {
                    return Ok(true);
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn node_exists(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let function = function(args, 1)?;
    Ok(Value::Bool(node_exists_inner(value, &[], function)?))
}

fn optional_object(args: &[RValue], index: usize) -> Option<Value> {
    match args.get(index) {
        Some(RValue::Val(value @ Value::Object(_))) => Some(value.clone()),
        _ => None,
    }
}

fn object_property(object: Option<&Value>, key: &str) -> Option<Value> {
    object.and_then(|value| value.get(key)).cloned()
}

fn property_string(object: Option<&Value>, key: &str) -> Option<String> {
    match object_property(object, key) {
        Some(Value::String(text)) => Some(text),
        Some(other) => Some(other.key_string()),
        None => None,
    }
}

fn property_bool(object: Option<&Value>, key: &str, default: bool) -> bool {
    match object_property(object, key) {
        Some(Value::Bool(value)) => value,
        Some(Value::String(text)) => text.eq_ignore_ascii_case("true"),
        _ => default,
    }
}

fn content_type(args: &[RValue], index: usize, default: &str) -> String {
    match args.get(index) {
        Some(RValue::Val(Value::String(text))) if !text.is_empty() => {
            text.split(';').next().unwrap_or(text).trim().to_string()
        }
        _ => default.to_string(),
    }
}

fn text_or_binary(value: &Value) -> Result<Vec<u8>, DwError> {
    match value {
        Value::String(text) => Ok(text.as_bytes().to_vec()),
        Value::Binary(bytes) => Ok(bytes.clone()),
        other => Err(DwError::new(format!(
            "expected String or Binary, found {}",
            other.type_name()
        ))),
    }
}

fn read_value(args: &[RValue]) -> Result<Value, DwError> {
    let input = value(args, 0)?;
    let content_type = content_type(args, 1, "application/dw");
    let properties = optional_object(args, 2);
    read_content(input, &content_type, properties.as_ref())
}

fn read_content(
    input: &Value,
    content_type: &str,
    properties: Option<&Value>,
) -> Result<Value, DwError> {
    let text = || -> Result<String, DwError> {
        match input {
            Value::String(text) => Ok(text.clone()),
            Value::Binary(bytes) => String::from_utf8(bytes.clone())
                .map_err(|_| DwError::new("binary content is not valid UTF-8")),
            other => Err(DwError::new(format!(
                "expected String or Binary, found {}",
                other.type_name()
            ))),
        }
    };
    match content_type {
        "" | "application/json" | "text/json" => {
            let text = text()?;
            crate::json::parse(&text)
        }
        "application/dw" | "application/x-dw" | "text/x-dw" => {
            let text = text()?;
            let expression = crate::parser::parse_expression(&text)?;
            let value = crate::eval::eval(&expression, &crate::runtime::Env::default())?;
            crate::eval::as_value(value)
        }
        "text/plain" => Ok(Value::String(text()?)),
        "application/octet-stream" | "application/x-binary" | "binary" => {
            Ok(Value::Binary(text_or_binary(input)?))
        }
        "application/csv" | "text/csv" => {
            let text = text()?;
            Ok(read_csv(&text, properties))
        }
        other => Err(DwError::new(format!("unsupported content type '{other}'"))),
    }
}

fn read_csv(text: &str, properties: Option<&Value>) -> Value {
    let separator = property_string(properties, "separator")
        .and_then(|text| text.chars().next())
        .unwrap_or(',');
    let has_header = property_bool(properties, "header", true);
    let mut rows = text.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split(separator)
            .map(str::to_string)
            .collect::<Vec<_>>()
    });
    let headers = if has_header {
        rows.next().unwrap_or_default()
    } else {
        Vec::new()
    };
    let mut out = Vec::new();
    for row in rows {
        let mut entries = Vec::new();
        for (index, cell) in row.into_iter().enumerate() {
            let key = headers
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("column_{index}"));
            entries.push((key, Value::String(cell)));
        }
        out.push(Value::Object(entries));
    }
    Value::Array(out)
}

fn write_value(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let content_type = content_type(args, 1, "application/dw");
    let properties = optional_object(args, 2);
    write_content(value, &content_type, properties.as_ref())
}

fn write_content(
    value: &Value,
    content_type: &str,
    properties: Option<&Value>,
) -> Result<Value, DwError> {
    match content_type {
        "" | "application/json" | "text/json" => {
            let options = crate::json::WriteOptions {
                indent: property_bool(properties, "indent", true),
                skip_null_on: property_string(properties, "skipNullOn"),
                duplicate_key_as_array: property_bool(properties, "duplicateKeyAsArray", false),
            };
            Ok(Value::String(crate::json::write_with(value, &options)))
        }
        "application/dw" | "application/x-dw" | "text/x-dw" => {
            Ok(Value::String(dw_write(value, 0)))
        }
        "text/plain" => Ok(Value::String(value.key_string())),
        "application/octet-stream" | "application/x-binary" | "binary" => {
            Ok(Value::Binary(text_or_binary(value)?))
        }
        other => Err(DwError::new(format!("unsupported content type '{other}'"))),
    }
}

fn valid_dw_key(text: &str) -> bool {
    let mut chars = text.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

fn dw_write(value: &Value, level: usize) -> String {
    let mut out = String::new();
    let pad = "  ".repeat(level);
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
        Value::Number(number) => out.push_str(&number.to_string()),
        Value::String(_) => out.push_str(&crate::json::write(value)),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push('\n');
                out.push_str(&"  ".repeat(level + 1));
                out.push_str(&dw_write(item, level + 1));
            }
            if !items.is_empty() {
                out.push('\n');
                out.push_str(&pad);
            }
            out.push(']');
        }
        Value::Object(entries) => {
            out.push('{');
            for (index, (key, item)) in entries.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push('\n');
                out.push_str(&"  ".repeat(level + 1));
                if valid_dw_key(key) {
                    out.push_str(key);
                } else {
                    out.push_str(&crate::json::write(&Value::String(key.clone())));
                }
                out.push_str(": ");
                out.push_str(&dw_write(item, level + 1));
            }
            if !entries.is_empty() {
                out.push('\n');
                out.push_str(&pad);
            }
            out.push('}');
        }
        Value::Date(date) => out.push_str(&format!("|{}|", crate::temporal::format_date(date))),
        Value::DateTime(date) => {
            out.push_str(&format!("|{}|", crate::temporal::format_date_time(date)))
        }
        Value::LocalDateTime(date) => out.push_str(&format!(
            "|{}|",
            crate::temporal::format_local_date_time(date)
        )),
        Value::LocalTime(time) => {
            out.push_str(&format!("|{}|", crate::temporal::format_local_time(time)))
        }
        Value::Time(time) => out.push_str(&format!("|{}|", crate::temporal::format_time(time))),
        Value::TimeZone(zone) => out.push_str(&format!("|{}|", zone.label())),
        Value::Period(period) => {
            out.push_str(&format!("|{}|", crate::temporal::format_period(period)))
        }
        Value::Binary(bytes) => out.push_str(&format!(
            "\"{}\" as Binary {{base: \"64\"}}",
            crate::temporal::base64_encode(bytes)
        )),
        Value::Regex(pattern) => out.push_str(&format!("/{pattern}/")),
        Value::Uri(text) | Value::Key(text) => out.push_str(text),
    }
    out
}

fn read_url(args: &[RValue]) -> Result<Value, DwError> {
    let url = string(args, 0)?;
    let content_type = content_type(args, 1, "application/dw");
    let properties = optional_object(args, 2);
    let bytes = if let Some(path) = url.strip_prefix("file://") {
        std::fs::read(path)
    } else if let Some(path) = url.strip_prefix("classpath://") {
        std::fs::read(path)
    } else if url.starts_with("http://") {
        http_get(&url)
    } else {
        std::fs::read(&url)
    }
    .map_err(|error| DwError::new(format!("cannot read URL '{url}': {error}")))?;
    let text =
        String::from_utf8(bytes).map_err(|_| DwError::new("URL content is not valid UTF-8"))?;
    read_content(&Value::String(text), &content_type, properties.as_ref())
}

fn http_get(url: &str) -> std::io::Result<Vec<u8>> {
    use std::io::{Read, Write};
    let rest = url.strip_prefix("http://").unwrap_or(url);
    let (host, path) = match rest.split_once('/') {
        Some((host, path)) => (host, format!("/{path}")),
        None => (rest, "/".to_string()),
    };
    let mut stream = std::net::TcpStream::connect(host)?;
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let split = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
        .unwrap_or(0);
    Ok(response[split..].to_vec())
}

fn concat_with(args: &[RValue]) -> Result<Value, DwError> {
    match (value(args, 0)?, value(args, 1)?) {
        (Value::Binary(left), Value::Binary(right)) => {
            let mut out = left.clone();
            out.extend(right);
            Ok(Value::Binary(out))
        }
        (Value::Null, Value::Binary(right)) => Ok(Value::Binary(right.clone())),
        (Value::Binary(left), Value::Null) => Ok(Value::Binary(left.clone())),
        (Value::Null, Value::Null) => Ok(Value::Null),
        (left, right) => Err(DwError::new(format!(
            "cannot concatenate {} and {} as binary",
            left.type_name(),
            right.type_name()
        ))),
    }
}

fn decode_charset(bytes: &[u8], charset: &str) -> Result<String, DwError> {
    match charset.to_ascii_uppercase().replace('_', "-").as_str() {
        "UTF-8" | "UTF8" | "US-ASCII" | "ASCII" => String::from_utf8(bytes.to_vec())
            .map_err(|_| DwError::new("binary content is not valid UTF-8")),
        other => Err(DwError::new(format!("unsupported charset '{other}'"))),
    }
}

fn encode_charset(text: &str, charset: &str) -> Result<Vec<u8>, DwError> {
    match charset.to_ascii_uppercase().replace('_', "-").as_str() {
        "UTF-8" | "UTF8" | "US-ASCII" | "ASCII" => Ok(text.as_bytes().to_vec()),
        other => Err(DwError::new(format!("unsupported charset '{other}'"))),
    }
}

fn read_lines_with(args: &[RValue]) -> Result<Value, DwError> {
    let bytes = value(args, 0)?
        .as_binary()
        .ok_or_else(|| DwError::new("expected Binary"))?;
    let charset = string(args, 1)?;
    let text = decode_charset(bytes, &charset)?;
    let mut lines: Vec<String> = text
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
        .collect();
    if lines.last().map(|line| line.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    Ok(Value::Array(lines.into_iter().map(Value::String).collect()))
}

fn write_lines_with(args: &[RValue]) -> Result<Value, DwError> {
    let lines = array(args, 0)?;
    let charset = string(args, 1)?;
    let mut text = String::new();
    for line in lines {
        text.push_str(&line.key_string());
        text.push('\n');
    }
    Ok(Value::Binary(encode_charset(&text, &charset)?))
}
