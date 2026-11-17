// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for mixed language and library behaviour.

mod common;

const CASES: &[(&str, &str)] = &[
    (r####"do { var x = 2 --- x * 3 }"####, r####"6"####),
    (r####"do { var x = 2 --- x - 1 }"####, r####"1"####),
    (r####"do { var x = 2 --- x / 2 }"####, r####"1"####),
    (
        r####"do { var x = 2 var y = 3 --- x + y }"####,
        r####"5"####,
    ),
    (
        r####"do { var x = 2 --- do { var y = 3 --- x + y } }"####,
        r####"5"####,
    ),
    (
        r####"1 match { case 1 -> "one" case 2 -> "two" else -> "other" }"####,
        r####""one""####,
    ),
    (
        r####"3 match { case 1 -> "one" case 2 -> "two" else -> "other" }"####,
        r####""other""####,
    ),
    (
        r####"1 match { case 1 -> "one" else -> "other" }"####,
        r####""one""####,
    ),
    (r####"[1,2] match { case [a ~ b] -> a }"####, r####"1"####),
    (
        r####"do { var x = 1 --- if (x > 0) x else 0 }"####,
        r####"1"####,
    ),
    (
        r####"do { var x = 1 --- if (x > 0) {a: x} else {a: 0} }"####,
        r####"{
  "a": 1
}"####,
    ),
    (
        r####"do { var f = (x) -> x + 1 --- f(2) }"####,
        r####"3"####,
    ),
    (
        r####"do { var x = 1 --- x match { case 1 -> "one" else -> "other" } }"####,
        r####""one""####,
    ),
    (
        r####"{a:1} ++ {a:2}"####,
        r####"{
  "a": 1,
  "a": 2
}"####,
    ),
    (
        r####"{a:1, b:2} ++ {a:3, c:4}"####,
        r####"{
  "a": 1,
  "b": 2,
  "a": 3,
  "c": 4
}"####,
    ),
    (
        r####"{a:1, a:2}"####,
        r####"{
  "a": 1,
  "a": 2
}"####,
    ),
    (r####""10" - 3"####, r####"7"####),
    (r####""10" - "3""####, r####"7"####),
    (r####"10 - "3""####, r####"7"####),
    (
        r####"avg([1,2,4])"####,
        r####"2.333333333333333333333333333333333"####,
    ),
    (
        r####"1 / 7"####,
        r####"0.1428571428571428571428571428571429"####,
    ),
    (
        r####"2 / 3"####,
        r####"0.6666666666666666666666666666666667"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
distinctBy([1,1.0,2], (x, i) -> x)"####,
        r####"[
  1,
  2
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
orderBy(["b","a"], (x, i) -> x)"####,
        r####"[
  "a",
  "b"
]"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
substring("abcdef",2,4)"####,
        r####""cd""####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
leftPad("a",3,"0")"####,
        r####""00a""####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Objects
output application/json
---
mergeWith({a:1},{a:2})"####,
        r####"{
  "a": 2
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Numbers
output application/json
---
toHex(10)"####,
        r####""a""####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Numbers
output application/json
---
fromHex("0a")"####,
        r####"10"####,
    ),
    (
        r####""a1b2" splitBy /[0-9]/"####,
        r####"[
  "a",
  "b"
]"####,
    ),
    (
        r####""a1b2" find /[0-9]/"####,
        r####"[
  [
    1
  ],
  [
    3
  ]
]"####,
    ),
    (
        r####""a1b2" replace /[0-9]/ with "X""####,
        r####""aXbX""####,
    ),
    (r####""abc" matches /b/"####, r####"false"####),
    (r####""abc" contains /b/"####, r####"true"####),
    (r####"{a:1}.a?"####, r####"true"####),
    (r####"{a:1}.b?"####, r####"false"####),
    (r####"[1][0]?"####, r####"true"####),
    (r####"[1][5]?"####, r####"false"####),
    (r####""a$(1+1)b""####, r####""a2b""####),
    (r####""$(1+2)""####, r####""3""####),
    (
        r####"1 to 4"####,
        r####"[
  1,
  2,
  3,
  4
]"####,
    ),
    (
        r####"[10,20,30,40][1 to 3]"####,
        r####"[
  20,
  30,
  40
]"####,
    ),
    (
        r####"{ a: 1, b: null }"####,
        r####"{
  "a": 1,
  "b": null
}"####,
    ),
    (
        r####"{ a: [1,2], b: {} }"####,
        r####"{
  "a": [
    1,
    2
  ],
  "b": {
    
  }
}"####,
    ),
    (r####""a\"b""####, r####""a\"b""####),
    (r####"0.000"####, r####"0"####),
];

const ERROR_CASES: &[&str] = &[
    r####"do { var x = 1 --- if (x > 0) x }"####,
    r####"do { --- if (true) 1 else 2 }"####,
    r####"do { var x = 1 --- x + do { --- 2 } }"####,
    r####"{a:1} match { case {a: x} -> x else -> 0 }"####,
    r####"[1,2] match { case [a, b] -> a + b else -> 0 }"####,
    r####""x" match { case y -> y else -> "other" }"####,
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
    for script in ERROR_CASES {
        assert!(
            common::run_result(script).is_err(),
            "expected error: {script}"
        );
    }
}
