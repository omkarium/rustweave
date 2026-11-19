// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for coercions and value updates.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn coercion_functions() {
    let script = r#"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  array: toArray("hola"),
  emptyArray: toArray(""),
  boolean: [toBoolean("true"), toBoolean("false"), toBoolean("FALSE"), toBoolean("TrUe")],
  date: toDate("2015-10-01"),
  formattedDate: toDate("2003/10/01", "uuuu/MM/dd"),
  dateTime: toDateTime("2015-10-01T23:57:59Z"),
  localDateTime: toLocalDateTime("2015-10-01T23:57:59"),
  localTime: toLocalTime("13:44:12.283", "HH:mm:ss.n"),
  number: toNumber("1.0"),
  periodSeconds: toNumber(|PT1H10M|, "seconds"),
  epoch: toNumber(|2015-10-01T23:57:59Z|),
  millis: toNumber(|2015-10-01T23:57:59Z|, "milliseconds"),
  period: toPeriod("PT1H1M"),
  string: toString(1.0),
  dateString: toString(|2003-10-01|, "uuuu/MM/dd"),
  zone: toTimeZone("-03:00"),
  uri: toURI("https://www.google.com/")
}"#;
    let expected = r#"{
  "array": [
    "h",
    "o",
    "l",
    "a"
  ],
  "emptyArray": [
    
  ],
  "boolean": [
    true,
    false,
    false,
    true
  ],
  "date": "2015-10-01",
  "formattedDate": "2003-10-01",
  "dateTime": "2015-10-01T23:57:59Z",
  "localDateTime": "2015-10-01T23:57:59",
  "localTime": "13:44:12.283",
  "number": 1,
  "periodSeconds": 4200,
  "epoch": 1443743879,
  "millis": 1443743879000,
  "period": "PT1H1M",
  "string": "1",
  "dateString": "2003/10/01",
  "zone": "-03:00",
  "uri": "https://www.google.com/"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn coercion_formatters_and_or_null() {
    let script = r#"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  a: toDate("2023-28-03", [{format: "yyyy/MM/dd"}, {format: "yyyy-dd-MM"}]),
  b: toDateOrNull("2023-28-03", [{format: "yyyy/MM/dd"}]),
  c: toDateTimeOrNull("2003-10-01 23:57:59Z", [{format: "uuuu/MM/dd HH:mm:ssz"}, {format: "uuuu-MM-dd HH:mm:ssz"}]),
  d: toNumberOrNull("0.005", [{format: "seconds"}, {format: ".00"}]),
  e: toTimeOrNull("13:44:12.283-08:00", [{format: "HH:mm:ss.nxxx"}]),
  f: toLocalDateTimeOrNull("2003-10-01 23:57:59", [{format: "uuuu-MM-dd HH:mm:ss"}])
}"#;
    let expected = r#"{
  "a": "2023-03-28",
  "b": null,
  "c": "2003-10-01T23:57:59Z",
  "d": 0.005,
  "e": "13:44:12.283-08:00",
  "f": "2003-10-01T23:57:59"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn binary_coercions_and_strings() {
    let script = r#"%dw 2.0
import * from dw::util::Coercions
import * from dw::core::Binaries
output application/json
---
{
  binary: toHex(toBinary("DW", "UTF-8")),
  text: toString(toBinary("DW", "UTF-8"), "UTF-8"),
  regex: toString(toRegex("a-Z")),
  period: toPeriod("P1D")
}"#;
    let expected = r#"{
  "binary": "4457",
  "text": "DW",
  "regex": "a-Z",
  "period": "P1D"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn path_element_factories() {
    let script = r#"%dw 2.0
import * from dw::util::Values
output application/json
---
{
  field: field("myFieldName"),
  index: index(0),
  attribute: attr(null, "myAttr")
}"#;
    let expected = r#"{
  "field": {
    "kind": "Object",
    "namespace": null,
    "selector": "myFieldName"
  },
  "index": {
    "kind": "Array",
    "namespace": null,
    "selector": 0
  },
  "attribute": {
    "kind": "Attribute",
    "namespace": null,
    "selector": "myAttr"
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn update_examples() {
    let script = r#"%dw 2.0
import * from dw::util::Values
output application/json
---
{
  a: {name: "Mariano"} update "name" with "Data Weave",
  b: {name: "Mariano"} update field("name") with "Data Weave",
  c: [1,2,3] update 1 with 5,
  d: [1,2,3] update index(1) with 5,
  e: [{role: "a", name: "spiderman"}, {role: "b", name: "batman"}] update "role" with "Super Hero",
  f: {user: {name: "Mariano"}} update ["user", field("name")] with "Data Weave",
  g: {name: "Mariano", age: 10} update "age" with ($ + 1),
  h: {name: "Mariano", age: 10} mask "age" with 0
}"#;
    let expected = r#"{
  "a": {
    "name": "Data Weave"
  },
  "b": {
    "name": "Data Weave"
  },
  "c": [
    1,
    5,
    3
  ],
  "d": [
    1,
    5,
    3
  ],
  "e": [
    {
      "role": "Super Hero",
      "name": "spiderman"
    },
    {
      "role": "Super Hero",
      "name": "batman"
    }
  ],
  "f": {
    "user": {
      "name": "Data Weave"
    }
  },
  "g": {
    "name": "Mariano",
    "age": 11
  },
  "h": {
    "name": "Mariano",
    "age": 0
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn update_nested_array_path() {
    let script = r#"%dw 2.0
import * from dw::util::Values
output application/json
---
{users: [{name: "ana"}, {name: "ben"}]} update ["users", index(1), field("name")] with "cara""#;
    let expected = r#"{
  "users": [
    {
      "name": "ana"
    },
    {
      "name": "cara"
    }
  ]
}"#;
    assert_eq!(run(script), expected);
}
