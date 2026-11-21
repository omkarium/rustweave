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
    "fail",
    "failIf",
    "try",
    "orElseTry",
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
    "toArray",
    "toBinary",
    "toBoolean",
    "toDate",
    "toDateOrNull",
    "toDateTime",
    "toDateTimeOrNull",
    "toLocalDateTime",
    "toLocalDateTimeOrNull",
    "toLocalTime",
    "toLocalTimeOrNull",
    "toNumber",
    "toNumberOrNull",
    "toPeriod",
    "toRegex",
    "toString",
    "toTime",
    "toTimeOrNull",
    "toTimeZone",
    "toURI",
    "toUri",
    "attr",
    "field",
    "index",
    "mask",
    "update",
    "acos",
    "asin",
    "atan",
    "cos",
    "decimalAdd",
    "decimalDivide",
    "decimalMultiply",
    "decimalPow",
    "decimalRound",
    "decimalSqrt",
    "decimalSubtract",
    "log10",
    "logn",
    "sin",
    "tan",
    "toDegrees",
    "toRadians",
    "compose",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "parseURI",
    "diff",
    "logDebug",
    "logInfo",
    "logWarn",
    "logError",
    "logWith",
    "plusPlus",
    "minusMinus",
    "arrayItem",
    "baseTypeOf",
    "functionParamTypes",
    "functionReturnType",
    "intersectionItems",
    "isAnyType",
    "isBinaryType",
    "isBooleanType",
    "isDateTimeType",
    "isDateType",
    "isFunctionType",
    "isIntersectionType",
    "isKeyType",
    "isLiteralType",
    "isLocalDateTimeType",
    "isLocalTimeType",
    "isNamespaceType",
    "isNothingType",
    "isNullType",
    "isNumberType",
    "isPeriodType",
    "isRangeType",
    "isReferenceType",
    "isRegexType",
    "isStringType",
    "isTimeType",
    "isTimeZoneType",
    "isTypeType",
    "isUnionType",
    "isUriType",
    "literalValueOf",
    "metadataOf",
    "nameOf",
    "objectFields",
    "unionItems",
    "enumerate",
    "range",
    "sorted",
    "reversed",
    "chunk",
    "compact",
    "unique",
    "fromEntries",
    "assign",
    "pick",
    "omit",
    "hasOwn",
    "parseInt",
    "parseFloat",
    "toFixed",
    "clamp",
    "trunc",
    "sign",
    "title",
    "lstrip",
    "rstrip",
    "zfill",
    "rpartition",
    "identity",
    "pipe",
    "fold",
    "filterMap",
    "stepBy",
    "position",
    "rposition",
    "dedup",
    "windows",
    "chunksExact",
    "binarySearch",
    "rotateLeft",
    "rotateRight",
    "swap",
    "fill",
    "truncate",
    "splitOnce",
    "rsplitOnce",
    "stripPrefix",
    "stripSuffix",
    "unwrapOr",
    "unwrapOrElse",
    "mapOr",
    "andThen",
    "orElse",
    "MD5",
    "SHA1",
    "hashWith",
    "HMACWith",
    "HMACBinary",
    "currentMilliseconds",
    "toMilliseconds",
    "aesGcmEncrypt",
    "aesGcmDecrypt",
    "randomBytes",
    "hkdfSha256",
    "pbkdf2Sha256",
    "constantTimeEquals",
];

/// Returns every name understood by the runtime dispatcher.
#[cfg(test)]
pub(crate) fn registry_names() -> &'static [&'static str] {
    NAMES
}

/// Returns the canonical built-in name for a function name.
pub fn builtin_name(name: &str) -> Option<&'static str> {
    NAMES.iter().copied().find(|candidate| *candidate == name)
}

/// Returns local name aliases for functions from an imported module.
pub fn module_aliases(module: &str) -> &'static [(&'static str, &'static str)] {
    match module {
        "dw::core::Binaries" => &[("fromHex", "binariesFromHex"), ("toHex", "binariesToHex")],
        "dw::core::Arrays" => &[("partition", "partition")],
        "dw::core::URL" => &[("compose", "compose")],
        "dw::core::Types" => &[
            ("isArrayType", "typesIsArrayType"),
            ("isObjectType", "typesIsObjectType"),
        ],
        "dw::extra::Common" => &[
            ("enumerate", "enumerate"),
            ("range", "range"),
            ("sorted", "sorted"),
            ("reversed", "reversed"),
            ("chunk", "chunk"),
            ("compact", "compact"),
            ("unique", "unique"),
            ("fromEntries", "fromEntries"),
            ("assign", "assign"),
            ("pick", "pick"),
            ("omit", "omit"),
            ("hasOwn", "hasOwn"),
            ("parseInt", "parseInt"),
            ("parseFloat", "parseFloat"),
            ("toFixed", "toFixed"),
            ("clamp", "clamp"),
            ("trunc", "trunc"),
            ("sign", "sign"),
            ("title", "title"),
            ("lstrip", "lstrip"),
            ("rstrip", "rstrip"),
            ("zfill", "zfill"),
            ("partition", "extraPartition"),
            ("rpartition", "rpartition"),
            ("identity", "identity"),
            ("pipe", "pipe"),
            ("compose", "extraCompose"),
            ("fold", "fold"),
            ("filterMap", "filterMap"),
            ("stepBy", "stepBy"),
            ("position", "position"),
            ("rposition", "rposition"),
            ("dedup", "dedup"),
            ("windows", "windows"),
            ("chunksExact", "chunksExact"),
            ("binarySearch", "binarySearch"),
            ("rotateLeft", "rotateLeft"),
            ("rotateRight", "rotateRight"),
            ("swap", "swap"),
            ("fill", "fill"),
            ("truncate", "truncate"),
            ("splitOnce", "splitOnce"),
            ("rsplitOnce", "rsplitOnce"),
            ("stripPrefix", "stripPrefix"),
            ("stripSuffix", "stripSuffix"),
            ("unwrapOr", "unwrapOr"),
            ("unwrapOrElse", "unwrapOrElse"),
            ("mapOr", "mapOr"),
            ("andThen", "andThen"),
            ("orElse", "orElse"),
            ("aesGcmEncrypt", "aesGcmEncrypt"),
            ("aesGcmDecrypt", "aesGcmDecrypt"),
            ("randomBytes", "randomBytes"),
            ("hkdfSha256", "hkdfSha256"),
            ("pbkdf2Sha256", "pbkdf2Sha256"),
            ("constantTimeEquals", "constantTimeEquals"),
        ],
        "dw::Runtime" => &[
            ("fail", "fail"),
            ("failIf", "failIf"),
            ("try", "try"),
            ("orElse", "runtimeOrElse"),
            ("orElseTry", "orElseTry"),
        ],
        "dw::Crypto" => &[
            ("MD5", "MD5"),
            ("SHA1", "SHA1"),
            ("hashWith", "hashWith"),
            ("HMACWith", "HMACWith"),
            ("HMACBinary", "HMACBinary"),
        ],
        "dw::util::Timer" => &[
            ("currentMilliseconds", "currentMilliseconds"),
            ("duration", "timerDuration"),
            ("time", "timerTime"),
            ("toMilliseconds", "toMilliseconds"),
        ],
        _ => &[],
    }
}

