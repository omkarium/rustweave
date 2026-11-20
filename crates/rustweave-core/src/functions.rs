// SPDX-License-Identifier: MIT OR Apache-2.0
//! Catalog of supported DataWeave functions.
//!
//! The catalog is documentation-only. Use it to discover the supported names and their
//! namespaces without loading the evaluator.
//!
//! ```
//! use rustweave_core::functions::Function;
//!
//! assert_eq!(Function::CORE_MAP.name, "map");
//! assert_eq!(Function::CORE_MAP.namespace, "dw::Core");
//! ```

/// One supported DataWeave function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Function {
    /// The exact DataWeave function name.
    pub name: &'static str,
    /// The module or namespace that exposes the function.
    pub namespace: &'static str,
    /// A short description of the function.
    pub description: &'static str,
}

macro_rules! catalog_function {
    ($constant:ident, $namespace:literal, $name:literal, $description:literal) => {
        #[doc = $description]
        #[doc(alias = $name)]
        pub const $constant: Function = Function {
            name: $name,
            namespace: $namespace,
            description: $description,
        };
    };
}

impl Function {
    catalog_function!(
        CORE_MAP,
        "dw::Core",
        "map",
        "Applies a transformation to every array element."
    );
    catalog_function!(
        CORE_FILTER,
        "dw::Core",
        "filter",
        "Keeps the array elements that satisfy a predicate."
    );
    catalog_function!(
        CORE_REDUCE,
        "dw::Core",
        "reduce",
        "Folds an array into a single value."
    );
    catalog_function!(
        CORE_GROUP_BY,
        "dw::Core",
        "groupBy",
        "Groups array elements by a selector."
    );
    catalog_function!(
        CORE_SIZE_OF,
        "dw::Core",
        "sizeOf",
        "Returns the size of a value."
    );
    catalog_function!(
        CORE_IS_EMPTY,
        "dw::Core",
        "isEmpty",
        "Reports whether a value is empty."
    );
    catalog_function!(CORE_SUM, "dw::Core", "sum", "Adds the numbers in an array.");
    catalog_function!(
        CORE_MAP_OBJECT,
        "dw::Core",
        "mapObject",
        "Transforms the values of an object."
    );
    catalog_function!(
        CORE_PLUCK,
        "dw::Core",
        "pluck",
        "Collects values selected from objects in an array."
    );
    catalog_function!(
        CORE_FILTER_OBJECT,
        "dw::Core",
        "filterObject",
        "Keeps object entries that satisfy a predicate."
    );
    catalog_function!(
        CORE_FLAT_MAP,
        "dw::Core",
        "flatMap",
        "Maps each element and flattens one level."
    );
    catalog_function!(
        CORE_ENTRIES_OF,
        "dw::Core",
        "entriesOf",
        "Returns object entries as key/value objects."
    );
    catalog_function!(CORE_NAMES_OF, "dw::Core", "namesOf", "Returns object keys.");
    catalog_function!(CORE_KEYS_OF, "dw::Core", "keysOf", "Returns object keys.");
    catalog_function!(
        CORE_VALUES_OF,
        "dw::Core",
        "valuesOf",
        "Returns object values."
    );
    catalog_function!(
        CORE_FLATTEN,
        "dw::Core",
        "flatten",
        "Flattens nested arrays into one array."
    );
    catalog_function!(
        CORE_DISTINCT_BY,
        "dw::Core",
        "distinctBy",
        "Removes duplicate elements selected by a function."
    );
    catalog_function!(
        CORE_ORDER_BY,
        "dw::Core",
        "orderBy",
        "Sorts array elements by a selector."
    );
    catalog_function!(
        CORE_FIND,
        "dw::Core",
        "find",
        "Returns the first element that matches a predicate."
    );
    catalog_function!(
        CORE_INDEX_OF,
        "dw::Core",
        "indexOf",
        "Returns the index of the first matching value."
    );
    catalog_function!(
        CORE_LAST_INDEX_OF,
        "dw::Core",
        "lastIndexOf",
        "Returns the index of the last matching value."
    );
    catalog_function!(
        CORE_CONTAINS,
        "dw::Core",
        "contains",
        "Reports whether a value contains another value."
    );
    catalog_function!(
        CORE_TO,
        "dw::Core",
        "to",
        "Creates an inclusive numeric range."
    );
    catalog_function!(
        CORE_ZIP,
        "dw::Core",
        "zip",
        "Combines parallel arrays into pairs."
    );
    catalog_function!(
        CORE_UNZIP,
        "dw::Core",
        "unzip",
        "Splits an array of pairs into parallel arrays."
    );
    catalog_function!(
        CORE_UPPER,
        "dw::Core",
        "upper",
        "Converts text to uppercase."
    );
    catalog_function!(
        CORE_LOWER,
        "dw::Core",
        "lower",
        "Converts text to lowercase."
    );
    catalog_function!(
        CORE_TRIM,
        "dw::Core",
        "trim",
        "Removes leading and trailing whitespace."
    );
    catalog_function!(
        CORE_STARTS_WITH,
        "dw::Core",
        "startsWith",
        "Reports whether text starts with a prefix."
    );
    catalog_function!(
        CORE_ENDS_WITH,
        "dw::Core",
        "endsWith",
        "Reports whether text ends with a suffix."
    );
    catalog_function!(
        CORE_SPLIT_BY,
        "dw::Core",
        "splitBy",
        "Splits text using a separator or pattern."
    );
    catalog_function!(
        CORE_WITH,
        "dw::Core",
        "with",
        "Supplies the replacement argument for replace or update expressions."
    );
    catalog_function!(
        CORE_REPLACE,
        "dw::Core",
        "replace",
        "Replaces matches with a computed value."
    );
    catalog_function!(
        CORE_JOIN_BY,
        "dw::Core",
        "joinBy",
        "Joins array elements into text."
    );
    catalog_function!(
        CORE_ABS,
        "dw::Core",
        "abs",
        "Returns the absolute value of a number."
    );
    catalog_function!(
        CORE_CEIL,
        "dw::Core",
        "ceil",
        "Rounds a number up to an integer."
    );
    catalog_function!(
        CORE_FLOOR,
        "dw::Core",
        "floor",
        "Rounds a number down to an integer."
    );
    catalog_function!(
        CORE_ROUND,
        "dw::Core",
        "round",
        "Rounds a number to the nearest integer."
    );
    catalog_function!(CORE_POW, "dw::Core", "pow", "Raises a number to a power.");
    catalog_function!(
        CORE_SQRT,
        "dw::Core",
        "sqrt",
        "Returns the square root of a number."
    );
    catalog_function!(
        CORE_MOD,
        "dw::Core",
        "mod",
        "Returns the remainder of division."
    );
    catalog_function!(
        CORE_IS_INTEGER,
        "dw::Core",
        "isInteger",
        "Reports whether a number is an integer."
    );
    catalog_function!(
        CORE_IS_DECIMAL,
        "dw::Core",
        "isDecimal",
        "Reports whether a number has a fractional part."
    );
    catalog_function!(
        CORE_IS_ODD,
        "dw::Core",
        "isOdd",
        "Reports whether an integer is odd."
    );
    catalog_function!(
        CORE_IS_EVEN,
        "dw::Core",
        "isEven",
        "Reports whether an integer is even."
    );
    catalog_function!(
        CORE_IS_BLANK,
        "dw::Core",
        "isBlank",
        "Reports whether a value is blank."
    );
    catalog_function!(
        CORE_MIN_BY,
        "dw::Core",
        "minBy",
        "Returns the element with the smallest selected value."
    );
    catalog_function!(
        CORE_MAX_BY,
        "dw::Core",
        "maxBy",
        "Returns the element with the largest selected value."
    );
    catalog_function!(CORE_MAX, "dw::Core", "max", "Returns the largest value.");
    catalog_function!(CORE_MIN, "dw::Core", "min", "Returns the smallest value.");
    catalog_function!(CORE_AVG, "dw::Core", "avg", "Returns the arithmetic mean.");
    catalog_function!(
        CORE_THEN,
        "dw::Core",
        "then",
        "Applies a function and returns its result."
    );
    catalog_function!(
        CORE_ON_NULL,
        "dw::Core",
        "onNull",
        "Returns a fallback when the input is null."
    );
    catalog_function!(
        CORE_MATCHES,
        "dw::Core",
        "matches",
        "Tests text against a regular expression."
    );
    catalog_function!(
        CORE_MATCH,
        "dw::Core",
        "match",
        "Returns the parts matched by a regular expression."
    );
    catalog_function!(
        CORE_SCAN,
        "dw::Core",
        "scan",
        "Returns successive regular-expression matches."
    );
    catalog_function!(
        CORE_TYPE_OF,
        "dw::Core",
        "typeOf",
        "Returns the runtime type name of a value."
    );
    catalog_function!(
        CORE_RANDOM,
        "dw::Core",
        "random",
        "Returns a random number."
    );
    catalog_function!(
        CORE_RANDOM_INT,
        "dw::Core",
        "randomInt",
        "Returns a random integer."
    );
    catalog_function!(
        CORE_UUID,
        "dw::Core",
        "uuid",
        "Returns a random UUID string."
    );
    catalog_function!(
        CORE_LOG,
        "dw::Core",
        "log",
        "Writes a value to the log and returns it."
    );
    catalog_function!(
        CORE_DAYS_BETWEEN,
        "dw::Core",
        "daysBetween",
        "Returns the number of days between two dates."
    );
    catalog_function!(
        CORE_IS_LEAP_YEAR,
        "dw::Core",
        "isLeapYear",
        "Reports whether a date falls in a leap year."
    );
    catalog_function!(
        CORE_NOW,
        "dw::Core",
        "now",
        "Returns the current date and time."
    );
    catalog_function!(
        CORE_READ,
        "dw::Core",
        "read",
        "Parses text or binary content using a content type."
    );
    catalog_function!(
        CORE_WRITE,
        "dw::Core",
        "write",
        "Serializes a value using a content type."
    );
    catalog_function!(
        CORE_READ_URL,
        "dw::Core",
        "readUrl",
        "Reads and parses content from a URL."
    );
    catalog_function!(
        CORE_LOG_DEBUG,
        "dw::Core",
        "logDebug",
        "Logs a value at debug level and returns it."
    );
    catalog_function!(
        CORE_LOG_INFO,
        "dw::Core",
        "logInfo",
        "Logs a value at info level and returns it."
    );
    catalog_function!(
        CORE_LOG_WARN,
        "dw::Core",
        "logWarn",
        "Logs a value at warn level and returns it."
    );
    catalog_function!(
        CORE_LOG_ERROR,
        "dw::Core",
        "logError",
        "Logs a value at error level and returns it."
    );
    catalog_function!(
        CORE_LOG_WITH,
        "dw::Core",
        "logWith",
        "Logs a value at a selected level and returns it."
    );
    catalog_function!(
        CORE_PLUS_PLUS,
        "dw::Core",
        "plusPlus",
        "Named form of the concatenation operator."
    );
    catalog_function!(
        CORE_MINUS_MINUS,
        "dw::Core",
        "minusMinus",
        "Named form of the difference operator."
    );
    catalog_function!(
        ARRAYS_COUNT_BY,
        "dw::core::Arrays",
        "countBy",
        "Counts elements that satisfy a predicate."
    );
    catalog_function!(
        ARRAYS_SUM_BY,
        "dw::core::Arrays",
        "sumBy",
        "Sums values selected from array elements."
    );
    catalog_function!(
        ARRAYS_SOME,
        "dw::core::Arrays",
        "some",
        "Reports whether any element satisfies a predicate."
    );
    catalog_function!(
        ARRAYS_EVERY,
        "dw::core::Arrays",
        "every",
        "Reports whether every element satisfies a predicate."
    );
    catalog_function!(
        ARRAYS_TAKE,
        "dw::core::Arrays",
        "take",
        "Returns the first elements of an array."
    );
    catalog_function!(
        ARRAYS_DROP,
        "dw::core::Arrays",
        "drop",
        "Returns an array without its first elements."
    );
    catalog_function!(
        ARRAYS_SLICE,
        "dw::core::Arrays",
        "slice",
        "Returns a section of an array or string."
    );
    catalog_function!(
        ARRAYS_INDEX_OF,
        "dw::core::Arrays",
        "indexOf",
        "Returns the index of the first matching array value."
    );
    catalog_function!(
        ARRAYS_INDEX_WHERE,
        "dw::core::Arrays",
        "indexWhere",
        "Returns the index of the first matching element."
    );
    catalog_function!(
        ARRAYS_SPLIT_AT,
        "dw::core::Arrays",
        "splitAt",
        "Splits an array at an index."
    );
    catalog_function!(
        ARRAYS_TAKE_WHILE,
        "dw::core::Arrays",
        "takeWhile",
        "Returns array elements while a predicate holds."
    );
    catalog_function!(
        ARRAYS_DROP_WHILE,
        "dw::core::Arrays",
        "dropWhile",
        "Drops elements while a predicate holds."
    );
    catalog_function!(
        ARRAYS_PARTITION,
        "dw::core::Arrays",
        "partition",
        "Splits elements into success and failure groups."
    );
    catalog_function!(
        ARRAYS_FIRST_WITH,
        "dw::core::Arrays",
        "firstWith",
        "Returns the first element matching a predicate."
    );
    catalog_function!(
        ARRAYS_DIVIDE_BY,
        "dw::core::Arrays",
        "divideBy",
        "Splits an array into groups of a fixed size."
    );
    catalog_function!(
        ARRAYS_SPLIT_WHERE,
        "dw::core::Arrays",
        "splitWhere",
        "Splits an array where a predicate changes."
    );
    catalog_function!(
        ARRAYS_JOIN,
        "dw::core::Arrays",
        "join",
        "Joins two arrays by matching keys."
    );
    catalog_function!(
        ARRAYS_LEFT_JOIN,
        "dw::core::Arrays",
        "leftJoin",
        "Joins two arrays, keeping unmatched left elements."
    );
    catalog_function!(
        ARRAYS_OUTER_JOIN,
        "dw::core::Arrays",
        "outerJoin",
        "Joins two arrays, keeping unmatched elements from both sides."
    );
    catalog_function!(
        STRINGS_APPEND_IF_MISSING,
        "dw::core::Strings",
        "appendIfMissing",
        "Appends a suffix when it is absent."
    );
    catalog_function!(
        STRINGS_CAMELIZE,
        "dw::core::Strings",
        "camelize",
        "Converts text to camelCase."
    );
    catalog_function!(
        STRINGS_CAPITALIZE,
        "dw::core::Strings",
        "capitalize",
        "Capitalizes the first character of each word."
    );
    catalog_function!(
        STRINGS_CHAR_CODE,
        "dw::core::Strings",
        "charCode",
        "Returns the character code of the first character."
    );
    catalog_function!(
        STRINGS_CHAR_CODE_AT,
        "dw::core::Strings",
        "charCodeAt",
        "Returns the character code at an index."
    );
    catalog_function!(
        STRINGS_COLLAPSE,
        "dw::core::Strings",
        "collapse",
        "Collapses repeated adjacent characters."
    );
    catalog_function!(
        STRINGS_COUNT_CHARACTERS_BY,
        "dw::core::Strings",
        "countCharactersBy",
        "Counts characters that satisfy a predicate."
    );
    catalog_function!(
        STRINGS_COUNT_MATCHES,
        "dw::core::Strings",
        "countMatches",
        "Counts regular-expression matches."
    );
    catalog_function!(
        STRINGS_DASHERIZE,
        "dw::core::Strings",
        "dasherize",
        "Converts text to kebab-case."
    );
    catalog_function!(
        STRINGS_EVERY_CHARACTER,
        "dw::core::Strings",
        "everyCharacter",
        "Reports whether every character matches a predicate."
    );
    catalog_function!(
        STRINGS_FIRST,
        "dw::core::Strings",
        "first",
        "Returns the first character or element."
    );
    catalog_function!(
        STRINGS_FROM_CHAR_CODE,
        "dw::core::Strings",
        "fromCharCode",
        "Creates text from a character code."
    );
    catalog_function!(
        STRINGS_HAMMING_DISTANCE,
        "dw::core::Strings",
        "hammingDistance",
        "Returns the Hamming distance between two strings."
    );
    catalog_function!(
        STRINGS_IS_ALPHA,
        "dw::core::Strings",
        "isAlpha",
        "Reports whether text contains only letters."
    );
    catalog_function!(
        STRINGS_IS_ALPHANUMERIC,
        "dw::core::Strings",
        "isAlphanumeric",
        "Reports whether text contains only letters and digits."
    );
    catalog_function!(
        STRINGS_IS_LOWER_CASE,
        "dw::core::Strings",
        "isLowerCase",
        "Reports whether text is lowercase."
    );
    catalog_function!(
        STRINGS_IS_NUMERIC,
        "dw::core::Strings",
        "isNumeric",
        "Reports whether text contains only digits."
    );
    catalog_function!(
        STRINGS_IS_UPPER_CASE,
        "dw::core::Strings",
        "isUpperCase",
        "Reports whether text is uppercase."
    );
    catalog_function!(
        STRINGS_IS_WHITESPACE,
        "dw::core::Strings",
        "isWhitespace",
        "Reports whether text contains only whitespace."
    );
    catalog_function!(
        STRINGS_LAST,
        "dw::core::Strings",
        "last",
        "Returns the last character or element."
    );
    catalog_function!(
        STRINGS_LEFT_PAD,
        "dw::core::Strings",
        "leftPad",
        "Pads text on the left."
    );
    catalog_function!(
        STRINGS_LEVENSHTEIN_DISTANCE,
        "dw::core::Strings",
        "levenshteinDistance",
        "Returns the edit distance between two strings."
    );
    catalog_function!(
        STRINGS_LINES,
        "dw::core::Strings",
        "lines",
        "Splits text into lines."
    );
    catalog_function!(
        STRINGS_MAP_STRING,
        "dw::core::Strings",
        "mapString",
        "Transforms each character in text."
    );
    catalog_function!(
        STRINGS_ORDINALIZE,
        "dw::core::Strings",
        "ordinalize",
        "Formats a number as an ordinal."
    );
    catalog_function!(
        STRINGS_PLURALIZE,
        "dw::core::Strings",
        "pluralize",
        "Converts a word to plural form."
    );
    catalog_function!(
        STRINGS_PREPEND_IF_MISSING,
        "dw::core::Strings",
        "prependIfMissing",
        "Prepends a prefix when it is absent."
    );
    catalog_function!(
        STRINGS_REMOVE,
        "dw::core::Strings",
        "remove",
        "Removes every occurrence of text."
    );
    catalog_function!(
        STRINGS_REPEAT,
        "dw::core::Strings",
        "repeat",
        "Repeats text a number of times."
    );
    catalog_function!(
        STRINGS_REPLACE_ALL,
        "dw::core::Strings",
        "replaceAll",
        "Replaces all literal matches in text."
    );
    catalog_function!(
        STRINGS_REVERSE,
        "dw::core::Strings",
        "reverse",
        "Reverses text."
    );
    catalog_function!(
        STRINGS_RIGHT_PAD,
        "dw::core::Strings",
        "rightPad",
        "Pads text on the right."
    );
    catalog_function!(
        STRINGS_SINGULARIZE,
        "dw::core::Strings",
        "singularize",
        "Converts a word to singular form."
    );
    catalog_function!(
        STRINGS_SOME_CHARACTER,
        "dw::core::Strings",
        "someCharacter",
        "Reports whether any character matches a predicate."
    );
    catalog_function!(
        STRINGS_SUBSTRING,
        "dw::core::Strings",
        "substring",
        "Returns a substring by range."
    );
    catalog_function!(
        STRINGS_SUBSTRING_AFTER,
        "dw::core::Strings",
        "substringAfter",
        "Returns text after a separator."
    );
    catalog_function!(
        STRINGS_SUBSTRING_AFTER_LAST,
        "dw::core::Strings",
        "substringAfterLast",
        "Returns text after the last separator."
    );
    catalog_function!(
        STRINGS_SUBSTRING_BEFORE,
        "dw::core::Strings",
        "substringBefore",
        "Returns text before a separator."
    );
    catalog_function!(
        STRINGS_SUBSTRING_BEFORE_LAST,
        "dw::core::Strings",
        "substringBeforeLast",
        "Returns text before the last separator."
    );
    catalog_function!(
        STRINGS_SUBSTRING_BY,
        "dw::core::Strings",
        "substringBy",
        "Selects text by a character predicate."
    );
    catalog_function!(
        STRINGS_SUBSTRING_EVERY,
        "dw::core::Strings",
        "substringEvery",
        "Returns every nth character."
    );
    catalog_function!(
        STRINGS_UNDERSCORE,
        "dw::core::Strings",
        "underscore",
        "Converts text to snake_case."
    );
    catalog_function!(
        STRINGS_UNWRAP,
        "dw::core::Strings",
        "unwrap",
        "Removes a surrounding wrapper."
    );
    catalog_function!(
        STRINGS_WITH_MAX_SIZE,
        "dw::core::Strings",
        "withMaxSize",
        "Limits text to a maximum size."
    );
    catalog_function!(
        STRINGS_WORDS,
        "dw::core::Strings",
        "words",
        "Splits text into words."
    );
    catalog_function!(
        STRINGS_WRAP_IF_MISSING,
        "dw::core::Strings",
        "wrapIfMissing",
        "Adds a wrapper when it is absent."
    );
    catalog_function!(
        STRINGS_WRAP_WITH,
        "dw::core::Strings",
        "wrapWith",
        "Wraps text with a prefix and suffix."
    );
    catalog_function!(
        OBJECTS_ENTRY_SET,
        "dw::core::Objects",
        "entrySet",
        "Returns object entries as an array."
    );
    catalog_function!(
        OBJECTS_NAME_SET,
        "dw::core::Objects",
        "nameSet",
        "Returns object keys as an array."
    );
    catalog_function!(
        OBJECTS_KEY_SET,
        "dw::core::Objects",
        "keySet",
        "Returns object keys as an array."
    );
    catalog_function!(
        OBJECTS_VALUE_SET,
        "dw::core::Objects",
        "valueSet",
        "Returns object values as an array."
    );
    catalog_function!(
        OBJECTS_EVERY_ENTRY,
        "dw::core::Objects",
        "everyEntry",
        "Reports whether every object entry satisfies a predicate."
    );
    catalog_function!(
        OBJECTS_SOME_ENTRY,
        "dw::core::Objects",
        "someEntry",
        "Reports whether any object entry satisfies a predicate."
    );
    catalog_function!(
        OBJECTS_MERGE_WITH,
        "dw::core::Objects",
        "mergeWith",
        "Merges objects using a value combiner."
    );
    catalog_function!(
        OBJECTS_DIVIDE_BY,
        "dw::core::Objects",
        "divideBy",
        "Splits object entries into groups of a fixed size."
    );
    catalog_function!(
        OBJECTS_TAKE_WHILE,
        "dw::core::Objects",
        "takeWhile",
        "Keeps object entries while a predicate holds."
    );
    catalog_function!(
        NUMBERS_TO_HEX,
        "dw::core::Numbers",
        "toHex",
        "Converts a number to a hexadecimal string."
    );
    catalog_function!(
        NUMBERS_TO_BINARY,
        "dw::core::Numbers",
        "toBinary",
        "Converts a number to a binary string."
    );
    catalog_function!(
        NUMBERS_FROM_HEX,
        "dw::core::Numbers",
        "fromHex",
        "Parses a hexadecimal string as a number."
    );
    catalog_function!(
        NUMBERS_FROM_BINARY,
        "dw::core::Numbers",
        "fromBinary",
        "Parses a binary string as a number."
    );
    catalog_function!(
        NUMBERS_FROM_RADIX_NUMBER,
        "dw::core::Numbers",
        "fromRadixNumber",
        "Parses text in a given radix."
    );
    catalog_function!(
        NUMBERS_TO_RADIX_NUMBER,
        "dw::core::Numbers",
        "toRadixNumber",
        "Formats a number in a given radix."
    );
    catalog_function!(
        DATES_AT_BEGINNING_OF_DAY,
        "dw::core::Dates",
        "atBeginningOfDay",
        "Returns the start of the day for a date-time."
    );
    catalog_function!(
        DATES_AT_BEGINNING_OF_HOUR,
        "dw::core::Dates",
        "atBeginningOfHour",
        "Returns the start of the hour for a time value."
    );
    catalog_function!(
        DATES_AT_BEGINNING_OF_MONTH,
        "dw::core::Dates",
        "atBeginningOfMonth",
        "Returns the first day of the month."
    );
    catalog_function!(
        DATES_AT_BEGINNING_OF_WEEK,
        "dw::core::Dates",
        "atBeginningOfWeek",
        "Returns the first day of the week."
    );
    catalog_function!(
        DATES_AT_BEGINNING_OF_YEAR,
        "dw::core::Dates",
        "atBeginningOfYear",
        "Returns the first day of the year."
    );
    catalog_function!(
        DATES_DATE,
        "dw::core::Dates",
        "date",
        "Creates a Date from year, month, and day."
    );
    catalog_function!(
        DATES_DATE_TIME,
        "dw::core::Dates",
        "dateTime",
        "Creates a DateTime from date, time, and zone fields."
    );
    catalog_function!(
        DATES_LOCAL_DATE_TIME,
        "dw::core::Dates",
        "localDateTime",
        "Creates a LocalDateTime from date and time fields."
    );
    catalog_function!(
        DATES_LOCAL_TIME,
        "dw::core::Dates",
        "localTime",
        "Creates a LocalTime from hour, minute, and second fields."
    );
    catalog_function!(
        DATES_TIME,
        "dw::core::Dates",
        "time",
        "Creates a Time from time and zone fields."
    );
    catalog_function!(
        DATES_TODAY,
        "dw::core::Dates",
        "today",
        "Returns today as a Date."
    );
    catalog_function!(
        DATES_TOMORROW,
        "dw::core::Dates",
        "tomorrow",
        "Returns tomorrow as a Date."
    );
    catalog_function!(
        DATES_YESTERDAY,
        "dw::core::Dates",
        "yesterday",
        "Returns yesterday as a Date."
    );
    catalog_function!(
        PERIODS_BETWEEN,
        "dw::core::Periods",
        "between",
        "Returns the period between two dates."
    );
    catalog_function!(
        PERIODS_DAYS,
        "dw::core::Periods",
        "days",
        "Creates a Period from a number of days."
    );
    catalog_function!(
        PERIODS_DURATION,
        "dw::core::Periods",
        "duration",
        "Creates a duration Period from day and time fields."
    );
    catalog_function!(
        PERIODS_HOURS,
        "dw::core::Periods",
        "hours",
        "Creates a Period from a number of hours."
    );
    catalog_function!(
        PERIODS_MINUTES,
        "dw::core::Periods",
        "minutes",
        "Creates a Period from a number of minutes."
    );
    catalog_function!(
        PERIODS_MONTHS,
        "dw::core::Periods",
        "months",
        "Creates a Period from a number of months."
    );
    catalog_function!(
        PERIODS_PERIOD,
        "dw::core::Periods",
        "period",
        "Creates a date-based Period from year, month, and day fields."
    );
    catalog_function!(
        PERIODS_SECONDS,
        "dw::core::Periods",
        "seconds",
        "Creates a Period from a number of seconds."
    );
    catalog_function!(
        PERIODS_YEARS,
        "dw::core::Periods",
        "years",
        "Creates a Period from a number of years."
    );
    catalog_function!(
        BINARIES_CONCAT_WITH,
        "dw::core::Binaries",
        "concatWith",
        "Concatenates two binary values."
    );
    catalog_function!(
        BINARIES_FROM_BASE_64,
        "dw::core::Binaries",
        "fromBase64",
        "Decodes a Base64 string to Binary."
    );
    catalog_function!(
        BINARIES_FROM_HEX,
        "dw::core::Binaries",
        "fromHex",
        "Decodes a hexadecimal string to Binary."
    );
    catalog_function!(
        BINARIES_READ_LINES_WITH,
        "dw::core::Binaries",
        "readLinesWith",
        "Splits binary content into lines."
    );
    catalog_function!(
        BINARIES_TO_BASE_64,
        "dw::core::Binaries",
        "toBase64",
        "Encodes Binary as a Base64 string."
    );
    catalog_function!(
        BINARIES_TO_HEX,
        "dw::core::Binaries",
        "toHex",
        "Encodes Binary as a hexadecimal string."
    );
    catalog_function!(
        BINARIES_WRITE_LINES_WITH,
        "dw::core::Binaries",
        "writeLinesWith",
        "Encodes lines as binary content."
    );
    catalog_function!(
        URL_COMPOSE,
        "dw::core::URL",
        "compose",
        "Builds a URL from base parts and encoded values."
    );
    catalog_function!(
        URL_DECODE_URI,
        "dw::core::URL",
        "decodeURI",
        "Decodes a URI while preserving reserved characters."
    );
    catalog_function!(
        URL_DECODE_URI_COMPONENT,
        "dw::core::URL",
        "decodeURIComponent",
        "Decodes a URI component."
    );
    catalog_function!(
        URL_ENCODE_URI,
        "dw::core::URL",
        "encodeURI",
        "Encodes a URI while preserving reserved characters."
    );
    catalog_function!(
        URL_ENCODE_URI_COMPONENT,
        "dw::core::URL",
        "encodeURIComponent",
        "Encodes a URI component."
    );
    catalog_function!(
        URL_PARSE_URI,
        "dw::core::URL",
        "parseURI",
        "Parses a URI into its component fields."
    );
    catalog_function!(
        TYPES_ARRAY_ITEM,
        "dw::core::Types",
        "arrayItem",
        "Returns the item type of an Array type."
    );
    catalog_function!(
        TYPES_BASE_TYPE_OF,
        "dw::core::Types",
        "baseTypeOf",
        "Returns the base type of a type."
    );
    catalog_function!(
        TYPES_FUNCTION_PARAM_TYPES,
        "dw::core::Types",
        "functionParamTypes",
        "Returns function parameter descriptions."
    );
    catalog_function!(
        TYPES_FUNCTION_RETURN_TYPE,
        "dw::core::Types",
        "functionReturnType",
        "Returns a function return type."
    );
    catalog_function!(
        TYPES_INTERSECTION_ITEMS,
        "dw::core::Types",
        "intersectionItems",
        "Returns the members of an intersection type."
    );
    catalog_function!(
        TYPES_IS_ANY_TYPE,
        "dw::core::Types",
        "isAnyType",
        "Reports whether a type is the Any type."
    );
    catalog_function!(
        TYPES_IS_ARRAY_TYPE,
        "dw::core::Types",
        "isArrayType",
        "Reports whether a type is an Array type."
    );
    catalog_function!(
        TYPES_IS_BINARY_TYPE,
        "dw::core::Types",
        "isBinaryType",
        "Reports whether a type is a Binary type."
    );
    catalog_function!(
        TYPES_IS_BOOLEAN_TYPE,
        "dw::core::Types",
        "isBooleanType",
        "Reports whether a type is a Boolean type."
    );
    catalog_function!(
        TYPES_IS_DATE_TIME_TYPE,
        "dw::core::Types",
        "isDateTimeType",
        "Reports whether a type is a DateTime type."
    );
    catalog_function!(
        TYPES_IS_DATE_TYPE,
        "dw::core::Types",
        "isDateType",
        "Reports whether a type is a Date type."
    );
    catalog_function!(
        TYPES_IS_FUNCTION_TYPE,
        "dw::core::Types",
        "isFunctionType",
        "Reports whether a type is a Function type."
    );
    catalog_function!(
        TYPES_IS_INTERSECTION_TYPE,
        "dw::core::Types",
        "isIntersectionType",
        "Reports whether a type is an intersection type."
    );
    catalog_function!(
        TYPES_IS_KEY_TYPE,
        "dw::core::Types",
        "isKeyType",
        "Reports whether a type is a Key type."
    );
    catalog_function!(
        TYPES_IS_LITERAL_TYPE,
        "dw::core::Types",
        "isLiteralType",
        "Reports whether a type is a Literal type."
    );
    catalog_function!(
        TYPES_IS_LOCAL_DATE_TIME_TYPE,
        "dw::core::Types",
        "isLocalDateTimeType",
        "Reports whether a type is a LocalDateTime type."
    );
    catalog_function!(
        TYPES_IS_LOCAL_TIME_TYPE,
        "dw::core::Types",
        "isLocalTimeType",
        "Reports whether a type is a LocalTime type."
    );
    catalog_function!(
        TYPES_IS_NAMESPACE_TYPE,
        "dw::core::Types",
        "isNamespaceType",
        "Reports whether a type is a Namespace type."
    );
    catalog_function!(
        TYPES_IS_NOTHING_TYPE,
        "dw::core::Types",
        "isNothingType",
        "Reports whether a type is a Nothing type."
    );
    catalog_function!(
        TYPES_IS_NULL_TYPE,
        "dw::core::Types",
        "isNullType",
        "Reports whether a type is a Null type."
    );
    catalog_function!(
        TYPES_IS_NUMBER_TYPE,
        "dw::core::Types",
        "isNumberType",
        "Reports whether a type is a Number type."
    );
    catalog_function!(
        TYPES_IS_OBJECT_TYPE,
        "dw::core::Types",
        "isObjectType",
        "Reports whether a type is an Object type."
    );
    catalog_function!(
        TYPES_IS_PERIOD_TYPE,
        "dw::core::Types",
        "isPeriodType",
        "Reports whether a type is a Period type."
    );
    catalog_function!(
        TYPES_IS_RANGE_TYPE,
        "dw::core::Types",
        "isRangeType",
        "Reports whether a type is a Range type."
    );
    catalog_function!(
        TYPES_IS_REFERENCE_TYPE,
        "dw::core::Types",
        "isReferenceType",
        "Reports whether a type is a reference type."
    );
    catalog_function!(
        TYPES_IS_REGEX_TYPE,
        "dw::core::Types",
        "isRegexType",
        "Reports whether a type is a Regex type."
    );
    catalog_function!(
        TYPES_IS_STRING_TYPE,
        "dw::core::Types",
        "isStringType",
        "Reports whether a type is a String type."
    );
    catalog_function!(
        TYPES_IS_TIME_TYPE,
        "dw::core::Types",
        "isTimeType",
        "Reports whether a type is a Time type."
    );
    catalog_function!(
        TYPES_IS_TIME_ZONE_TYPE,
        "dw::core::Types",
        "isTimeZoneType",
        "Reports whether a type is a TimeZone type."
    );
    catalog_function!(
        TYPES_IS_TYPE_TYPE,
        "dw::core::Types",
        "isTypeType",
        "Reports whether a type is a Type type."
    );
    catalog_function!(
        TYPES_IS_UNION_TYPE,
        "dw::core::Types",
        "isUnionType",
        "Reports whether a type is a union type."
    );
    catalog_function!(
        TYPES_IS_URI_TYPE,
        "dw::core::Types",
        "isUriType",
        "Reports whether a type is a URI type."
    );
    catalog_function!(
        TYPES_LITERAL_VALUE_OF,
        "dw::core::Types",
        "literalValueOf",
        "Returns the value of a literal type."
    );
    catalog_function!(
        TYPES_METADATA_OF,
        "dw::core::Types",
        "metadataOf",
        "Returns metadata attached to a type."
    );
    catalog_function!(
        TYPES_NAME_OF,
        "dw::core::Types",
        "nameOf",
        "Returns the name of a type."
    );
    catalog_function!(
        TYPES_OBJECT_FIELDS,
        "dw::core::Types",
        "objectFields",
        "Returns the fields of an object type."
    );
    catalog_function!(
        TYPES_UNION_ITEMS,
        "dw::core::Types",
        "unionItems",
        "Returns the members of a union type."
    );
    catalog_function!(
        TREE_AS_EXPRESSION_STRING,
        "dw::util::Tree",
        "asExpressionString",
        "Renders a Path as a selector expression."
    );
    catalog_function!(
        TREE_FILTER_ARRAY_LEAFS,
        "dw::util::Tree",
        "filterArrayLeafs",
        "Filters leaf values inside arrays."
    );
    catalog_function!(
        TREE_FILTER_OBJECT_LEAFS,
        "dw::util::Tree",
        "filterObjectLeafs",
        "Filters leaf values inside objects."
    );
    catalog_function!(
        TREE_FILTER_TREE,
        "dw::util::Tree",
        "filterTree",
        "Filters nodes throughout a tree."
    );
    catalog_function!(
        TREE_IS_ARRAY_TYPE,
        "dw::util::Tree",
        "isArrayType",
        "Reports whether a path element selects an array item."
    );
    catalog_function!(
        TREE_IS_ATTRIBUTE_TYPE,
        "dw::util::Tree",
        "isAttributeType",
        "Reports whether a path element selects an attribute."
    );
    catalog_function!(
        TREE_IS_OBJECT_TYPE,
        "dw::util::Tree",
        "isObjectType",
        "Reports whether a path element selects an object field."
    );
    catalog_function!(
        TREE_MAP_LEAF_VALUES,
        "dw::util::Tree",
        "mapLeafValues",
        "Maps terminal leaf values in a tree."
    );
    catalog_function!(
        TREE_NODE_EXISTS,
        "dw::util::Tree",
        "nodeExists",
        "Reports whether any tree node matches a predicate."
    );
    catalog_function!(
        COERCIONS_TO_ARRAY,
        "dw::util::Coercions",
        "toArray",
        "Converts text into an array of characters."
    );
    catalog_function!(
        COERCIONS_TO_BINARY,
        "dw::util::Coercions",
        "toBinary",
        "Converts text to Binary using an encoding."
    );
    catalog_function!(
        COERCIONS_TO_BOOLEAN,
        "dw::util::Coercions",
        "toBoolean",
        "Converts a value to Boolean."
    );
    catalog_function!(
        COERCIONS_TO_DATE,
        "dw::util::Coercions",
        "toDate",
        "Converts a value to Date."
    );
    catalog_function!(
        COERCIONS_TO_DATE_OR_NULL,
        "dw::util::Coercions",
        "toDateOrNull",
        "Converts a value to Date or returns null."
    );
    catalog_function!(
        COERCIONS_TO_DATE_TIME,
        "dw::util::Coercions",
        "toDateTime",
        "Converts a value to DateTime."
    );
    catalog_function!(
        COERCIONS_TO_DATE_TIME_OR_NULL,
        "dw::util::Coercions",
        "toDateTimeOrNull",
        "Converts a value to DateTime or returns null."
    );
    catalog_function!(
        COERCIONS_TO_LOCAL_DATE_TIME,
        "dw::util::Coercions",
        "toLocalDateTime",
        "Converts a value to LocalDateTime."
    );
    catalog_function!(
        COERCIONS_TO_LOCAL_DATE_TIME_OR_NULL,
        "dw::util::Coercions",
        "toLocalDateTimeOrNull",
        "Converts a value to LocalDateTime or returns null."
    );
    catalog_function!(
        COERCIONS_TO_LOCAL_TIME,
        "dw::util::Coercions",
        "toLocalTime",
        "Converts a value to LocalTime."
    );
    catalog_function!(
        COERCIONS_TO_LOCAL_TIME_OR_NULL,
        "dw::util::Coercions",
        "toLocalTimeOrNull",
        "Converts a value to LocalTime or returns null."
    );
    catalog_function!(
        COERCIONS_TO_NUMBER,
        "dw::util::Coercions",
        "toNumber",
        "Converts a value to Number."
    );
    catalog_function!(
        COERCIONS_TO_NUMBER_OR_NULL,
        "dw::util::Coercions",
        "toNumberOrNull",
        "Converts a value to Number or returns null."
    );
    catalog_function!(
        COERCIONS_TO_PERIOD,
        "dw::util::Coercions",
        "toPeriod",
        "Converts a value to Period."
    );
    catalog_function!(
        COERCIONS_TO_REGEX,
        "dw::util::Coercions",
        "toRegex",
        "Converts text to a regular expression."
    );
    catalog_function!(
        COERCIONS_TO_STRING,
        "dw::util::Coercions",
        "toString",
        "Converts a value to String."
    );
    catalog_function!(
        COERCIONS_TO_TIME,
        "dw::util::Coercions",
        "toTime",
        "Converts a value to Time."
    );
    catalog_function!(
        COERCIONS_TO_TIME_OR_NULL,
        "dw::util::Coercions",
        "toTimeOrNull",
        "Converts a value to Time or returns null."
    );
    catalog_function!(
        COERCIONS_TO_TIME_ZONE,
        "dw::util::Coercions",
        "toTimeZone",
        "Converts a value to TimeZone."
    );
    catalog_function!(
        COERCIONS_TO_URI,
        "dw::util::Coercions",
        "toURI",
        "Converts text to a URI value."
    );
    catalog_function!(
        COERCIONS_TO_URI_ALIAS,
        "dw::util::Coercions",
        "toUri",
        "Converts text to a URI value."
    );
    catalog_function!(
        VALUES_ATTR,
        "dw::util::Values",
        "attr",
        "Creates an attribute path element."
    );
    catalog_function!(
        VALUES_FIELD,
        "dw::util::Values",
        "field",
        "Creates an object field path element."
    );
    catalog_function!(
        VALUES_INDEX,
        "dw::util::Values",
        "index",
        "Creates an array index path element."
    );
    catalog_function!(
        VALUES_MASK,
        "dw::util::Values",
        "mask",
        "Replaces values selected by a path."
    );
    catalog_function!(
        VALUES_UPDATE,
        "dw::util::Values",
        "update",
        "Updates a value selected by a path."
    );
    catalog_function!(
        MATH_ACOS,
        "dw::util::Math",
        "acos",
        "Returns the arc cosine of a number."
    );
    catalog_function!(
        MATH_ASIN,
        "dw::util::Math",
        "asin",
        "Returns the arc sine of a number."
    );
    catalog_function!(
        MATH_ATAN,
        "dw::util::Math",
        "atan",
        "Returns the arc tangent of a number."
    );
    catalog_function!(
        MATH_COS,
        "dw::util::Math",
        "cos",
        "Returns the cosine of an angle in radians."
    );
    catalog_function!(
        MATH_DECIMAL_ADD,
        "dw::util::Math",
        "decimalAdd",
        "Adds decimals with explicit rounding context."
    );
    catalog_function!(
        MATH_DECIMAL_DIVIDE,
        "dw::util::Math",
        "decimalDivide",
        "Divides decimals with explicit rounding context."
    );
    catalog_function!(
        MATH_DECIMAL_MULTIPLY,
        "dw::util::Math",
        "decimalMultiply",
        "Multiplies decimals with explicit rounding context."
    );
    catalog_function!(
        MATH_DECIMAL_POW,
        "dw::util::Math",
        "decimalPow",
        "Raises a decimal to a power with rounding context."
    );
    catalog_function!(
        MATH_DECIMAL_ROUND,
        "dw::util::Math",
        "decimalRound",
        "Rounds a decimal with explicit precision."
    );
    catalog_function!(
        MATH_DECIMAL_SQRT,
        "dw::util::Math",
        "decimalSqrt",
        "Returns a decimal square root with rounding context."
    );
    catalog_function!(
        MATH_DECIMAL_SUBTRACT,
        "dw::util::Math",
        "decimalSubtract",
        "Subtracts decimals with explicit rounding context."
    );
    catalog_function!(
        MATH_LOG_10,
        "dw::util::Math",
        "log10",
        "Returns the base-10 logarithm."
    );
    catalog_function!(
        MATH_LOGN,
        "dw::util::Math",
        "logn",
        "Returns the natural logarithm."
    );
    catalog_function!(
        MATH_SIN,
        "dw::util::Math",
        "sin",
        "Returns the sine of an angle in radians."
    );
    catalog_function!(
        MATH_TAN,
        "dw::util::Math",
        "tan",
        "Returns the tangent of an angle in radians."
    );
    catalog_function!(
        MATH_TO_DEGREES,
        "dw::util::Math",
        "toDegrees",
        "Converts radians to degrees."
    );
    catalog_function!(
        MATH_TO_RADIANS,
        "dw::util::Math",
        "toRadians",
        "Converts degrees to radians."
    );
    catalog_function!(
        DIFF_DIFF,
        "dw::util::Diff",
        "diff",
        "Compares two values and returns structural differences."
    );

