// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for preserving parse formats and type-parameter output.

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
  date: toDate("1998-14-07", [{format: "yyyy/MM/dd"}, {format: "yyyy-dd-MM"}]),
  dateTime: toDateTimeOrNull("1999-03-21 08:14:02Z", [{format: "uuuu/MM/dd HH:mm:ssz"}, {format: "uuuu-MM-dd HH:mm:ssz"}]),
  literalDate: |1999-03-21| as Date {format: "uuuu/MM/dd"},
  localTime: toLocalTime("08:14:02.375", "HH:mm:ss.n"),
  localDate: toLocalDateTime("1999-03-21 08:14:02", "uuuu-MM-dd HH:mm:ss")
}"####,
        r####"{
  "date": "1998-14-07",
  "dateTime": "1999-03-21 08:14:02Z",
  "literalDate": "1999/03/21",
  "localTime": "08:14:02.375",
  "localDate": "1999-03-21 08:14:02"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Coercions
output application/json
---
{
  castString: (toDate("1998-14-07", [{format: "yyyy-dd-MM"}])) as String,
  toStringResult: toString(toDate("1998-14-07", [{format: "yyyy-dd-MM"}])),
  castDateTimeString: (toDateTimeOrNull("1999-03-21 08:14:02Z", [{format: "uuuu-MM-dd HH:mm:ssz"}])) as String,
  plusPeriod: (|1999-03-21| as Date {format: "uuuu/MM/dd"}) + |P2D|,
  minusPeriod: (|1999-03-21| as Date {format: "uuuu/MM/dd"}) - |P2D|,
  dateDifference: (|1999-03-23| as Date {format: "uuuu/MM/dd"}) - (|1999-03-21| as Date {format: "uuuu/MM/dd"})
}"####,
        r####"{
  "castString": "1998-07-14",
  "toStringResult": "1998-07-14",
  "castDateTimeString": "1999-03-21T08:14:02Z",
  "plusPeriod": "1999-03-23",
  "minusPeriod": "1999-03-19",
  "dateDifference": "PT48H"
}"####,
    ),
    (
        r####"{
  am: |2021-06-15T06:05:09| as String {format: "KK:mm:ss a"},
  pm: |2021-06-15T18:05:09| as String {format: "KK:mm:ss a"}
}"####,
        r####"{
  "am": "06:05:09 am",
  "pm": "06:05:09 pm"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Types
type Mapper = (Number, String) -> Boolean
output application/json
---
functionParamTypes(Mapper)"####,
        r####"[
  {
    "paramType": "Number",
    "optional": false
  },
  {
    "paramType": "String",
    "optional": false
  }
]"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