/// Returns the value of a built-in constant, if the name is one.
pub fn constant(name: &str) -> Option<Value> {
    match name {
        "ARRAY_TYPE" => Some(Value::String("Array".to_string())),
        "ATTRIBUTE_TYPE" => Some(Value::String("Attribute".to_string())),
        "OBJECT_TYPE" => Some(Value::String("Object".to_string())),
        "PI" => Number::parse("3.141592653589793").map(Value::Number),
        "E" => Number::parse("2.718281828459045").map(Value::Number),
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
        | "mapLeafValues" | "nodeExists" | "sort" | "sorted" => &[1],
        "update" | "mask" => &[2],
        "failIf" => &[1],
        "try" => &[0],
        "runtimeOrElse" | "orElseTry" => &[1],
        "fold" | "mapOr" => &[2],
        "filterMap" | "position" | "rposition" | "dedup" | "unwrapOrElse" | "andThen" => &[1],
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
        "toArray" => to_array(args),
        "toBoolean" => coerce(args, "Boolean", false),
        "toDate" => coerce_with_formatters(args, "Date", false),
        "toDateOrNull" => coerce_with_formatters(args, "Date", true),
        "toDateTime" => coerce_with_formatters(args, "DateTime", false),
        "toDateTimeOrNull" => coerce_with_formatters(args, "DateTime", true),
        "toLocalDateTime" => coerce_with_formatters(args, "LocalDateTime", false),
        "toLocalDateTimeOrNull" => coerce_with_formatters(args, "LocalDateTime", true),
        "toLocalTime" => coerce_with_formatters(args, "LocalTime", false),
        "toLocalTimeOrNull" => coerce_with_formatters(args, "LocalTime", true),
        "toNumber" => coerce_with_formatters(args, "Number", false),
        "toNumberOrNull" => coerce_with_formatters(args, "Number", true),
        "toPeriod" => coerce(args, "Period", false),
        "toRegex" => coerce(args, "Regex", false),
        "toString" => to_string(args),
        "toTime" => coerce_with_formatters(args, "Time", false),
        "toTimeOrNull" => coerce_with_formatters(args, "Time", true),
        "toTimeZone" => coerce(args, "TimeZone", false),
        "toURI" | "toUri" => coerce(args, "Uri", false),
        "attr" => value_path_element(args, "Attribute"),
        "field" => value_path_element(args, "Object"),
        "index" => value_index(args),
        "mask" | "update" => update_value(args),
        "acos" => inverse_trig(args, |value| value.acos()),
        "asin" => inverse_trig(args, |value| value.asin()),
        "atan" => Ok(math_result(number_f64(args, 0)?.atan())),
        "cos" => Ok(math_result(number_f64(args, 0)?.cos())),
        "sin" => Ok(math_result(number_f64(args, 0)?.sin())),
        "tan" => Ok(math_result(number_f64(args, 0)?.tan())),
        "toDegrees" => Ok(math_result(number_f64(args, 0)?.to_degrees())),
        "toRadians" => Ok(math_result(number_f64(args, 0)?.to_radians())),
        "log10" => logarithm(args, |value| value.log10()),
        "logn" => logarithm(args, |value| value.ln()),
        "decimalAdd" => decimal_basic(args, |a, b| a.add(b)),
        "decimalSubtract" => decimal_basic(args, |a, b| a.sub(b)),
        "decimalMultiply" => decimal_basic(args, |a, b| a.mul(b)),
        "decimalDivide" => decimal_divide(args),
        "decimalPow" => decimal_pow(args),
        "decimalSqrt" => decimal_sqrt(args),
        "decimalRound" => decimal_round(args),
        "compose" => compose(args),
        "decodeURI" => Ok(Value::String(decode_uri(&string(args, 0)?, false))),
        "decodeURIComponent" => Ok(Value::String(decode_uri(&string(args, 0)?, true))),
        "encodeURI" => Ok(Value::String(encode_uri(&string(args, 0)?, false))),
        "encodeURIComponent" => Ok(Value::String(encode_uri(&string(args, 0)?, true))),
        "parseURI" => parse_uri(args),
        "diff" => diff_values(args),
        "logDebug" | "logInfo" | "logWarn" | "logError" => level_log(args),
        "logWith" => log_with(args),
        "plusPlus" => crate::eval::concat_values(value(args, 0)?.clone(), value(args, 1)?.clone()),
        "minusMinus" => {
            crate::eval::difference_values(value(args, 0)?.clone(), value(args, 1)?.clone())
        }
        "enumerate" => extra_enumerate(args),
        "range" => extra_range(args),
        "sorted" => extra_sorted(args),
        "reversed" => extra_reversed(args),
        "chunk" => extra_chunk(args),
        "compact" => extra_compact(args),
        "unique" => extra_unique(args),
        "fromEntries" => extra_from_entries(args),
        "assign" => extra_assign(args),
        "pick" => extra_pick(args),
        "omit" => extra_omit(args),
        "hasOwn" => extra_has_own(args),
        "parseInt" => extra_parse_int(args),
        "parseFloat" => extra_parse_float(args),
        "toFixed" => extra_to_fixed(args),
        "clamp" => extra_clamp(args),
        "trunc" => extra_trunc(args),
        "sign" => extra_sign(args),
        "title" => extra_title(args),
        "lstrip" => extra_strip(args, true),
        "rstrip" => extra_strip(args, false),
        "zfill" => extra_zfill(args),
        "extraPartition" => extra_partition(args, false),
        "rpartition" => extra_partition(args, true),
        "identity" => Ok(value(args, 0)?.clone()),
        "fold" => extra_fold(args),
        "filterMap" => extra_filter_map(args),
        "stepBy" => extra_step_by(args),
        "position" => extra_position(args, false),
        "rposition" => extra_position(args, true),
        "dedup" => extra_dedup(args),
        "windows" => extra_windows(args),
        "chunksExact" => extra_chunks_exact(args),
        "binarySearch" => extra_binary_search(args),
        "rotateLeft" => extra_rotate(args, true),
        "rotateRight" => extra_rotate(args, false),
        "swap" => extra_swap(args),
        "fill" => extra_fill(args),
        "truncate" => extra_truncate(args),
        "splitOnce" => extra_split_once(args, false),
        "rsplitOnce" => extra_split_once(args, true),
        "stripPrefix" => extra_strip_affix(args, true),
        "stripSuffix" => extra_strip_affix(args, false),
        "fail" => runtime_fail(args),
        "failIf" => runtime_fail_if(args),
        "try" => runtime_try(args),
        "runtimeOrElse" => runtime_or_else(args),
        "orElseTry" => runtime_or_else_try(args),
        "unwrapOr" | "orElse" => extra_unwrap_or(args),
        "unwrapOrElse" => extra_unwrap_or_else(args),
        "mapOr" => extra_map_or(args),
        "andThen" => extra_and_then(args),
        "MD5" => crate::crypto::md5(value(args, 0)?),
        "SHA1" => crate::crypto::sha1(value(args, 0)?),
        "hashWith" => crate::crypto::hash_with(value(args, 0)?, &string(args, 1)?),
        "HMACWith" => crate::crypto::hmac_with(value(args, 0)?, value(args, 1)?, &string(args, 2)?),
        "HMACBinary" => {
            crate::crypto::hmac_binary(value(args, 0)?, value(args, 1)?, &string(args, 2)?)
        }
        "currentMilliseconds" => Ok(Value::Number(Number::from_i64(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis() as i64)
                .unwrap_or(0),
        ))),
        "toMilliseconds" => timer_to_milliseconds(args),
        "timerDuration" => timer_duration(args),
        "timerTime" => timer_wall_clock(args),
        "aesGcmEncrypt" => crate::crypto::aes_gcm_encrypt(
            value(args, 0)?,
            value(args, 1)?,
            value(args, 2)?,
            optional_value(args, 3),
        ),
        "aesGcmDecrypt" => crate::crypto::aes_gcm_decrypt(
            value(args, 0)?,
            value(args, 1)?,
            value(args, 2)?,
            optional_value(args, 3),
        ),
        "randomBytes" => crate::crypto::random_bytes(integer(args, 0)?),
        "hkdfSha256" => crate::crypto::hkdf_sha256(
            value(args, 0)?,
            optional_value(args, 1),
            optional_value(args, 2),
            args.get(3)
                .map(|_| integer(args, 3))
                .transpose()?
                .unwrap_or(32),
        ),
        "pbkdf2Sha256" => crate::crypto::pbkdf2_sha256(
            value(args, 0)?,
            value(args, 1)?,
            integer(args, 2)?,
            args.get(3)
                .map(|_| integer(args, 3))
                .transpose()?
                .unwrap_or(32),
        ),
        "constantTimeEquals" => {
            crate::crypto::constant_time_equals(value(args, 0)?, value(args, 1)?)
        }
        "arrayItem" => type_array_item(args),
        "baseTypeOf" => type_base(args),
        "functionParamTypes" => type_function_params(args),
        "functionReturnType" => type_function_return(args),
        "intersectionItems" => type_items(args, true),
        "unionItems" => type_items(args, false),
        "typesIsArrayType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Array(_))
        }),
        "typesIsObjectType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Object(_))
        }),
        "isAnyType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Any)
        }),
        "isBinaryType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Binary)
        }),
        "isBooleanType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Boolean)
        }),
        "isDateTimeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::DateTime)
        }),
        "isDateType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Date)
        }),
        "isFunctionType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Function(_, _))
        }),
        "isIntersectionType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Intersection(_))
        }),
        "isKeyType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Key)
        }),
        "isLiteralType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Literal(_))
        }),
        "isLocalDateTimeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::LocalDateTime)
        }),
        "isLocalTimeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::LocalTime)
        }),
        "isNamespaceType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Namespace)
        }),
        "isNothingType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Nothing)
        }),
        "isNullType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Null)
        }),
        "isNumberType" => type_predicate(args, |value| {
            matches!(
                value.structure(),
                crate::types::TypeValue::Number
                    | crate::types::TypeValue::Integer
                    | crate::types::TypeValue::Decimal
            )
        }),
        "isPeriodType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Period)
        }),
        "isRangeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Range)
        }),
        "isReferenceType" => type_reference(args),
        "isRegexType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Regex)
        }),
        "isStringType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::String)
        }),
        "isTimeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Time)
        }),
        "isTimeZoneType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::TimeZone)
        }),
        "isTypeType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Type)
        }),
        "isUnionType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Union(_))
        }),
        "isUriType" => type_predicate(args, |value| {
            matches!(value.structure(), crate::types::TypeValue::Uri)
        }),
        "literalValueOf" => type_literal_value(args),
        "metadataOf" => type_metadata(args),
        "nameOf" => type_name(args),
        "objectFields" => type_object_fields(args),
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
        "toBinary" => match value(args, 0)? {
            Value::String(text) => Ok(Value::Binary(text.as_bytes().to_vec())),
            _ => radix_string(args, 2, false),
        },
        "fromBinary" => radix_number(args, 2, false),
        "fromRadixNumber" => radix_number(args, 0, true),
        "toRadixNumber" => radix_string(args, 0, true),
        other => Err(DwError::new(format!("unknown function '{other}'"))),
    }
}

/// Dispatches a standard-library function that may produce a function value.
pub fn call_rvalue(name: &str, args: &[RValue]) -> Result<RValue, DwError> {
    match name {
        "pipe" => extra_sequence(args, true),
        "extraCompose" => extra_sequence(args, false),
        _ => call(name, args).map(RValue::Val),
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
        Value::Binary(bytes) => bytes.len(),
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
    // Boundary helpers operate on the ISO value and return an unformatted result.
    let value = value(args, 0)?.unwrap_formatted().clone();
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
    let value = value.unwrap_formatted();
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
        Some(RValue::Val(Value::String(text))) if !text.trim().is_empty() => {
            crate::formats::normalize_mime(text)
        }
        _ => crate::formats::normalize_mime(default),
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
            crate::formats::read_csv(&text, properties)
        }
        "application/xml" | "text/xml" => crate::formats::read_xml(&text()?, properties),
        "application/yaml" | "application/x-yaml" | "text/yaml" => {
            crate::formats::read_yaml(&text()?, properties)
        }
        other => Err(DwError::new(format!("unsupported content type '{other}'"))),
    }
}

fn write_value(args: &[RValue]) -> Result<Value, DwError> {
    let value = value(args, 0)?;
    let content_type = content_type(args, 1, "application/dw");
    let properties = optional_object(args, 2);
    write_content(value, &content_type, properties.as_ref(), &[])
}

