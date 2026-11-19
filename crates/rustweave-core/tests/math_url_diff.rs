// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for math, URL, diff, logging, and named operator helpers.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn math_functions() {
    let script = r#"%dw 2.0
import * from dw::util::Math
output application/json
---
{
  acos0: acos(0),
  acos13: acos(0.13),
  acosMinus1: acos(-1),
  acos1: acos(1),
  acosInvalid: acos(1.1),
  asin0: asin(0),
  asin13: asin(0.13),
  asinMinus1: asin(-1),
  asinInvalid: asin(1.1),
  atan13: atan(0.13),
  cos13: cos(0.13),
  sin13: sin(0.13),
  tan13: tan(0.13),
  log1010: log10(10),
  log1013: log10(0.13),
  log10Invalid: log10(-20),
  logn10: logn(10),
  logn13: logn(0.13),
  lognInvalid: logn(-20)
}"#;
    let expected = r#"{
  "acos0": 1.5707963267948966,
  "acos13": 1.440427347091751,
  "acosMinus1": 3.141592653589793,
  "acos1": 0,
  "acosInvalid": null,
  "asin0": 0,
  "asin13": 0.1303689797031455,
  "asinMinus1": -1.5707963267948966,
  "asinInvalid": null,
  "atan13": 0.12927500404814307,
  "cos13": 0.9915618937147881,
  "sin13": 0.12963414261969486,
  "tan13": 0.13073731800446006,
  "log1010": 1,
  "log1013": -0.8860566476931632,
  "log10Invalid": null,
  "logn10": 2.302585092994046,
  "logn13": -2.0402208285265546,
  "lognInvalid": null
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn decimal_functions() {
    let script = r#"%dw 2.0
import * from dw::util::Math
output application/json
---
{
  add: decimalAdd(1.2, 3.4),
  subtract: decimalSubtract(5, 1.25),
  multiply: decimalMultiply(1.5, 2),
  divide: decimalDivide(1, 4),
  pow: decimalPow(2, 10),
  sqrt: decimalSqrt(9),
  round123: decimalRound(123.456, {precision: 4, roundingMode: "HALF_UP"}),
  round25Up: decimalRound(2.5, {precision: 1, roundingMode: "HALF_UP"}),
  round25Even: decimalRound(2.5, {precision: 1, roundingMode: "HALF_EVEN"})
}"#;
    let expected = r#"{
  "add": 4.6,
  "subtract": 3.75,
  "multiply": 3,
  "divide": 0.25,
  "pow": 1024,
  "sqrt": 3,
  "round123": 123.5,
  "round25Up": 3,
  "round25Even": 2
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn url_encoding_and_parsing() {
    let script = r#"%dw 2.0
import * from dw::core::URL
output application/json
---
{
  component: encodeURIComponent(";,/?:@&=+$#"),
  componentSafe: encodeURIComponent("-_.!~*'()"),
  uriReserved: encodeURI(";,/?:@&=+$#"),
  uriSpaces: encodeURI(" PATH/ TO /ENCODE "),
  decode: decodeURIComponent("%20PATH/%20TO%20/DECODE%20"),
  decodeReserved: decodeURIComponent("%3B%2C%2F%3F%3A%40%26%3D"),
  compose: compose(["http://examplewebsite.com/", "/page.html"], ["content folder"]),
  parsed: parseURI("https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#footer")
}"#;
    let expected = r#"{
  "component": "%3B%2C%2F%3F%3A%40%26%3D%2B%24%23",
  "componentSafe": "-_.!~*'()",
  "uriReserved": ";,/?:@&=+$#",
  "uriSpaces": "%20PATH/%20TO%20/ENCODE%20",
  "decode": " PATH/ TO /DECODE ",
  "decodeReserved": ";,/?:@&=",
  "compose": "http://examplewebsite.com/content%20folder/page.html",
  "parsed": {
    "isValid": true,
    "raw": "https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#footer",
    "host": "en.wikipedia.org",
    "authority": "en.wikipedia.org",
    "fragment": "footer",
    "path": "/wiki/Uniform_Resource_Identifier",
    "scheme": "https",
    "isAbsolute": true,
    "isOpaque": false
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn diff_examples() {
    let script = r#"%dw 2.0
import diff from dw::util::Diff
output application/json
---
{
  missing: diff({a: 1}, {b: 1}),
  arraySize: diff([1,2,3], []),
  arraySizeBack: diff([], [1,2,3]),
  nested: diff([{a: 1}], [{a: 2}]),
  scalar: diff(1, 2),
  string: diff("test", "other test"),
  equal: diff({a: 1}, {a: 1}),
  equalArray: diff([1,2,3], [1,2,3]),
  unordered: diff({a:1, b:2}, {b:2, a:1}, {unordered: true})
}"#;
    let expected = r#"{
  "missing": {
    "matches": false,
    "diffs": [
      {
        "expected": "Entry (root).b with type Number",
        "actual": "was not present in object.",
        "path": "(root).b"
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
        "expected": "2",
        "actual": "1",
        "path": "(root)[0].a"
      }
    ]
  },
  "scalar": {
    "matches": false,
    "diffs": [
      {
        "expected": "2",
        "actual": "1",
        "path": "(root)"
      }
    ]
  },
  "string": {
    "matches": false,
    "diffs": [
      {
        "expected": "\"other test\"",
        "actual": "\"test\"",
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
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn logging_and_named_operators() {
    let script = r#"{
  debug: logDebug("value", 1),
  info: logInfo("value", 2),
  warn: logWarn("value", 3),
  error: logError("value", 4),
  with: logWith("INFO", "value", 5),
  plus: plusPlus([1,2], [3,4]),
  minus: minusMinus([1,2,1,3], [1])
}"#;
    let expected = r#"{
  "debug": 1,
  "info": 2,
  "warn": 3,
  "error": 4,
  "with": 5,
  "plus": [
    1,
    2,
    3,
    4
  ],
  "minus": [
    2,
    3
  ]
}"#;
    assert_eq!(run(script), expected);
}
