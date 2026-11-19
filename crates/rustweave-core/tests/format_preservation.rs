// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for preserving parse formats and type-parameter output.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn parsed_temporal_values_keep_their_input_format() {
    let script = r#"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  date: toDate("2023-28-03", [{format: "yyyy/MM/dd"}, {format: "yyyy-dd-MM"}]),
  dateTime: toDateTimeOrNull("2003-10-01 23:57:59Z", [{format: "uuuu/MM/dd HH:mm:ssz"}, {format: "uuuu-MM-dd HH:mm:ssz"}]),
  literalDate: |2003-10-01| as Date {format: "uuuu/MM/dd"},
  localTime: toLocalTime("13:44:12.283", "HH:mm:ss.n"),
  localDate: toLocalDateTime("2003-10-01 23:57:59", "uuuu-MM-dd HH:mm:ss")
}"#;
    let expected = r#"{
  "date": "2023-28-03",
  "dateTime": "2003-10-01 23:57:59Z",
  "literalDate": "2003/10/01",
  "localTime": "13:44:12.283",
  "localDate": "2003-10-01 23:57:59"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn string_casts_and_arithmetic_revert_to_iso() {
    let script = r#"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  castString: (toDate("2023-28-03", [{format: "yyyy-dd-MM"}])) as String,
  toStringResult: toString(toDate("2023-28-03", [{format: "yyyy-dd-MM"}])),
  castDateTimeString: (toDateTimeOrNull("2003-10-01 23:57:59Z", [{format: "uuuu-MM-dd HH:mm:ssz"}])) as String,
  plusPeriod: (|2003-10-01| as Date {format: "uuuu/MM/dd"}) + |P1D|,
  minusPeriod: (|2003-10-01| as Date {format: "uuuu/MM/dd"}) - |P1D|,
  dateDifference: (|2003-10-02| as Date {format: "uuuu/MM/dd"}) - (|2003-10-01| as Date {format: "uuuu/MM/dd"})
}"#;
    let expected = r#"{
  "castString": "2023-03-28",
  "toStringResult": "2023-03-28",
  "castDateTimeString": "2003-10-01T23:57:59Z",
  "plusPeriod": "2003-10-02",
  "minusPeriod": "2003-09-30",
  "dateDifference": "PT24H"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn am_pm_format_token_is_lowercase() {
    let script = r#"{
  am: |2020-10-01T09:57:59| as String {format: "KK:mm:ss a"},
  pm: |2020-10-01T23:57:59| as String {format: "KK:mm:ss a"}
}"#;
    let expected = r#"{
  "am": "09:57:59 am",
  "pm": "11:57:59 pm"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn function_param_types_have_only_param_type_and_optional() {
    let script = r#"%dw 2.0
import * from dw::core::Types
type AFunction = (String, Number) -> Number
output application/json
---
functionParamTypes(AFunction)"#;
    let expected = r#"[
  {
    "paramType": "String",
    "optional": false
  },
  {
    "paramType": "Number",
    "optional": false
  }
]"#;
    assert_eq!(run(script), expected);
}
