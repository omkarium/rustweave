//! Behaviour tests for regular expressions.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { split: "a1b2" splitBy /[0-9]/, find: "a1b22" find /([0-9])([0-9]?)/,        scan: "ab" scan /(a)(x)?(b)/, match: "a1b2" match /([a-z])([0-9])/,        matches: ["Aa" matches /a/, "a1" matches /[a-z][0-9]/],        contains: ["a1b2" contains /[0-9]/, "abc" contains /[0-9]/],        replaceStr: "a1b2" replace /[0-9]/ with "X",        replaceFn: "a1b2" replace /([a-z])([0-9])/ with (m, i) -> m[2] ++ m[1],        replaceIdx: "a1b2" replace /[0-9]/ with (m, i) -> i,        count: "hello, ciao!" countMatches /[aeiou]/ }"####,
        r####"{
  "split": [
    "a",
    "b"
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
      "ab",
      "a",
      null,
      "b"
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
  "replaceStr": "aXbX",
  "replaceFn": "1a2b",
  "replaceIdx": "a0b1",
  "count": 5
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { trailing: "a1" splitBy /[0-9]/, emptyText: "" splitBy /x/,        noMatch: "abc" find /x/, unicode: "é1" find /[0-9]/, div: 10/2,        words: words("  one\ntwo\tthree  "), lines: lines("a\r\nb\rc\nd"),        scanNull: "ab" scan /(a)(x)?(b)/ }"####,
        r####"{
  "trailing": [
    "a"
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
  "div": 5,
  "words": [
    "one",
    "two",
    "three"
  ],
  "lines": [
    "a",
    "b",
    "c",
    "d"
  ],
  "scanNull": [
    [
      "ab",
      "a",
      null,
      "b"
    ]
  ]
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