fn write_content(
    value: &Value,
    content_type: &str,
    properties: Option<&Value>,
    namespaces: &[(String, String)],
) -> Result<Value, DwError> {
    match content_type {
        "" | "application/json" | "text/json" => {
            let options = crate::json::WriteOptions {
                indent: property_bool(properties, "indent", true),
                skip_null_on: property_string(properties, "skipNullOn"),
                duplicate_key_as_array: property_bool(properties, "duplicateKeyAsArray", false),
            };
            let value = crate::formats::strip_namespace_keys(value, namespaces);
            Ok(Value::String(crate::json::write_with(&value, &options)))
        }
        "application/dw" | "application/x-dw" | "text/x-dw" => {
            Ok(Value::String(dw_write(value, 0)))
        }
        "text/plain" => Ok(Value::String(value.key_string())),
        "application/octet-stream" | "application/x-binary" | "binary" => {
            Ok(Value::Binary(text_or_binary(value)?))
        }
        "application/csv" | "text/csv" => {
            Ok(Value::String(crate::formats::write_csv(value, properties)?))
        }
        "application/xml" | "text/xml" => Ok(Value::String(crate::formats::write_xml(
            value, properties, namespaces,
        )?)),
        "application/yaml" | "application/x-yaml" | "text/yaml" => Ok(Value::String(
            crate::formats::write_yaml(value, properties)?,
        )),
        other => Err(DwError::new(format!("unsupported content type '{other}'"))),
    }
}

/// Writes a value using the MIME type from a script output declaration.
pub(crate) fn write_output(
    value: &Value,
    content_type: &str,
    properties: Option<&Value>,
    namespaces: &[(String, String)],
) -> Result<String, DwError> {
    match write_content(value, content_type, properties, namespaces)? {
        Value::String(text) => Ok(text),
        other => Ok(other.key_string()),
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
        Value::Type(value) => out.push_str(&value.display()),
        Value::Formatted(_, display) => out.push_str(display),
        Value::Meta(inner, _) => out.push_str(&dw_write(inner, level)),
    }
    out
}

fn mime_from_path(path: &str) -> &'static str {
    let path = path.split(['?', '#']).next().unwrap_or(path);
    let path = path.to_ascii_lowercase();
    if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".csv") {
        "application/csv"
    } else if path.ends_with(".xml") {
        "application/xml"
    } else if path.ends_with(".yaml") || path.ends_with(".yml") {
        "application/yaml"
    } else {
        "text/plain"
    }
}

fn read_url(args: &[RValue]) -> Result<Value, DwError> {
    let url = string(args, 0)?;
    let content_type = match args.get(1) {
        Some(RValue::Val(Value::String(text))) if !text.trim().is_empty() => {
            crate::formats::normalize_mime(text)
        }
        _ => mime_from_path(&url).to_string(),
    };
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

fn value_path_element(args: &[RValue], kind: &str) -> Result<Value, DwError> {
    let (namespace, name) = match args.len() {
        0 => return Err(DwError::new("missing name")),
        1 => (Value::Null, value(args, 0)?.clone()),
        _ => (value(args, 0)?.clone(), value(args, 1)?.clone()),
    };
    Ok(Value::Object(vec![
        ("kind".to_string(), Value::String(kind.to_string())),
        ("namespace".to_string(), namespace),
        ("selector".to_string(), Value::String(name.key_string())),
    ]))
}

fn value_index(args: &[RValue]) -> Result<Value, DwError> {
    Ok(Value::Object(vec![
        ("kind".to_string(), Value::String("Array".to_string())),
        ("namespace".to_string(), Value::Null),
        ("selector".to_string(), Value::Number(number(args, 0)?)),
    ]))
}

fn to_array(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)? {
        Value::String(text) => Ok(Value::Array(
            text.chars().map(|c| Value::String(c.to_string())).collect(),
        )),
        Value::Array(_) => Ok(value(args, 0)?.clone()),
        Value::Null => Ok(Value::Array(Vec::new())),
        other => Ok(Value::Array(vec![other.clone()])),
    }
}

fn options_object(entries: Vec<(&str, Value)>) -> Value {
    Value::Object(
        entries
            .into_iter()
            .filter(|(_, value)| !value.is_null())
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )
}

fn coercion_options(args: &[RValue], target: &str) -> Option<Value> {
    match args.get(1) {
        Some(RValue::Val(Value::Object(entries))) => Some(Value::Object(entries.clone())),
        Some(RValue::Val(Value::String(text))) if !text.is_empty() => {
            let key = match target {
                "Number" => match args.first() {
                    Some(RValue::Val(Value::DateTime(_) | Value::Period(_))) => "unit",
                    _ => "format",
                },
                _ => "format",
            };
            Some(options_object(vec![(key, Value::String(text.clone()))]))
        }
        _ => None,
    }
}

fn coerce(args: &[RValue], target: &str, or_null: bool) -> Result<Value, DwError> {
    let value = value(args, 0)?.clone();
    let options = coercion_options(args, target);
    match crate::eval::cast_value(value, target, options.as_ref()) {
        Ok(value) => Ok(value),
        Err(error) if or_null => {
            let _ = error;
            Ok(Value::Null)
        }
        Err(error) => Err(error),
    }
}

fn coerce_with_formatters(args: &[RValue], target: &str, or_null: bool) -> Result<Value, DwError> {
    if let Some(RValue::Val(Value::Array(formatters))) = args.get(1) {
        for formatter in formatters {
            let options = match formatter {
                Value::Object(entries) => Value::Object(entries.clone()),
                Value::String(text) => {
                    options_object(vec![("format", Value::String(text.clone()))])
                }
                _ => continue,
            };
            if let Ok(value) =
                crate::eval::cast_value(value(args, 0)?.clone(), target, Some(&options))
            {
                return Ok(value);
            }
        }
        if or_null {
            return Ok(Value::Null);
        }
        return Err(DwError::new(format!(
            "could not find a valid formatter for '{}'",
            value(args, 0)?.key_string()
        )));
    }
    coerce(args, target, or_null)
}

fn to_string(args: &[RValue]) -> Result<Value, DwError> {
    let input = value(args, 0)?.clone();
    if let Value::Binary(bytes) = &input {
        if let Some(RValue::Val(Value::String(charset))) = args.get(1) {
            return decode_charset(bytes, charset).map(Value::String);
        }
    }
    let options = coercion_options(args, "String");
    crate::eval::cast_value(input, "String", options.as_ref())
}

fn update_selector(selector: &Value) -> Result<Value, DwError> {
    match selector {
        Value::String(_) | Value::Number(_) => Ok(selector.clone()),
        Value::Object(_) => Ok(selector.clone()),
        Value::Array(_) => Ok(selector.clone()),
        other => Err(DwError::new(format!(
            "invalid update selector {}",
            other.type_name()
        ))),
    }
}

fn selector_kind(selector: &Value) -> Option<(String, Value)> {
    let entries = selector.as_object()?;
    let part = |name: &str| {
        entries
            .iter()
            .rev()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.clone())
    };
    Some((part("kind")?.key_string(), part("selector")?))
}

fn apply_updater(new_value: &RValue, old: &Value, index: usize) -> Result<Value, DwError> {
    match new_value {
        RValue::Val(value) => Ok(value.clone()),
        RValue::Func(_) | RValue::Builtin(_) | RValue::Overloads(_) | RValue::Sequence(_, _) => {
            call_fn(
                new_value,
                vec![old.clone(), Value::Number(Number::from_usize(index))],
            )
        }
        RValue::FunctionList(_) => Err(DwError::new("a function list is not a value provider")),
        RValue::Regex(_) => Err(DwError::new("a regular expression is not a value provider")),
    }
}

fn update_field(
    object: &Value,
    field: &str,
    updater: &RValue,
    index: usize,
) -> Result<Value, DwError> {
    let mut object = object.clone();
    match &object {
        Value::Object(entries) => {
            let old = entries
                .iter()
                .rev()
                .find(|(key, _)| key == field)
                .map(|(_, value)| value.clone())
                .unwrap_or(Value::Null);
            let new = apply_updater(updater, &old, index)?;
            object.insert(field.to_string(), new);
            Ok(object)
        }
        _ => Err(DwError::new("expected an Object")),
    }
}

fn update_at(
    value: &Value,
    selector: &Value,
    updater: &RValue,
    index: usize,
) -> Result<Value, DwError> {
    if let Value::Array(path) = selector {
        if path.is_empty() {
            return apply_updater(updater, value, index);
        }
        let first = update_selector(&path[0])?;
        let rest = if path.len() > 1 {
            Value::Array(path[1..].to_vec())
        } else {
            Value::Null
        };
        let old = get_at(value, &first)?;
        let new = if rest.is_null() {
            apply_updater(updater, &old, index)?
        } else {
            update_at(&old, &rest, updater, index)?
        };
        return set_at(value, &first, new);
    }
    if let Some((kind, selector)) = selector_kind(selector) {
        return match kind.as_str() {
            "Array" => update_at(value, &selector, updater, index),
            _ => match value {
                Value::Object(_) => update_field(value, &selector.key_string(), updater, index),
                Value::Array(items) => {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        out.push(update_field(item, &selector.key_string(), updater, index)?);
                    }
                    Ok(Value::Array(out))
                }
                _ => Err(DwError::new("expected an Object or Array")),
            },
        };
    }
    match (value, selector) {
        (Value::Object(_), Value::String(field)) => update_field(value, field, updater, index),
        (Value::Array(items), Value::Number(number)) => {
            let position = number
                .to_i64()
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            let position = if position < 0 {
                items.len() as i64 + position
            } else {
                position
            };
            if position < 0 || position as usize >= items.len() {
                return Err(DwError::new("array index is out of range"));
            }
            let position = position as usize;
            let new = apply_updater(updater, &items[position], position)?;
            let mut out = items.clone();
            out[position] = new;
            Ok(Value::Array(out))
        }
        (Value::Array(items), Value::String(field)) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(update_field(item, field, updater, index)?);
            }
            Ok(Value::Array(out))
        }
        (Value::Null, Value::String(field)) => {
            let mut object = Value::Object(Vec::new());
            object.insert(field.clone(), apply_updater(updater, &Value::Null, index)?);
            Ok(object)
        }
        _ => Err(DwError::new("cannot update this value with the selector")),
    }
}