    /// Every supported function, grouped by the catalog order below.
    pub const ALL: &'static [Function] = &[
        Self::CORE_MAP,
        Self::CORE_FILTER,
        Self::CORE_REDUCE,
        Self::CORE_GROUP_BY,
        Self::CORE_SIZE_OF,
        Self::CORE_IS_EMPTY,
        Self::CORE_SUM,
        Self::CORE_MAP_OBJECT,
        Self::CORE_PLUCK,
        Self::CORE_FILTER_OBJECT,
        Self::CORE_FLAT_MAP,
        Self::CORE_ENTRIES_OF,
        Self::CORE_NAMES_OF,
        Self::CORE_KEYS_OF,
        Self::CORE_VALUES_OF,
        Self::CORE_FLATTEN,
        Self::CORE_DISTINCT_BY,
        Self::CORE_ORDER_BY,
        Self::CORE_FIND,
        Self::CORE_INDEX_OF,
        Self::CORE_LAST_INDEX_OF,
        Self::CORE_CONTAINS,
        Self::CORE_TO,
        Self::CORE_ZIP,
        Self::CORE_UNZIP,
        Self::CORE_UPPER,
        Self::CORE_LOWER,
        Self::CORE_TRIM,
        Self::CORE_STARTS_WITH,
        Self::CORE_ENDS_WITH,
        Self::CORE_SPLIT_BY,
        Self::CORE_WITH,
        Self::CORE_REPLACE,
        Self::CORE_JOIN_BY,
        Self::CORE_ABS,
        Self::CORE_CEIL,
        Self::CORE_FLOOR,
        Self::CORE_ROUND,
        Self::CORE_POW,
        Self::CORE_SQRT,
        Self::CORE_MOD,
        Self::CORE_IS_INTEGER,
        Self::CORE_IS_DECIMAL,
        Self::CORE_IS_ODD,
        Self::CORE_IS_EVEN,
        Self::CORE_IS_BLANK,
        Self::CORE_MIN_BY,
        Self::CORE_MAX_BY,
        Self::CORE_MAX,
        Self::CORE_MIN,
        Self::CORE_AVG,
        Self::CORE_THEN,
        Self::CORE_ON_NULL,
        Self::CORE_MATCHES,
        Self::CORE_MATCH,
        Self::CORE_SCAN,
        Self::CORE_TYPE_OF,
        Self::CORE_RANDOM,
        Self::CORE_RANDOM_INT,
        Self::CORE_UUID,
        Self::CORE_LOG,
        Self::CORE_DAYS_BETWEEN,
        Self::CORE_IS_LEAP_YEAR,
        Self::CORE_NOW,
        Self::CORE_READ,
        Self::CORE_WRITE,
        Self::CORE_READ_URL,
        Self::CORE_LOG_DEBUG,
        Self::CORE_LOG_INFO,
        Self::CORE_LOG_WARN,
        Self::CORE_LOG_ERROR,
        Self::CORE_LOG_WITH,
        Self::CORE_PLUS_PLUS,
        Self::CORE_MINUS_MINUS,
        Self::ARRAYS_COUNT_BY,
        Self::ARRAYS_SUM_BY,
        Self::ARRAYS_SOME,
        Self::ARRAYS_EVERY,
        Self::ARRAYS_TAKE,
        Self::ARRAYS_DROP,
        Self::ARRAYS_SLICE,
        Self::ARRAYS_INDEX_OF,
        Self::ARRAYS_INDEX_WHERE,
        Self::ARRAYS_SPLIT_AT,
        Self::ARRAYS_TAKE_WHILE,
        Self::ARRAYS_DROP_WHILE,
        Self::ARRAYS_PARTITION,
        Self::ARRAYS_FIRST_WITH,
        Self::ARRAYS_DIVIDE_BY,
        Self::ARRAYS_SPLIT_WHERE,
        Self::ARRAYS_JOIN,
        Self::ARRAYS_LEFT_JOIN,
        Self::ARRAYS_OUTER_JOIN,
        Self::STRINGS_APPEND_IF_MISSING,
        Self::STRINGS_CAMELIZE,
        Self::STRINGS_CAPITALIZE,
        Self::STRINGS_CHAR_CODE,
        Self::STRINGS_CHAR_CODE_AT,
        Self::STRINGS_COLLAPSE,
        Self::STRINGS_COUNT_CHARACTERS_BY,
        Self::STRINGS_COUNT_MATCHES,
        Self::STRINGS_DASHERIZE,
        Self::STRINGS_EVERY_CHARACTER,
        Self::STRINGS_FIRST,
        Self::STRINGS_FROM_CHAR_CODE,
        Self::STRINGS_HAMMING_DISTANCE,
        Self::STRINGS_IS_ALPHA,
        Self::STRINGS_IS_ALPHANUMERIC,
        Self::STRINGS_IS_LOWER_CASE,
        Self::STRINGS_IS_NUMERIC,
        Self::STRINGS_IS_UPPER_CASE,
        Self::STRINGS_IS_WHITESPACE,
        Self::STRINGS_LAST,
        Self::STRINGS_LEFT_PAD,
        Self::STRINGS_LEVENSHTEIN_DISTANCE,
        Self::STRINGS_LINES,
        Self::STRINGS_MAP_STRING,
        Self::STRINGS_ORDINALIZE,
        Self::STRINGS_PLURALIZE,
        Self::STRINGS_PREPEND_IF_MISSING,
        Self::STRINGS_REMOVE,
        Self::STRINGS_REPEAT,
        Self::STRINGS_REPLACE_ALL,
        Self::STRINGS_REVERSE,
        Self::STRINGS_RIGHT_PAD,
        Self::STRINGS_SINGULARIZE,
        Self::STRINGS_SOME_CHARACTER,
        Self::STRINGS_SUBSTRING,
        Self::STRINGS_SUBSTRING_AFTER,
        Self::STRINGS_SUBSTRING_AFTER_LAST,
        Self::STRINGS_SUBSTRING_BEFORE,
        Self::STRINGS_SUBSTRING_BEFORE_LAST,
        Self::STRINGS_SUBSTRING_BY,
        Self::STRINGS_SUBSTRING_EVERY,
        Self::STRINGS_UNDERSCORE,
        Self::STRINGS_UNWRAP,
        Self::STRINGS_WITH_MAX_SIZE,
        Self::STRINGS_WORDS,
        Self::STRINGS_WRAP_IF_MISSING,
        Self::STRINGS_WRAP_WITH,
        Self::OBJECTS_ENTRY_SET,
        Self::OBJECTS_NAME_SET,
        Self::OBJECTS_KEY_SET,
        Self::OBJECTS_VALUE_SET,
        Self::OBJECTS_EVERY_ENTRY,
        Self::OBJECTS_SOME_ENTRY,
        Self::OBJECTS_MERGE_WITH,
        Self::OBJECTS_DIVIDE_BY,
        Self::OBJECTS_TAKE_WHILE,
        Self::NUMBERS_TO_HEX,
        Self::NUMBERS_TO_BINARY,
        Self::NUMBERS_FROM_HEX,
        Self::NUMBERS_FROM_BINARY,
        Self::NUMBERS_FROM_RADIX_NUMBER,
        Self::NUMBERS_TO_RADIX_NUMBER,
        Self::DATES_AT_BEGINNING_OF_DAY,
        Self::DATES_AT_BEGINNING_OF_HOUR,
        Self::DATES_AT_BEGINNING_OF_MONTH,
        Self::DATES_AT_BEGINNING_OF_WEEK,
        Self::DATES_AT_BEGINNING_OF_YEAR,
        Self::DATES_DATE,
        Self::DATES_DATE_TIME,
        Self::DATES_LOCAL_DATE_TIME,
        Self::DATES_LOCAL_TIME,
        Self::DATES_TIME,
        Self::DATES_TODAY,
        Self::DATES_TOMORROW,
        Self::DATES_YESTERDAY,
        Self::PERIODS_BETWEEN,
        Self::PERIODS_DAYS,
        Self::PERIODS_DURATION,
        Self::PERIODS_HOURS,
        Self::PERIODS_MINUTES,
        Self::PERIODS_MONTHS,
        Self::PERIODS_PERIOD,
        Self::PERIODS_SECONDS,
        Self::PERIODS_YEARS,
        Self::BINARIES_CONCAT_WITH,
        Self::BINARIES_FROM_BASE_64,
        Self::BINARIES_FROM_HEX,
        Self::BINARIES_READ_LINES_WITH,
        Self::BINARIES_TO_BASE_64,
        Self::BINARIES_TO_HEX,
        Self::BINARIES_WRITE_LINES_WITH,
        Self::URL_COMPOSE,
        Self::URL_DECODE_URI,
        Self::URL_DECODE_URI_COMPONENT,
        Self::URL_ENCODE_URI,
        Self::URL_ENCODE_URI_COMPONENT,
        Self::URL_PARSE_URI,
        Self::TYPES_ARRAY_ITEM,
        Self::TYPES_BASE_TYPE_OF,
        Self::TYPES_FUNCTION_PARAM_TYPES,
        Self::TYPES_FUNCTION_RETURN_TYPE,
        Self::TYPES_INTERSECTION_ITEMS,
        Self::TYPES_IS_ANY_TYPE,
        Self::TYPES_IS_ARRAY_TYPE,
        Self::TYPES_IS_BINARY_TYPE,
        Self::TYPES_IS_BOOLEAN_TYPE,
        Self::TYPES_IS_DATE_TIME_TYPE,
        Self::TYPES_IS_DATE_TYPE,
        Self::TYPES_IS_FUNCTION_TYPE,
        Self::TYPES_IS_INTERSECTION_TYPE,
        Self::TYPES_IS_KEY_TYPE,
        Self::TYPES_IS_LITERAL_TYPE,
        Self::TYPES_IS_LOCAL_DATE_TIME_TYPE,
        Self::TYPES_IS_LOCAL_TIME_TYPE,
        Self::TYPES_IS_NAMESPACE_TYPE,
        Self::TYPES_IS_NOTHING_TYPE,
        Self::TYPES_IS_NULL_TYPE,
        Self::TYPES_IS_NUMBER_TYPE,
        Self::TYPES_IS_OBJECT_TYPE,
        Self::TYPES_IS_PERIOD_TYPE,
        Self::TYPES_IS_RANGE_TYPE,
        Self::TYPES_IS_REFERENCE_TYPE,
        Self::TYPES_IS_REGEX_TYPE,
        Self::TYPES_IS_STRING_TYPE,
        Self::TYPES_IS_TIME_TYPE,
        Self::TYPES_IS_TIME_ZONE_TYPE,
        Self::TYPES_IS_TYPE_TYPE,
        Self::TYPES_IS_UNION_TYPE,
        Self::TYPES_IS_URI_TYPE,
        Self::TYPES_LITERAL_VALUE_OF,
        Self::TYPES_METADATA_OF,
        Self::TYPES_NAME_OF,
        Self::TYPES_OBJECT_FIELDS,
        Self::TYPES_UNION_ITEMS,
        Self::TREE_AS_EXPRESSION_STRING,
        Self::TREE_FILTER_ARRAY_LEAFS,
        Self::TREE_FILTER_OBJECT_LEAFS,
        Self::TREE_FILTER_TREE,
        Self::TREE_IS_ARRAY_TYPE,
        Self::TREE_IS_ATTRIBUTE_TYPE,
        Self::TREE_IS_OBJECT_TYPE,
        Self::TREE_MAP_LEAF_VALUES,
        Self::TREE_NODE_EXISTS,
        Self::COERCIONS_TO_ARRAY,
        Self::COERCIONS_TO_BINARY,
        Self::COERCIONS_TO_BOOLEAN,
        Self::COERCIONS_TO_DATE,
        Self::COERCIONS_TO_DATE_OR_NULL,
        Self::COERCIONS_TO_DATE_TIME,
        Self::COERCIONS_TO_DATE_TIME_OR_NULL,
        Self::COERCIONS_TO_LOCAL_DATE_TIME,
        Self::COERCIONS_TO_LOCAL_DATE_TIME_OR_NULL,
        Self::COERCIONS_TO_LOCAL_TIME,
        Self::COERCIONS_TO_LOCAL_TIME_OR_NULL,
        Self::COERCIONS_TO_NUMBER,
        Self::COERCIONS_TO_NUMBER_OR_NULL,
        Self::COERCIONS_TO_PERIOD,
        Self::COERCIONS_TO_REGEX,
        Self::COERCIONS_TO_STRING,
        Self::COERCIONS_TO_TIME,
        Self::COERCIONS_TO_TIME_OR_NULL,
        Self::COERCIONS_TO_TIME_ZONE,
        Self::COERCIONS_TO_URI,
        Self::COERCIONS_TO_URI_ALIAS,
        Self::VALUES_ATTR,
        Self::VALUES_FIELD,
        Self::VALUES_INDEX,
        Self::VALUES_MASK,
        Self::VALUES_UPDATE,
        Self::MATH_ACOS,
        Self::MATH_ASIN,
        Self::MATH_ATAN,
        Self::MATH_COS,
        Self::MATH_DECIMAL_ADD,
        Self::MATH_DECIMAL_DIVIDE,
        Self::MATH_DECIMAL_MULTIPLY,
        Self::MATH_DECIMAL_POW,
        Self::MATH_DECIMAL_ROUND,
        Self::MATH_DECIMAL_SQRT,
        Self::MATH_DECIMAL_SUBTRACT,
        Self::MATH_LOG_10,
        Self::MATH_LOGN,
        Self::MATH_SIN,
        Self::MATH_TAN,
        Self::MATH_TO_DEGREES,
        Self::MATH_TO_RADIANS,
        Self::DIFF_DIFF,
    ];
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Function;

    #[test]
    fn catalog_matches_runtime_registry() {
        let mut catalog = BTreeSet::new();
        for function in Function::ALL {
            assert!(!function.name.is_empty());
            assert!(!function.namespace.is_empty());
            assert!(!function.description.is_empty());
            assert!(
                catalog.insert((function.namespace, function.name)),
                "duplicate catalog entry: {}::{}",
                function.namespace,
                function.name
            );
        }
        assert_eq!(catalog.len(), Function::ALL.len());

        let runtime: BTreeSet<&'static str> =
            crate::stdlib::registry_names().iter().copied().collect();
        let catalog_names: BTreeSet<&'static str> = catalog.iter().map(|(_, name)| *name).collect();
        assert_eq!(runtime, catalog_names);

        // Aliases exposed by public modules must resolve to a catalog entry.
        for module in ["dw::core::Binaries", "dw::core::Types"] {
            for (local, _) in crate::stdlib::module_aliases(module) {
                assert!(
                    catalog.contains(&(module, *local)),
                    "missing alias in catalog: {module}::{local}"
                );
            }
        }
    }
}
