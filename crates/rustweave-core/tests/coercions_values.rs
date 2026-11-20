// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for coercions and value updates.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  array: toArray("river"),
  emptyArray: toArray(""),
  boolean: [toBoolean("true"), toBoolean("false"), toBoolean("FALSE"), toBoolean("TrUe")],
  date: toDate("2019-04-23"),
  formattedDate: toDate("2019/04/23", "uuuu/MM/dd"),
  dateTime: toDateTime("2019-04-23T08:15:30Z"),
  localDateTime: toLocalDateTime("2019-04-23T08:15:30"),
  localTime: toLocalTime("08:15:30.250", "HH:mm:ss.n"),
  number: toNumber("2.0"),
  periodSeconds: toNumber(|PT2H5M|, "seconds"),
  epoch: toNumber(|2019-04-23T08:15:30Z|),
  millis: toNumber(|2019-04-23T08:15:30Z|, "milliseconds"),
  period: toPeriod("PT2H5M"),
  string: toString(2.0),
  dateString: toString(|2019-04-23|, "uuuu/MM/dd"),
  zone: toTimeZone("-05:00"),
  uri: toURI("https://example.test/path")
}"####,
        r####"{
  "array": [
    "r",
    "i",
    "v",
    "e",
    "r"
  ],
  "emptyArray": [
    
  ],
  "boolean": [
    true,
    false,
    false,
    true
  ],
  "date": "2019-04-23",
  "formattedDate": "2019/04/23",
  "dateTime": "2019-04-23T08:15:30Z",
  "localDateTime": "2019-04-23T08:15:30",
  "localTime": "08:15:30.250",
  "number": 2,
  "periodSeconds": 7500,
  "epoch": 1556007330,
  "millis": 1556007330000,
  "period": "PT2H5M",
  "string": "2",
  "dateString": "2019/04/23",
  "zone": "-05:00",
  "uri": "https://example.test/path"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  a: toDate("2019-23-04", [{format: "yyyy/MM/dd"}, {format: "yyyy-dd-MM"}]),
  b: toDateOrNull("2019-23-04", [{format: "yyyy/MM/dd"}]),
  c: toDateTimeOrNull("2019-04-23 08:15:30Z", [{format: "uuuu/MM/dd HH:mm:ssz"}, {format: "uuuu-MM-dd HH:mm:ssz"}]),
  d: toNumberOrNull("0.125", [{format: "seconds"}, {format: ".000"}]),
  e: toTimeOrNull("08:15:30.250-05:00", [{format: "HH:mm:ss.nxxx"}]),
  f: toLocalDateTimeOrNull("2019-04-23 08:15:30", [{format: "uuuu-MM-dd HH:mm:ss"}])
}"####,
        r####"{
  "a": "2019-23-04",
  "b": null,
  "c": "2019-04-23 08:15:30Z",
  "d": 0.125,
  "e": "08:15:30.250-05:00",
  "f": "2019-04-23 08:15:30"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Coercions
import * from dw::core::Binaries
output application/json
---
{
  binary: toHex(toBinary("RX", "UTF-8")),
  text: toString(toBinary("RX", "UTF-8"), "UTF-8"),
  regex: toString(toRegex("a-Z")),
  period: toPeriod("P1D")
}"####,
        r####"{
  "binary": "5258",
  "text": "RX",
  "regex": "a-Z",
  "period": "P1D"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Values
output application/json
---
{
  field: field("skuCode"),
  index: index(2),
  attribute: attr(null, "lang")
}"####,
        r####"{
  "field": {
    "kind": "Object",
    "namespace": null,
    "selector": "skuCode"
  },
  "index": {
    "kind": "Array",
    "namespace": null,
    "selector": 2
  },
  "attribute": {
    "kind": "Attribute",
    "namespace": null,
    "selector": "lang"
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Values
output application/json
---
{
  a: {name: "Robin"} update "name" with "Rook",
  b: {name: "Robin"} update field("name") with "Rook",
  c: [4, 5, 6] update 1 with 9,
  d: [4, 5, 6] update index(1) with 9,
  e: [{role: "pilot", name: "Nora"}, {role: "navigator", name: "Silas"}] update "role" with "Crew",
  f: {profile: {name: "Robin"}} update ["profile", field("name")] with "Rook",
  g: {name: "Robin", age: 30} update "age" with ($ + 2),
  h: {name: "Robin", age: 30} mask "age" with 0
}"####,
        r####"{
  "a": {
    "name": "Rook"
  },
  "b": {
    "name": "Rook"
  },
  "c": [
    4,
    9,
    6
  ],
  "d": [
    4,
    9,
    6
  ],
  "e": [
    {
      "role": "Crew",
      "name": "Nora"
    },
    {
      "role": "Crew",
      "name": "Silas"
    }
  ],
  "f": {
    "profile": {
      "name": "Rook"
    }
  },
  "g": {
    "name": "Robin",
    "age": 32
  },
  "h": {
    "name": "Robin",
    "age": 0
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Values
output application/json
---
{members: [{name: "moss"}, {name: "fern"}]} update ["members", index(1), field("name")] with "reed""####,
        r####"{
  "members": [
    {
      "name": "moss"
    },
    {
      "name": "reed"
    }
  ]
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
