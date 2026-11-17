// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Numbers module.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"[1.50, 1 + 1.0, 2 * 40 * 0.9, 0.1 + 0.2, 1 / 3, 10 / 4, 5 - 7.25, 123456789012345678901234567890 + 1]"####,
        r####"[
  1.5,
  2,
  72,
  0.3,
  0.3333333333333333333333333333333333,
  2.5,
  -2.25,
  123456789012345678901234567891
]"####,
    ),
    (
        r####"{ s: 1.50 as String, n: "007.50" as Number, c: "a" ++ 1 }"####,
        r####"{
  "s": "1.5",
  "n": 7.5,
  "c": "a1"
}"####,
    ),
    (r####"abs(-1.5)"####, r####"1.5"####),
    (
        r####"{ c: ceil(1.2), f: floor(1.8), r: round(1.5), n: round(-1.5), p: pow(2, 3), s: sqrt(9) }"####,
        r####"{
  "c": 2,
  "f": 1,
  "r": 2,
  "n": -2,
  "p": 8,
  "s": 3
}"####,
    ),
    (
        r####"{ o: isOdd(3), e: isEven(-4), i: isInteger(1.0), d: isDecimal(1.5) }"####,
        r####"{
  "o": true,
  "e": true,
  "i": true,
  "d": true
}"####,
    ),
    (r####"max([1, 3, 2])"####, r####"3"####),
    (r####"min([3, 1, 2])"####, r####"1"####),
    (r####"avg([1, 2, 3])"####, r####"2"####),
    (
        r####"%dw 2.0
import * from dw::core::Numbers
output application/json
---
     { hex: [toHex(255), toHex(-255), toHex(0), toHex(1.0)],        bin: [toBinary(5), toBinary(-5)],        fromHex: [fromHex("FF"), fromHex("ff"), fromHex("-ff")],        fromBin: fromBinary("101"),        radix: [fromRadixNumber("z", 36), fromRadixNumber("10", 2),                toRadixNumber(35, 36), toRadixNumber(-255, 16)],        nulls: [toHex(null), fromHex(null), toBinary(null), fromBinary(null)] }"####,
        r####"{
  "hex": [
    "ff",
    "-ff",
    "0",
    "1"
  ],
  "bin": [
    "101",
    "-101"
  ],
  "fromHex": [
    255,
    255,
    -255
  ],
  "fromBin": 5,
  "radix": [
    35,
    2,
    "z",
    "-ff"
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
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
