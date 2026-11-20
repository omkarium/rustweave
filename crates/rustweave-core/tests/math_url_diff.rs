// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for math, URL, diff, logging, and named operator helpers.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::util::Math
output application/json
---
{
  acos0: acos(0),
  acos02: acos(0.2),
  acosMinus1: acos(-1),
  acos1: acos(1),
  acosInvalid: acos(1.2),
  asin0: asin(0),
  asin02: asin(0.2),
  asinMinus1: asin(-1),
  asinInvalid: asin(1.2),
  atan02: atan(0.2),
  cos02: cos(0.2),
  sin02: sin(0.2),
  tan02: tan(0.2),
  log10100: log10(100),
  log1002: log10(0.2),
  log10Invalid: log10(-5),
  logn100: logn(100),
  logn02: logn(0.2),
  lognInvalid: logn(-5)
}"####,
        r####"{
  "acos0": 1.5707963267948966,
  "acos02": 1.369438406004566,
  "acosMinus1": 3.141592653589793,
  "acos1": 0,
  "acosInvalid": null,
  "asin0": 0,
  "asin02": 0.2013579207903308,
  "asinMinus1": -1.5707963267948966,
  "asinInvalid": null,
  "atan02": 0.19739555984988078,
  "cos02": 0.9800665778412416,
  "sin02": 0.19866933079506122,
  "tan02": 0.2027100355086725,
  "log10100": 2,
  "log1002": -0.6989700043360187,
  "log10Invalid": null,
  "logn100": 4.605170185988092,
  "logn02": -1.6094379124341003,
  "lognInvalid": null
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Math
output application/json
---
{
  add: decimalAdd(2.3, 4.5),
  subtract: decimalSubtract(9, 2.75),
  multiply: decimalMultiply(2.5, 4),
  divide: decimalDivide(1, 8),
  pow: decimalPow(3, 4),
  sqrt: decimalSqrt(16),
  round123: decimalRound(234.567, {precision: 4, roundingMode: "HALF_UP"}),
  round25Up: decimalRound(4.5, {precision: 1, roundingMode: "HALF_UP"}),
  round25Even: decimalRound(4.5, {precision: 1, roundingMode: "HALF_EVEN"})
}"####,
        r####"{
  "add": 6.8,
  "subtract": 6.25,
  "multiply": 10,
  "divide": 0.125,
  "pow": 81,
  "sqrt": 4,
  "round123": 234.6,
  "round25Up": 5,
  "round25Even": 4
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::URL
output application/json
---
{
  component: encodeURIComponent("a b/c?d=e&f"),
  componentSafe: encodeURIComponent("_.-~*'()!"),
  uriReserved: encodeURI("a b/c?d=e&f#g"),
  uriSpaces: encodeURI(" PATH/ TO /ENCODE "),
  decode: decodeURIComponent("%20PATH/%20TO%20/DECODE%20"),
  decodeReserved: decodeURIComponent("%3B%2C%2F%3F%3A%40%26%3D"),
  compose: compose(["https://sample.test/", "/page.html"], ["content folder"]),
  parsed: parseURI("https://sample.test/docs/Guide#section")
}"####,
        r####"{
  "component": "a%20b%2Fc%3Fd%3De%26f",
  "componentSafe": "_.-~*'()!",
  "uriReserved": "a%20b/c?d=e&f#g",
  "uriSpaces": "%20PATH/%20TO%20/ENCODE%20",
  "decode": " PATH/ TO /DECODE ",
  "decodeReserved": ";,/?:@&=",
  "compose": "https://sample.test/content%20folder/page.html",
  "parsed": {
    "isValid": true,
    "raw": "https://sample.test/docs/Guide#section",
    "host": "sample.test",
    "authority": "sample.test",
    "fragment": "section",
    "path": "/docs/Guide",
    "scheme": "https",
    "isAbsolute": true,
    "isOpaque": false
  }
}"####,
    ),
    (
        r####"%dw 2.0
import diff from dw::util::Diff
output application/json
---
{
  missing: diff({x: 2}, {y: 2}),
  arraySize: diff([4, 5, 6], []),
  arraySizeBack: diff([], [4, 5, 6]),
  nested: diff([{x: 2}], [{x: 3}]),
  scalar: diff(5, 9),
  string: diff("north", "southern"),
  equal: diff({x: 2}, {x: 2}),
  equalArray: diff([4, 5, 6], [4, 5, 6]),
  unordered: diff({x: 2, y: 3}, {y: 3, x: 2}, {unordered: true})
}"####,
        r####"{
  "missing": {
    "matches": false,
    "diffs": [
      {
        "expected": "Entry (root).y with type Number",
        "actual": "was not present in object.",
        "path": "(root).y"
      }
    ]
  },
  "arraySize": {
    "matches": false,
    "diffs": [
      {
        "expected": "Array size is 0",
        "actual": "was 3",
        "path": "(root)"
      }
    ]
  },
  "arraySizeBack": {
    "matches": false,
    "diffs": [
      {
        "expected": "Array size is 3",
        "actual": "was 0",
        "path": "(root)"
      }
    ]
  },
  "nested": {
    "matches": false,
    "diffs": [
      {
        "expected": "3",
        "actual": "2",
        "path": "(root)[0].x"
      }
    ]
  },
  "scalar": {
    "matches": false,
    "diffs": [
      {
        "expected": "9",
        "actual": "5",
        "path": "(root)"
      }
    ]
  },
  "string": {
    "matches": false,
    "diffs": [
      {
        "expected": "\"southern\"",
        "actual": "\"north\"",
        "path": "(root)"
      }
    ]
  },
  "equal": {
    "matches": true,
    "diffs": [
      
    ]
  },
  "equalArray": {
    "matches": true,
    "diffs": [
      
    ]
  },
  "unordered": {
    "matches": true,
    "diffs": [
      
    ]
  }
}"####,
    ),
    (
        r####"{
  debug: logDebug("level", 9),
  info: logInfo("level", 8),
  warn: logWarn("level", 7),
  error: logError("level", 6),
  with: logWith("WARN", "level", 5),
  plus: plusPlus([2, 4], [6, 8]),
  minus: minusMinus([2, 4, 2, 6], [2])
}"####,
        r####"{
  "debug": 9,
  "info": 8,
  "warn": 7,
  "error": 6,
  "with": 5,
  "plus": [
    2,
    4,
    6,
    8
  ],
  "minus": [
    4,
    6
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
