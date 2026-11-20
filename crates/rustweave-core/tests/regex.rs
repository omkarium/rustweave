// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for regular expressions.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{
  split: "x7y8" splitBy /[0-9]/,
  find: "x7y88" find /([0-9])([0-9]?)/,
  scan: "cd" scan /(c)(z)?(d)/,
  match: "x7y8" match /([a-z])([0-9])/,
  matches: ["Bb" matches /b/, "b7" matches /[a-z][0-9]/],
  contains: ["x7y8" contains /[0-9]/, "xyz" contains /[0-9]/],
  replaceStr: "x7y8" replace /[0-9]/ with "Z",
  replaceFn: "x7y8" replace /([a-z])([0-9])/ with (m, i) -> m[2] ++ m[1],
  replaceIdx: "x7y8" replace /[0-9]/ with (m, i) -> i,
  count: "rhythm, sync!" countMatches /[aeiou]/
}"####,
        r####"{
  "split": [
    "x",
    "y"
  ],
  "find": [
    [
      1,
      1,
      2
    ],
    [
      3,
      3,
      4
    ]
  ],
  "scan": [
    [
      "cd",
      "c",
      null,
      "d"
    ]
  ],
  "match": [
    
  ],
  "matches": [
    false,
    true
  ],
  "contains": [
    true,
    false
  ],
  "replaceStr": "xZyZ",
  "replaceFn": "7x8y",
  "replaceIdx": "x0y1",
  "count": 0
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{
  trailing: "x7" splitBy /[0-9]/,
  emptyText: "" splitBy /z/,
  noMatch: "xyz" find /q/,
  unicode: "é7" find /[0-9]/,
  div: 12/3,
  words: words("  red\ngreen\tblue  "),
  lines: lines("n\r\ns\rn\nl"),
  scanNull: "cd" scan /(c)(z)?(d)/
}"####,
        r####"{
  "trailing": [
    "x"
  ],
  "emptyText": [
    ""
  ],
  "noMatch": [
    
  ],
  "unicode": [
    [
      1
    ]
  ],
  "div": 4,
  "words": [
    "red",
    "green",
    "blue"
  ],
  "lines": [
    "n",
    "s",
    "n",
    "l"
  ],
  "scanNull": [
    [
      "cd",
      "c",
      null,
      "d"
    ]
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