fn get_at(value: &Value, selector: &Value) -> Result<Value, DwError> {
    match (value, selector) {
        (Value::Object(_), Value::String(field)) => {
            Ok(value.get(field).cloned().unwrap_or(Value::Null))
        }
        (Value::Array(items), Value::Number(number)) => {
            let position = number
                .to_i64()
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            let position = if position < 0 {
                items.len() as i64 + position
            } else {
                position
            };
            Ok(items.get(position as usize).cloned().unwrap_or(Value::Null))
        }
        (_, Value::Object(_)) => {
            if let Some((kind, selector)) = selector_kind(selector) {
                if kind == "Array" {
                    get_at(value, &selector)
                } else {
                    get_at(value, &Value::String(selector.key_string()))
                }
            } else {
                Ok(Value::Null)
            }
        }
        _ => Ok(Value::Null),
    }
}

fn set_at(value: &Value, selector: &Value, new: Value) -> Result<Value, DwError> {
    match (value, selector) {
        (Value::Object(_), Value::String(field)) => {
            let mut object = value.clone();
            object.insert(field.clone(), new);
            Ok(object)
        }
        (Value::Array(items), Value::Number(number)) => {
            let position = number
                .to_i64()
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            let position = if position < 0 {
                items.len() as i64 + position
            } else {
                position
            };
            if position < 0 || position as usize >= items.len() {
                return Err(DwError::new("array index is out of range"));
            }
            let mut out = items.clone();
            out[position as usize] = new;
            Ok(Value::Array(out))
        }
        (_, Value::Object(_)) => {
            if let Some((kind, selector)) = selector_kind(selector) {
                if kind == "Array" {
                    set_at(value, &selector, new)
                } else {
                    set_at(value, &Value::String(selector.key_string()), new)
                }
            } else {
                Err(DwError::new("invalid update selector"))
            }
        }
        _ => Err(DwError::new("cannot update this value with the selector")),
    }
}

fn update_value(args: &[RValue]) -> Result<Value, DwError> {
    let target = value(args, 0)?;
    let selector = value(args, 1)?;
    let updater = args
        .get(2)
        .ok_or_else(|| DwError::new("missing update value"))?;
    let selector = update_selector(selector)?;
    update_at(target, &selector, updater, 0)
}

fn number_f64(args: &[RValue], index: usize) -> Result<f64, DwError> {
    number(args, index)?
        .to_f64()
        .ok_or_else(|| DwError::new("number is out of range"))
}

