// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Numbers module.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"[2.50, 2 + 1.0, 3 * 30 * 0.5, 0.2 + 0.3, 1 / 7, 12 / 5, 8 - 9.75, 987654321098765432109876543210 + 2]"####,
        r####"[
  2.5,
  3,
  45,
  0.5,
  0.1428571428571428571428571428571429,
  2.4,
  -1.75,
  987654321098765432109876543212
]"####,
    ),
    (
        r####"{ s: 2.50 as String, n: "009.50" as Number, c: "b" ++ 2 }"####,
        r####"{
  "s": "2.5",
  "n": 9.5,
  "c": "b2"
}"####,
    ),
    (r####"abs(-2.5)"####, r####"2.5"####),
    (
        r####"{ c: ceil(2.1), f: floor(2.9), r: round(2.5), n: round(-2.5), p: pow(3, 4), s: sqrt(16) }"####,
        r####"{
  "c": 3,
  "f": 2,
  "r": 3,
  "n": -3,
  "p": 81,
  "s": 4
}"####,
    ),
    (
        r####"{ o: isOdd(5), e: isEven(-6), i: isInteger(2.0), d: isDecimal(2.5) }"####,
        r####"{
  "o": true,
  "e": true,
  "i": true,
  "d": true
}"####,
    ),
    (r####"max([2, 7, 4])"####, r####"7"####),
    (r####"min([4, 2, 9])"####, r####"2"####),
    (r####"avg([2, 4, 9])"####, r####"5"####),
    (
        r####"%dw 2.0
import * from dw::core::Numbers
output application/json
---
{ hex: [toHex(300), toHex(-300), toHex(0), toHex(2.0)], bin: [toBinary(6), toBinary(-6)], fromHex: [fromHex("AC"), fromHex("ac"), fromHex("-ac")], fromBin: fromBinary("110"), radix: [fromRadixNumber("q", 36), fromRadixNumber("11", 2), toRadixNumber(26, 36), toRadixNumber(-300, 16)], nulls: [toHex(null), fromHex(null), toBinary(null), fromBinary(null)] }"####,
        r####"{
  "hex": [
    "12c",
    "-12c",
    "0",
    "2"
  ],
  "bin": [
    "110",
    "-110"
  ],
  "fromHex": [
    172,
    172,
    -172
  ],
  "fromBin": 6,
  "radix": [
    26,
    3,
    "q",
    "-12c"
  ],
  "nulls": [
    null,
    null,
    null,
    null
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