fn math_result(value: f64) -> Value {
    if !value.is_finite() {
        return Value::Null;
    }
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn inverse_trig<F>(args: &[RValue], f: F) -> Result<Value, DwError>
where
    F: FnOnce(f64) -> f64,
{
    let value = number_f64(args, 0)?;
    if value.abs() > 1.0 {
        return Ok(Value::Null);
    }
    Ok(math_result(f(value)))
}

fn logarithm<F>(args: &[RValue], f: F) -> Result<Value, DwError>
where
    F: FnOnce(f64) -> f64,
{
    let value = number_f64(args, 0)?;
    if value <= 0.0 {
        return Ok(Value::Null);
    }
    Ok(math_result(f(value)))
}

fn operation_context(args: &[RValue], index: usize) -> Option<Value> {
    match args.get(index) {
        Some(RValue::Val(value @ Value::Object(_))) => Some(value.clone()),
        _ => None,
    }
}

fn context_precision(context: Option<&Value>) -> Option<i64> {
    context
        .and_then(|value| value.get("precision"))
        .and_then(Value::as_number)
        .and_then(Number::to_i64)
}

fn context_mode(context: Option<&Value>) -> String {
    context
        .and_then(|value| value.get("roundingMode"))
        .map(Value::key_string)
        .unwrap_or_else(|| "HALF_UP".to_string())
}

fn round_abs(number: &Number, mode: &str) -> Number {
    let value = number.abs();
    let floor = value.floor();
    let fraction = value.sub(&floor);
    let half = Number::parse("0.5").expect("half");
    let one = Number::from_i64(1);
    match mode {
        "UP" => value.ceil(),
        "DOWN" => floor,
        "CEILING" | "FLOOR" => value.ceil(),
        "HALF_DOWN" => {
            if fraction == half {
                floor
            } else {
                value.round()
            }
        }
        "HALF_EVEN" => {
            if fraction == half {
                if floor
                    .rem(&Number::from_i64(2))
                    .map(|r| r.is_zero())
                    .unwrap_or(false)
                {
                    floor
                } else {
                    floor.add(&one)
                }
            } else {
                value.round()
            }
        }
        _ => value.round(),
    }
}

fn round_significant(number: &Number, precision: i64, mode: &str) -> Number {
    if precision <= 0 || number.is_zero() {
        return number.clone();
    }
    let abs = number.abs();
    let value = match abs.to_f64() {
        Some(value) if value > 0.0 && value.is_finite() => value,
        _ => return number.clone(),
    };
    let exponent = value.log10().floor() as i64 + 1;
    let places = exponent - precision;
    let factor = Number::from_i64(10)
        .pow(&Number::from_i64(places))
        .unwrap_or(Number::from_i64(1));
    if factor.is_zero() {
        return number.clone();
    }
    let scaled = match number.div(&factor) {
        Some(value) => value,
        None => return number.clone(),
    };
    let rounded = if scaled.is_negative() {
        let abs = round_abs(&scaled, mode);
        match mode {
            "CEILING" => abs.neg(),
            "FLOOR" => abs.neg(),
            _ => abs.neg(),
        }
    } else {
        match mode {
            "DOWN" => scaled.floor(),
            "FLOOR" => scaled.floor(),
            _ => round_abs(&scaled, mode),
        }
    };
    rounded.mul(&factor)
}

fn decimal_basic<F>(args: &[RValue], operation: F) -> Result<Value, DwError>
where
    F: FnOnce(&Number, &Number) -> Number,
{
    let left = number(args, 0)?;
    let right = number(args, 1)?;
    let mut result = operation(&left, &right);
    if let Some(context) = operation_context(args, 2) {
        if let Some(precision) = context_precision(Some(&context)) {
            result = round_significant(&result, precision, &context_mode(Some(&context)));
        }
    }
    Ok(Value::Number(result))
}

fn decimal_divide(args: &[RValue]) -> Result<Value, DwError> {
    let dividend = number(args, 0)?;
    let divisor = number(args, 1)?;
    let mut result = dividend
        .div(&divisor)
        .ok_or_else(|| DwError::new("division by zero"))?;
    if let Some(context) = operation_context(args, 2) {
        if let Some(precision) = context_precision(Some(&context)) {
            result = round_significant(&result, precision, &context_mode(Some(&context)));
        }
    }
    Ok(Value::Number(result))
}

fn decimal_pow(args: &[RValue]) -> Result<Value, DwError> {
    let base = number(args, 0)?;
    let exponent = number(args, 1)?;
    let mut result = base
        .pow(&exponent)
        .ok_or_else(|| DwError::new("power is out of range"))?;
    if let Some(context) = operation_context(args, 2) {
        if let Some(precision) = context_precision(Some(&context)) {
            result = round_significant(&result, precision, &context_mode(Some(&context)));
        }
    }
    Ok(Value::Number(result))
}

fn decimal_sqrt(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    let mut result = value
        .sqrt()
        .ok_or_else(|| DwError::new("square root of a negative number"))?;
    if let Some(context) = operation_context(args, 1) {
        if let Some(precision) = context_precision(Some(&context)) {
            result = round_significant(&result, precision, &context_mode(Some(&context)));
        }
    }
    Ok(Value::Number(result))
}

fn decimal_round(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    let context = operation_context(args, 1);
    let precision = context_precision(context.as_ref()).unwrap_or(34);
    let mode = context_mode(context.as_ref());
    Ok(Value::Number(round_significant(&value, precision, &mode)))
}

fn encode_uri(text: &str, component: bool) -> String {
    let mut out = String::new();
    for byte in text.bytes() {
        let c = byte as char;
        let unreserved = c.is_ascii_alphanumeric() || "-_.!~*'()".contains(c);
        let reserved = component || !";,/?:@&=+$#".contains(c);
        if unreserved || !reserved {
            out.push(c);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn decode_uri(text: &str, component: bool) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (
                (bytes[index + 1] as char).to_digit(16),
                (bytes[index + 2] as char).to_digit(16),
            ) {
                let decoded = (high * 16 + low) as u8;
                let c = decoded as char;
                let keep = !component && ";,/?:@&=+$#".contains(c);
                if keep {
                    out.extend_from_slice(&bytes[index..index + 3]);
                } else {
                    out.push(decoded);
                }
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn extra_enumerate(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let start = if args.len() > 1 { integer(args, 1)? } else { 0 };
    let mut out = Vec::with_capacity(items.len());
    for (offset, item) in items.into_iter().enumerate() {
        let index = start
            .checked_add(offset as i64)
            .ok_or_else(|| DwError::new("enumerate index overflow"))?;
        out.push(Value::Object(vec![
            ("index".to_string(), Value::Number(Number::from_i64(index))),
            ("value".to_string(), item),
        ]));
    }
    Ok(Value::Array(out))
}

fn extra_range(args: &[RValue]) -> Result<Value, DwError> {
    let (start, stop, step) = match args.len() {
        1 => (0, integer(args, 0)?, 1),
        2 => (integer(args, 0)?, integer(args, 1)?, 1),
        _ => (integer(args, 0)?, integer(args, 1)?, integer(args, 2)?),
    };
    if step == 0 {
        return Err(DwError::new("range step must not be zero"));
    }
    let mut out = Vec::new();
    let mut current = start;
    if step > 0 {
        while current < stop {
            out.push(Value::Number(Number::from_i64(current)));
            match current.checked_add(step) {
                Some(next) => current = next,
                None => break,
            }
        }
    } else {
        while current > stop {
            out.push(Value::Number(Number::from_i64(current)));
            match current.checked_add(step) {
                Some(next) => current = next,
                None => break,
            }
        }
    }
    Ok(Value::Array(out))
}

fn extra_compare_keys(left: &Value, right: &Value, reverse: bool) -> Result<Ordering, DwError> {
    let left = left.unwrap_formatted();
    let right = right.unwrap_formatted();
    match (left.is_null(), right.is_null()) {
        (true, true) => Ok(Ordering::Equal),
        (true, _) => Ok(if reverse {
            Ordering::Less
        } else {
            Ordering::Greater
        }),
        (_, true) => Ok(if reverse {
            Ordering::Greater
        } else {
            Ordering::Less
        }),
        _ => {
            let ordering = match (left, right) {
                (Value::Number(a), Value::Number(b)) => Some(a.cmp(b)),
                (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
                (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
                _ => None,
            }
            .ok_or_else(|| DwError::new("sorted values are not comparable"))?;
            Ok(if reverse {
                ordering.reverse()
            } else {
                ordering
            })
        }
    }
}

fn extra_sorted(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let key_function = if args.len() > 1 {
        Some(function(args, 1)?)
    } else {
        None
    };
    let reverse = match args.get(2) {
        Some(RValue::Val(Value::Bool(value))) => *value,
        Some(_) => return Err(DwError::new("sorted reverse must be a Boolean")),
        None => false,
    };
    let mut keyed = Vec::with_capacity(items.len());
    for item in items {
        let key = match key_function {
            Some(function) => call_simple(function, std::slice::from_ref(&item))?,
            None => item.clone(),
        };
        keyed.push((key, item));
    }
    let mut error = None;
    keyed.sort_by(|(left, _), (right, _)| {
        if error.is_some() {
            return Ordering::Equal;
        }
        match extra_compare_keys(left, right, reverse) {
            Ok(ordering) => ordering,
            Err(failure) => {
                error = Some(failure);
                Ordering::Equal
            }
        }
    });
    if let Some(failure) = error {
        return Err(failure);
    }
    Ok(Value::Array(
        keyed.into_iter().map(|(_, item)| item).collect(),
    ))
}

fn extra_reversed(args: &[RValue]) -> Result<Value, DwError> {
    let mut items = array(args, 0)?;
    items.reverse();
    Ok(Value::Array(items))
}

fn extra_chunk(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let size = integer(args, 1)?;
    if size <= 0 {
        return Err(DwError::new("chunk size must be a positive integer"));
    }
    let chunks = items
        .chunks(size as usize)
        .map(|chunk| Value::Array(chunk.to_vec()))
        .collect();
    Ok(Value::Array(chunks))
}

fn extra_compact(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    Ok(Value::Array(
        items.into_iter().filter(|item| !item.is_null()).collect(),
    ))
}

fn extra_unique(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let mut seen: Vec<Value> = Vec::new();
    let mut out = Vec::new();
    for item in items {
        let normalized = item.unwrap_formatted().clone();
        if !seen.iter().any(|value| value == &normalized) {
            seen.push(normalized);
            out.push(item);
        }
    }
    Ok(Value::Array(out))
}

fn extra_from_entries(args: &[RValue]) -> Result<Value, DwError> {
    let entries = array(args, 0)?;
    let mut out: Vec<(String, Value)> = Vec::new();
    for entry in entries {
        let (key, value) = match entry {
            Value::Object(fields) => {
                let mut key = None;
                let mut value = None;
                for (name, field) in fields {
                    match name.as_str() {
                        "key" => key = Some(field),
                        "value" => value = Some(field),
                        _ => {}
                    }
                }
                let key = match key {
                    Some(Value::String(text)) => text,
                    Some(other) => {
                        return Err(DwError::new(format!(
                            "entry key must be a String, found {}",
                            other.type_name()
                        )))
                    }
                    None => return Err(DwError::new("entry is missing its key")),
                };
                let value = value.ok_or_else(|| DwError::new("entry is missing its value"))?;
                (key, value)
            }
            Value::Array(items) => {
                if items.len() != 2 {
                    return Err(DwError::new("array entries must have two elements"));
                }
                let key = match &items[0] {
                    Value::String(text) => text.clone(),
                    other => {
                        return Err(DwError::new(format!(
                            "entry key must be a String, found {}",
                            other.type_name()
                        )))
                    }
                };
                (key, items[1].clone())
            }
            other => {
                return Err(DwError::new(format!(
                    "entries must be objects or arrays, found {}",
                    other.type_name()
                )))
            }
        };
        if let Some(existing) = out.iter_mut().find(|(name, _)| name == &key) {
            existing.1 = value;
        } else {
            out.push((key, value));
        }
    }
    Ok(Value::Object(out))
}

fn extra_assign(args: &[RValue]) -> Result<Value, DwError> {
    let target = object(args, 0)?;
    let source = object(args, 1)?;
    let mut out = target;
    for (key, value) in source {
        if let Some(position) = out.iter().position(|(name, _)| name == &key) {
            out[position] = (key.clone(), value);
            let mut index = out.len();
            while index > position + 1 {
                index -= 1;
                if out[index].0 == key {
                    out.remove(index);
                }
            }
        } else {
            out.push((key, value));
        }
    }
    Ok(Value::Object(out))
}

fn extra_key_set(args: &[RValue], index: usize) -> Result<HashSet<String>, DwError> {
    let mut keys = HashSet::new();
    for item in array(args, index)? {
        match item {
            Value::String(text) => {
                keys.insert(text);
            }
            other => {
                return Err(DwError::new(format!(
                    "expected String keys, found {}",
                    other.type_name()
                )))
            }
        }
    }
    Ok(keys)
}

fn extra_pick(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let keys = extra_key_set(args, 1)?;
    Ok(Value::Object(
        entries
            .into_iter()
            .filter(|(key, _)| keys.contains(key))
            .collect(),
    ))
}

fn extra_omit(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let keys = extra_key_set(args, 1)?;
    Ok(Value::Object(
        entries
            .into_iter()
            .filter(|(key, _)| !keys.contains(key))
            .collect(),
    ))
}

fn extra_has_own(args: &[RValue]) -> Result<Value, DwError> {
    let entries = object(args, 0)?;
    let key = string(args, 1)?;
    Ok(Value::Bool(entries.iter().any(|(name, _)| name == &key)))
}

fn extra_parse_int(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let radix = if args.len() > 1 {
        let value = number(args, 1)?;
        value
            .to_i64()
            .filter(|radix| (2..=36).contains(radix))
            .ok_or_else(|| DwError::new("parseInt radix must be an integer from 2 to 36"))?
    } else {
        10
    };
    let mut chars = text.trim_start().chars().peekable();
    let mut negative = false;
    if let Some(sign) = chars.peek() {
        match sign {
            '+' => {
                chars.next();
            }
            '-' => {
                negative = true;
                chars.next();
            }
            _ => {}
        }
    }
    let mut digits = String::new();
    for character in chars {
        match character.to_digit(radix as u32) {
            Some(digit) => digits.push(char::from_digit(digit, radix as u32).unwrap_or('0')),
            None => break,
        }
    }
    if digits.is_empty() {
        return Ok(Value::Null);
    }
    let coefficient = BigUint::parse_bytes(digits.as_bytes(), radix as u32)
        .ok_or_else(|| DwError::new("invalid integer digits"))?;
    let decimal = coefficient.to_str_radix(10);
    let sign = if negative && digits.chars().any(|digit| digit != '0') {
        "-"
    } else {
        ""
    };
    Number::parse(&format!("{sign}{decimal}"))
        .map(Value::Number)
        .ok_or_else(|| DwError::new("invalid integer"))
}

fn extra_number_prefix(text: &str) -> Option<&str> {
    let bytes = text.as_bytes();
    let mut index = 0;
    if index < bytes.len() && (bytes[index] == b'+' || bytes[index] == b'-') {
        index += 1;
    }
    let mut seen_digit = false;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        seen_digit = true;
        index += 1;
    }
    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            seen_digit = true;
            index += 1;
        }
    }
    if !seen_digit {
        return None;
    }
    if index < bytes.len() && (bytes[index] == b'e' || bytes[index] == b'E') {
        let mut exponent = index + 1;
        if exponent < bytes.len() && (bytes[exponent] == b'+' || bytes[exponent] == b'-') {
            exponent += 1;
        }
        let digits_start = exponent;
        while exponent < bytes.len() && bytes[exponent].is_ascii_digit() {
            exponent += 1;
        }
        if exponent > digits_start {
            index = exponent;
        }
    }
    Some(&text[..index])
}

fn extra_parse_float(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let trimmed = text.trim_start();
    match extra_number_prefix(trimmed) {
        Some(prefix) => Number::parse(prefix)
            .map(Value::Number)
            .ok_or_else(|| DwError::new("invalid number")),
        None => Ok(Value::Null),
    }
}

fn extra_to_fixed(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    let digits = if args.len() > 1 { integer(args, 1)? } else { 0 };
    if !(0..=20).contains(&digits) {
        return Err(DwError::new("toFixed digits must be from 0 to 20"));
    }
    let factor = Number::parse(&format!("1{}", "0".repeat(digits as usize)))
        .ok_or_else(|| DwError::new("invalid scale"))?;
    let scaled = value.mul(&factor).round();
    let mut text = scaled.to_plain_string();
    if digits == 0 {
        return Ok(Value::String(text));
    }
    let negative = text.starts_with('-');
    if negative {
        text.remove(0);
    }
    let width = digits as usize + 1;
    if text.len() < width {
        text = format!("{}{}", "0".repeat(width - text.len()), text);
    }
    let split = text.len() - digits as usize;
    let mut out = String::new();
    if negative {
        out.push('-');
    }
    out.push_str(&text[..split]);
    out.push('.');
    out.push_str(&text[split..]);
    Ok(Value::String(out))
}

fn extra_clamp(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    let minimum = number(args, 1)?;
    let maximum = number(args, 2)?;
    if minimum > maximum {
        return Err(DwError::new("clamp minimum must not exceed maximum"));
    }
    if value < minimum {
        Ok(Value::Number(minimum))
    } else if value > maximum {
        Ok(Value::Number(maximum))
    } else {
        Ok(Value::Number(value))
    }
}

fn extra_trunc(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    Ok(Value::Number(if value.is_negative() {
        value.ceil()
    } else {
        value.floor()
    }))
}

fn extra_sign(args: &[RValue]) -> Result<Value, DwError> {
    let value = number(args, 0)?;
    let sign = if value.is_zero() {
        0
    } else if value.is_negative() {
        -1
    } else {
        1
    };
    Ok(Value::Number(Number::from_i64(sign)))
}

fn extra_title(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let mut out = String::new();
    let mut word_start = true;
    for character in text.chars() {
        if character.is_alphabetic() {
            if word_start {
                out.extend(character.to_uppercase());
            } else {
                out.extend(character.to_lowercase());
            }
            word_start = false;
        } else {
            out.push(character);
            word_start = true;
        }
    }
    Ok(Value::String(out))
}

fn extra_strip(args: &[RValue], leading: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let chars = match args.get(1) {
        None => None,
        Some(RValue::Val(Value::Null)) => None,
        Some(RValue::Val(Value::String(chars))) => Some(chars.clone()),
        Some(_) => return Err(DwError::new("strip characters must be a String or null")),
    };
    let stripped = match chars {
        None if leading => text.trim_start().to_string(),
        None => text.trim_end().to_string(),
        Some(chars) if leading => text
            .trim_start_matches(|character| chars.contains(character))
            .to_string(),
        Some(chars) => text
            .trim_end_matches(|character| chars.contains(character))
            .to_string(),
    };
    Ok(Value::String(stripped))
}

fn extra_zfill(args: &[RValue]) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let width = integer(args, 1)?;
    if width < 0 {
        return Err(DwError::new("zfill width must not be negative"));
    }
    let width = width as usize;
    let length = text.chars().count();
    if length >= width {
        return Ok(Value::String(text));
    }
    let zeros = "0".repeat(width - length);
    let mut chars = text.chars();
    let output = match chars.next() {
        Some(sign @ ('+' | '-')) => format!("{sign}{zeros}{}", chars.as_str()),
        _ => format!("{zeros}{text}"),
    };
    Ok(Value::String(output))
}

fn extra_partition(args: &[RValue], last: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let separator = string(args, 1)?;
    if separator.is_empty() {
        return Err(DwError::new("partition separator must not be empty"));
    }
    let position = if last {
        text.rfind(&separator)
    } else {
        text.find(&separator)
    };
    let parts = match position {
        Some(index) => vec![
            Value::String(text[..index].to_string()),
            Value::String(separator.clone()),
            Value::String(text[index + separator.len()..].to_string()),
        ],
        None if last => vec![
            Value::String(String::new()),
            Value::String(String::new()),
            Value::String(text),
        ],
        None => vec![
            Value::String(text),
            Value::String(String::new()),
            Value::String(String::new()),
        ],
    };
    Ok(Value::Array(parts))
}

fn extra_sequence(args: &[RValue], left_to_right: bool) -> Result<RValue, DwError> {
    let functions = match args.first() {
        Some(RValue::FunctionList(items)) => items.clone(),
        Some(RValue::Val(Value::Array(items))) if items.is_empty() => Vec::new(),
        Some(RValue::Val(Value::Array(_))) => {
            return Err(DwError::new("expected an array of functions"))
        }
        Some(_) => return Err(DwError::new("expected an array of functions")),
        None => return Err(DwError::new("missing function array")),
    };
    for function in &functions {
        if !matches!(
            function,
            RValue::Func(_) | RValue::Overloads(_) | RValue::Builtin(_) | RValue::Sequence(_, _)
        ) {
            return Err(DwError::new("every item must be a function"));
        }
    }
    Ok(RValue::Sequence(functions, left_to_right))
}

fn extra_fold(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let mut accumulator = value(args, 1)?.clone();
    let function = function(args, 2)?;
    for item in items {
        accumulator = call_simple(function, &[item, accumulator])?;
    }
    Ok(accumulator)
}

fn extra_filter_map(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let function = function(args, 1)?;
    let mut out = Vec::new();
    for item in items {
        let mapped = call_simple(function, std::slice::from_ref(&item))?;
        if !mapped.unwrap_formatted().is_null() {
            out.push(mapped);
        }
    }
    Ok(Value::Array(out))
}

fn extra_step_by(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let step = integer(args, 1)?;
    if step <= 0 {
        return Err(DwError::new("stepBy step must be a positive integer"));
    }
    Ok(Value::Array(
        items.into_iter().step_by(step as usize).collect(),
    ))
}

fn extra_position(args: &[RValue], reverse: bool) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let predicate = function(args, 1)?;
    let found = if reverse {
        let mut found = -1;
        for (index, item) in items.iter().enumerate().rev() {
            if call_simple(predicate, std::slice::from_ref(item))?.truthy() {
                found = index as i64;
                break;
            }
        }
        found
    } else {
        let mut found = -1;
        for (index, item) in items.iter().enumerate() {
            if call_simple(predicate, std::slice::from_ref(item))?.truthy() {
                found = index as i64;
                break;
            }
        }
        found
    };
    Ok(Value::Number(Number::from_i64(found)))
}

fn extra_dedup(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let key_function = if args.len() > 1 {
        Some(function(args, 1)?)
    } else {
        None
    };
    let mut previous: Option<Value> = None;
    let mut out = Vec::new();
    for item in items {
        let key = match key_function {
            Some(function) => call_simple(function, std::slice::from_ref(&item))?,
            None => item.clone(),
        };
        let key = key.unwrap_formatted().clone();
        if previous.as_ref() != Some(&key) {
            previous = Some(key);
            out.push(item);
        }
    }
    Ok(Value::Array(out))
}

fn extra_windows(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let size = integer(args, 1)?;
    if size <= 0 {
        return Err(DwError::new("windows size must be a positive integer"));
    }
    let size = size as usize;
    if size > items.len() {
        return Ok(Value::Array(Vec::new()));
    }
    Ok(Value::Array(
        items
            .windows(size)
            .map(|window| Value::Array(window.to_vec()))
            .collect(),
    ))
}

fn extra_chunks_exact(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let size = integer(args, 1)?;
    if size <= 0 {
        return Err(DwError::new("chunksExact size must be a positive integer"));
    }
    Ok(Value::Array(
        items
            .chunks_exact(size as usize)
            .map(|chunk| Value::Array(chunk.to_vec()))
            .collect(),
    ))
}

fn extra_binary_search(args: &[RValue]) -> Result<Value, DwError> {
    let items = array(args, 0)?;
    let needle = value(args, 1)?;
    let mut low = 0usize;
    let mut high = items.len();
    while low < high {
        let middle = low + (high - low) / 2;
        if extra_compare_keys(&items[middle], needle, false)? == Ordering::Less {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    let found =
        low < items.len() && extra_compare_keys(&items[low], needle, false)? == Ordering::Equal;
    Ok(Value::Object(vec![
        ("found".to_string(), Value::Bool(found)),
        ("index".to_string(), Value::Number(Number::from_usize(low))),
    ]))
}

fn extra_rotate(args: &[RValue], left: bool) -> Result<Value, DwError> {
    let mut items = array(args, 0)?;
    let count = integer(args, 1)?;
    if items.is_empty() {
        return Ok(Value::Array(items));
    }
    let shift = count.rem_euclid(items.len() as i64) as usize;
    if left {
        items.rotate_left(shift);
    } else {
        items.rotate_right(shift);
    }
    Ok(Value::Array(items))
}

fn extra_swap(args: &[RValue]) -> Result<Value, DwError> {
    let mut items = array(args, 0)?;
    let left = integer(args, 1)?;
    let right = integer(args, 2)?;
    let length = items.len() as i64;
    if left < 0 || right < 0 || left >= length || right >= length {
        return Err(DwError::new("swap indices are out of bounds"));
    }
    items.swap(left as usize, right as usize);
    Ok(Value::Array(items))
}

fn extra_fill(args: &[RValue]) -> Result<Value, DwError> {
    let mut items = array(args, 0)?;
    let replacement = value(args, 1)?.clone();
    let length = items.len() as i64;
    let start = if args.len() > 2 { integer(args, 2)? } else { 0 };
    let end = if args.len() > 3 {
        integer(args, 3)?
    } else {
        length
    };
    if start < 0 || end < start || end > length {
        return Err(DwError::new("fill bounds are invalid"));
    }
    for slot in &mut items[start as usize..end as usize] {
        *slot = replacement.clone();
    }
    Ok(Value::Array(items))
}

fn extra_truncate(args: &[RValue]) -> Result<Value, DwError> {
    let mut items = array(args, 0)?;
    let length = integer(args, 1)?;
    if length < 0 {
        return Err(DwError::new(
            "truncate length must be a non-negative integer",
        ));
    }
    items.truncate(length as usize);
    Ok(Value::Array(items))
}

fn extra_split_once(args: &[RValue], last: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let separator = string(args, 1)?;
    if separator.is_empty() {
        return Err(DwError::new("split separator must not be empty"));
    }
    let position = if last {
        text.rfind(&separator)
    } else {
        text.find(&separator)
    };
    match position {
        Some(index) => Ok(Value::Array(vec![
            Value::String(text[..index].to_string()),
            Value::String(text[index + separator.len()..].to_string()),
        ])),
        None => Ok(Value::Null),
    }
}

fn extra_strip_affix(args: &[RValue], prefix: bool) -> Result<Value, DwError> {
    let text = string(args, 0)?;
    let affix = string(args, 1)?;
    if affix.is_empty() {
        return Ok(Value::String(text));
    }
    let stripped = if prefix {
        text.strip_prefix(&affix)
    } else {
        text.strip_suffix(&affix)
    };
    match stripped {
        Some(remainder) => Ok(Value::String(remainder.to_string())),
        None => Ok(Value::Null),
    }
}

/// Builds a failure `TryResult` from an error.
fn try_failure(error: &DwError) -> Value {
    Value::Object(vec![
        ("success".to_string(), Value::Bool(false)),
        (
            "error".to_string(),
            Value::Object(vec![
                ("kind".to_string(), Value::String(error.kind.clone())),
                ("message".to_string(), Value::String(error.message.clone())),
            ]),
        ),
    ])
}

/// Returns the `success` field of a `TryResult`.
fn try_success(value: &Value) -> Result<bool, DwError> {
    match value.get("success") {
        Some(Value::Bool(success)) => Ok(*success),
        _ => Err(DwError::new("expected a TryResult")),
    }
}

/// Raises a user error with an optional message.
fn runtime_fail(args: &[RValue]) -> Result<Value, DwError> {
    let message = match args.first() {
        None => String::new(),
        Some(RValue::Val(Value::Null)) => String::new(),
        Some(RValue::Val(Value::String(text))) => text.clone(),
        Some(RValue::Val(other)) => other.key_string(),
        Some(_) => return Err(DwError::new("expected a String message")),
    };
    Err(DwError::with_kind("UserException", message))
}

/// Returns a value unless a predicate raises a user error.
fn runtime_fail_if(args: &[RValue]) -> Result<Value, DwError> {
    let input = value(args, 0)?.clone();
    let predicate = function(args, 1)?;
    let result = call_fn(predicate, vec![input.clone()])?;
    if result.truthy() {
        Err(DwError::with_kind("UserException", "Failed"))
    } else {
        Ok(input)
    }
}

/// Runs a zero-argument function and captures success or failure.
fn runtime_try(args: &[RValue]) -> Result<Value, DwError> {
    let callable = function(args, 0)?;
    match crate::eval::call_value(callable, &[]) {
        Ok(value) => Ok(Value::Object(vec![
            ("success".to_string(), Value::Bool(true)),
            ("result".to_string(), crate::eval::as_value(value)?),
        ])),
        Err(error) => Ok(try_failure(&error)),
    }
}

/// Returns a successful `TryResult` result or evaluates a fallback.
fn runtime_or_else(args: &[RValue]) -> Result<Value, DwError> {
    let primary = value(args, 0)?.clone();
    let fallback = function(args, 1)?;
    if try_success(&primary)? {
        return Ok(primary.get("result").cloned().unwrap_or(Value::Null));
    }
    call_fn(fallback, Vec::new())
}

/// Returns the primary `TryResult` or wraps the fallback result or error.
fn runtime_or_else_try(args: &[RValue]) -> Result<Value, DwError> {
    let primary = value(args, 0)?.clone();
    let fallback = function(args, 1)?;
    if try_success(&primary)? {
        return Ok(primary);
    }
    match call_fn(fallback, Vec::new()) {
        Ok(value) => Ok(Value::Object(vec![
            ("success".to_string(), Value::Bool(true)),
            ("result".to_string(), value),
        ])),
        Err(error) => Ok(try_failure(&error)),
    }
}

fn extra_unwrap_or(args: &[RValue]) -> Result<Value, DwError> {
    let current = value(args, 0)?;
    let fallback = value(args, 1)?.clone();
    if current.unwrap_formatted().is_null() {
        Ok(fallback)
    } else {
        Ok(current.clone())
    }
}

fn extra_unwrap_or_else(args: &[RValue]) -> Result<Value, DwError> {
    let current = value(args, 0)?;
    let fallback = function(args, 1)?;
    if current.unwrap_formatted().is_null() {
        call_simple(fallback, &[])
    } else {
        Ok(current.clone())
    }
}

fn extra_map_or(args: &[RValue]) -> Result<Value, DwError> {
    let current = value(args, 0)?;
    let fallback = value(args, 1)?.clone();
    let function = function(args, 2)?;
    if current.unwrap_formatted().is_null() {
        Ok(fallback)
    } else {
        call_simple(function, std::slice::from_ref(current))
    }
}

fn extra_and_then(args: &[RValue]) -> Result<Value, DwError> {
    let current = value(args, 0)?;
    let function = function(args, 1)?;
    if current.unwrap_formatted().is_null() {
        Ok(Value::Null)
    } else {
        call_simple(function, std::slice::from_ref(current))
    }
}

fn compose(args: &[RValue]) -> Result<Value, DwError> {
    let parts = array(args, 0)?;
    let values = array(args, 1)?;
    let mut out = String::new();
    for (index, part) in parts.iter().enumerate() {
        out.push_str(&part.key_string());
        if let Some(value) = values.get(index) {
            out.push_str(&encode_uri(&value.key_string(), true));
        }
    }
    for value in values.iter().skip(parts.len()) {
        out.push_str(&encode_uri(&value.key_string(), true));
    }
    Ok(Value::String(out))
}

fn parse_uri(args: &[RValue]) -> Result<Value, DwError> {
    let raw = string(args, 0)?;
    let mut entries = vec![("isValid".to_string(), Value::Bool(true))];
    entries.push(("raw".to_string(), Value::String(raw.clone())));
    let mut rest = raw.as_str();
    let mut scheme = None;
    if let Some(index) = rest.find(':') {
        let candidate = &rest[..index];
        if !candidate.is_empty()
            && candidate
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
        {
            scheme = Some(candidate.to_string());
            rest = &rest[index + 1..];
        }
    }
    let mut fragment = None;
    if let Some(index) = rest.find('#') {
        fragment = Some(rest[index + 1..].to_string());
        rest = &rest[..index];
    }
    let mut query = None;
    if let Some(index) = rest.find('?') {
        query = Some(rest[index + 1..].to_string());
        rest = &rest[..index];
    }
    let mut authority = None;
    let mut user = None;
    let mut host = None;
    let mut port = None;
    let mut path = rest.to_string();
    if let Some(stripped) = rest.strip_prefix("//") {
        let end = stripped.find('/').unwrap_or(stripped.len());
        let auth = &stripped[..end];
        authority = Some(auth.to_string());
        path = stripped[end..].to_string();
        let mut host_port = auth;
        if let Some((before, after)) = auth.rsplit_once('@') {
            user = Some(before.to_string());
            host_port = after;
        }
        if let Some((h, p)) = host_port.rsplit_once(':') {
            if !h.is_empty() && p.chars().all(|c| c.is_ascii_digit()) {
                host = Some(h.to_string());
                port = p.parse::<i64>().ok().map(Number::from_i64);
            } else {
                host = Some(host_port.to_string());
            }
        } else {
            host = Some(host_port.to_string());
        }
    }
    let is_absolute = scheme.is_some();
    let is_opaque = scheme.is_some() && authority.is_none() && !path.is_empty();
    if let Some(host) = host {
        entries.push(("host".to_string(), Value::String(host)));
    }
    if let Some(authority) = authority {
        entries.push(("authority".to_string(), Value::String(authority)));
    }
    if let Some(fragment) = fragment {
        entries.push(("fragment".to_string(), Value::String(fragment)));
    }
    if !path.is_empty() {
        entries.push(("path".to_string(), Value::String(path)));
    }
    if let Some(port) = port {
        entries.push(("port".to_string(), Value::Number(port)));
    }
    if let Some(query) = query {
        entries.push(("query".to_string(), Value::String(query)));
    }
    if let Some(scheme) = scheme {
        entries.push(("scheme".to_string(), Value::String(scheme)));
    }
    if let Some(user) = user {
        entries.push(("user".to_string(), Value::String(user)));
    }
    entries.push(("isAbsolute".to_string(), Value::Bool(is_absolute)));
    entries.push(("isOpaque".to_string(), Value::Bool(is_opaque)));
    Ok(Value::Object(entries))
}

fn diff_repr(value: &Value) -> String {
    match value {
        Value::String(_) => crate::json::write(value),
        Value::Null | Value::Bool(_) | Value::Number(_) => value.key_string(),
        Value::Date(_)
        | Value::DateTime(_)
        | Value::LocalDateTime(_)
        | Value::LocalTime(_)
        | Value::Time(_)
        | Value::TimeZone(_)
        | Value::Period(_) => format!("|{}|", value.key_string()),
        Value::Binary(_) => crate::json::write(value),
        _ => crate::json::write(value),
    }
}

fn diff_add(diffs: &mut Vec<Value>, expected: String, actual: String, path: &str) {
    diffs.push(Value::Object(vec![
        ("expected".to_string(), Value::String(expected)),
        ("actual".to_string(), Value::String(actual)),
        ("path".to_string(), Value::String(path.to_string())),
    ]));
}

fn diff_walk(
    actual: &Value,
    expected: &Value,
    unordered: bool,
    path: &str,
    diffs: &mut Vec<Value>,
) {
    if actual == expected {
        return;
    }
    match (actual, expected) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, expected_value) in b {
                match a.iter().rev().find(|(name, _)| name == key) {
                    Some((_, actual_value)) => {
                        diff_walk(
                            actual_value,
                            expected_value,
                            unordered,
                            &format!("{path}.{key}"),
                            diffs,
                        );
                    }
                    None => diff_add(
                        diffs,
                        format!(
                            "Entry {path}.{key} with type {}",
                            expected_value.type_name()
                        ),
                        "was not present in object.".to_string(),
                        &format!("{path}.{key}"),
                    ),
                }
            }
            if !unordered && a.len() == b.len() {
                let keys_a: Vec<&String> = a.iter().map(|(key, _)| key).collect();
                let keys_b: Vec<&String> = b.iter().map(|(key, _)| key).collect();
                if keys_a != keys_b && diffs.is_empty() {
                    diff_add(
                        diffs,
                        crate::json::write(expected),
                        crate::json::write(actual),
                        path,
                    );
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                diff_add(
                    diffs,
                    format!("Array size is {}", b.len()),
                    format!("was {}", a.len()),
                    path,
                );
            } else {
                for (index, (actual_value, expected_value)) in a.iter().zip(b.iter()).enumerate() {
                    diff_walk(
                        actual_value,
                        expected_value,
                        unordered,
                        &format!("{path}[{index}]"),
                        diffs,
                    );
                }
            }
        }
        _ => diff_add(diffs, diff_repr(expected), diff_repr(actual), path),
    }
}

fn diff_values(args: &[RValue]) -> Result<Value, DwError> {
    let actual = value(args, 0)?;
    let expected = value(args, 1)?;
    let unordered = optional_object(args, 2)
        .and_then(|value| value.get("unordered").cloned())
        .map(|value| value.truthy())
        .unwrap_or(false);
    let path = match args.get(3) {
        Some(RValue::Val(Value::String(text))) => text.clone(),
        _ => "(root)".to_string(),
    };
    let mut diffs = Vec::new();
    diff_walk(actual, expected, unordered, &path, &mut diffs);
    Ok(Value::Object(vec![
        ("matches".to_string(), Value::Bool(diffs.is_empty())),
        ("diffs".to_string(), Value::Array(diffs)),
    ]))
}

fn level_log(args: &[RValue]) -> Result<Value, DwError> {
    let (prefix, value) = match args.len() {
        0 => return Ok(Value::Null),
        1 => (None, value(args, 0)?.clone()),
        _ => (Some(value(args, 0)?.key_string()), value(args, 1)?.clone()),
    };
    match prefix {
        Some(prefix) => eprintln!("{prefix}: {}", crate::json::write(&value)),
        None => eprintln!("{}", crate::json::write(&value)),
    }
    Ok(value)
}

fn log_with(args: &[RValue]) -> Result<Value, DwError> {
    let logged = value(args, 2)?.clone();
    let level = value(args, 0)?.key_string();
    let prefix = value(args, 1)?.key_string();
    eprintln!("[{level}] {prefix}: {}", crate::json::write(&logged));
    Ok(logged)
}

fn timer_to_milliseconds(args: &[RValue]) -> Result<Value, DwError> {
    match value(args, 0)?.unwrap_formatted() {
        Value::DateTime(value) => {
            let seconds = crate::temporal::timestamp_seconds(value)?;
            let millis = (value.time.nano / 1_000_000) as i64;
            Ok(Value::Number(Number::from_i64(seconds * 1000 + millis)))
        }
        other => Err(DwError::new(format!(
            "toMilliseconds expects a DateTime, found {}",
            other.type_name()
        ))),
    }
}

fn timer_callable(args: &[RValue]) -> Result<&RValue, DwError> {
    match args.first() {
        Some(
            callable @ (RValue::Func(_)
            | RValue::Builtin(_)
            | RValue::Overloads(_)
            | RValue::Sequence(_, _)),
        ) => Ok(callable),
        Some(other) => {
            let type_name = match other {
                RValue::Val(value) => value.type_name(),
                RValue::Regex(_) => "Regex",
                _ => "value",
            };
            Err(DwError::new(format!(
                "expected a zero-argument function, found {type_name}"
            )))
        }
        None => Err(DwError::new("missing function argument")),
    }
}

fn elapsed_milliseconds(start: SystemTime) -> i64 {
    start
        .elapsed()
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

fn timer_duration(args: &[RValue]) -> Result<Value, DwError> {
    let callable = timer_callable(args)?;
    let start = SystemTime::now();
    let result = crate::eval::as_value(crate::eval::call_value(callable, &[])?)?;
    let elapsed = elapsed_milliseconds(start);
    Ok(Value::Object(vec![
        ("time".to_string(), Value::Number(Number::from_i64(elapsed))),
        ("result".to_string(), result),
    ]))
}

fn timer_wall_clock(args: &[RValue]) -> Result<Value, DwError> {
    let callable = timer_callable(args)?;
    let start = crate::temporal::now();
    let result = crate::eval::as_value(crate::eval::call_value(callable, &[])?)?;
    let end = crate::temporal::now();
    Ok(Value::Object(vec![
        ("start".to_string(), Value::DateTime(start)),
        ("result".to_string(), result),
        ("end".to_string(), Value::DateTime(end)),
    ]))
}

fn type_arg(args: &[RValue], index: usize) -> Result<&crate::types::TypeValue, DwError> {
    match value(args, index)? {
        Value::Type(value) => Ok(value),
        other => Err(DwError::new(format!(
            "expected a Type, found {}",
            other.type_name()
        ))),
    }
}

fn type_array_item(args: &[RValue]) -> Result<Value, DwError> {
    match type_arg(args, 0)?.structure() {
        crate::types::TypeValue::Array(item) => Ok(Value::Type(item.clone())),
        _ => Err(DwError::new("expected an Array type")),
    }
}

fn type_base(args: &[RValue]) -> Result<Value, DwError> {
    Ok(Value::Type(Box::new(type_arg(args, 0)?.base())))
}

fn type_function_params(args: &[RValue]) -> Result<Value, DwError> {
    // FunctionParam exposes only the declared parameter type and optionality.
    match type_arg(args, 0)?.structure() {
        crate::types::TypeValue::Function(params, _) => Ok(Value::Array(
            params
                .iter()
                .map(|param| {
                    Value::Object(vec![
                        (
                            "paramType".to_string(),
                            Value::Type(Box::new(param.value.clone())),
                        ),
                        ("optional".to_string(), Value::Bool(param.optional)),
                    ])
                })
                .collect(),
        )),
        _ => Err(DwError::new("expected a Function type")),
    }
}

fn type_function_return(args: &[RValue]) -> Result<Value, DwError> {
    match type_arg(args, 0)?.structure() {
        crate::types::TypeValue::Function(_, result) => Ok(Value::Type(result.clone())),
        _ => Err(DwError::new("expected a Function type")),
    }
}

fn type_items(args: &[RValue], intersection: bool) -> Result<Value, DwError> {
    let value = type_arg(args, 0)?.structure();
    let items = match (intersection, value) {
        (true, crate::types::TypeValue::Intersection(items)) => items,
        (false, crate::types::TypeValue::Union(items)) => items,
        _ => {
            return Err(DwError::new(if intersection {
                "expected an Intersection type"
            } else {
                "expected a Union type"
            }))
        }
    };
    Ok(Value::Array(
        items
            .iter()
            .map(|item| Value::Type(Box::new(item.clone())))
            .collect(),
    ))
}

fn type_predicate<F>(args: &[RValue], predicate: F) -> Result<Value, DwError>
where
    F: FnOnce(&crate::types::TypeValue) -> bool,
{
    Ok(Value::Bool(predicate(type_arg(args, 0)?)))
}

fn type_reference(args: &[RValue]) -> Result<Value, DwError> {
    fn is_reference(value: &crate::types::TypeValue) -> bool {
        match value {
            crate::types::TypeValue::Declared { .. } => false,
            crate::types::TypeValue::Ref(_) => true,
            crate::types::TypeValue::Meta(inner, _) => is_reference(inner),
            _ => false,
        }
    }
    Ok(Value::Bool(is_reference(type_arg(args, 0)?)))
}

fn type_literal_value(args: &[RValue]) -> Result<Value, DwError> {
    match type_arg(args, 0)?.structure() {
        crate::types::TypeValue::Literal(value) => Ok(value.clone()),
        _ => Err(DwError::new("expected a Literal type")),
    }
}

fn type_metadata(args: &[RValue]) -> Result<Value, DwError> {
    Ok(Value::Object(type_arg(args, 0)?.metadata()))
}

fn type_name(args: &[RValue]) -> Result<Value, DwError> {
    Ok(Value::String(type_arg(args, 0)?.display()))
}

fn type_field_key(field: &crate::types::TypeField) -> Value {
    let namespace = field
        .namespace
        .clone()
        .map(Value::String)
        .unwrap_or(Value::Null);
    let attributes = field
        .attributes
        .iter()
        .map(|attribute| {
            let namespace = attribute
                .namespace
                .clone()
                .map(Value::String)
                .unwrap_or(Value::Null);
            Value::Object(vec![
                (
                    "name".to_string(),
                    Value::Object(vec![
                        (
                            "localName".to_string(),
                            Value::String(attribute.name.clone()),
                        ),
                        ("namespace".to_string(), namespace),
                    ]),
                ),
                (
                    "value".to_string(),
                    Value::String(attribute.value.display()),
                ),
                ("required".to_string(), Value::Bool(attribute.required)),
            ])
        })
        .collect();
    Value::Object(vec![
        (
            "name".to_string(),
            Value::Object(vec![
                ("localName".to_string(), Value::String(field.name.clone())),
                ("namespace".to_string(), namespace),
            ]),
        ),
        ("attributes".to_string(), Value::Array(attributes)),
    ])
}

fn type_object_fields(args: &[RValue]) -> Result<Value, DwError> {
    match type_arg(args, 0)?.structure() {
        crate::types::TypeValue::Object(fields) => Ok(Value::Array(
            fields
                .iter()
                .map(|field| {
                    Value::Object(vec![
                        ("key".to_string(), type_field_key(field)),
                        ("required".to_string(), Value::Bool(field.required)),
                        ("repeated".to_string(), Value::Bool(field.repeated)),
                        ("value".to_string(), Value::String(field.value.display())),
                    ])
                })
                .collect(),
        )),
        _ => Err(DwError::new("expected an Object type")),
    }
}
