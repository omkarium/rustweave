//! Behaviour tests for the language surface.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
output application/json
var rate = 0.1
     fun tax(n: Number) = n * rate
fun tax(s: String) = "n/a"
---
     payload.orders map { id: $.id, tax: tax($.total), label: tax($.customer),        big: if ($.total > 100) "yes" else "no", missing: $.coupon default "none" }"####,
        r####"[
  {
    "id": 1,
    "tax": 12.05,
    "label": "n/a",
    "big": "yes",
    "missing": "none"
  },
  {
    "id": 2,
    "tax": 8,
    "label": "n/a",
    "big": "no",
    "missing": "none"
  },
  {
    "id": 3,
    "tax": 1.525,
    "label": "n/a",
    "big": "no",
    "missing": "none"
  },
  {
    "id": 4,
    "tax": 30.01,
    "label": "n/a",
    "big": "yes",
    "missing": "none"
  }
]"####,
    ),
    (
        r####"[payload.orders[-1].id, payload.orders[10], { a: 1, b: 2 } == { b: 2, a: 1 }, 1 == 1.0, 1 == "1"]"####,
        r####"[
  4,
  null,
  false,
  true,
  false
]"####,
    ),
    (
        r####"1 to 3"####,
        r####"[
  1,
  2,
  3
]"####,
    ),
    (
        r####"[10, 20, 30, 40][1 to 2]"####,
        r####"[
  20,
  30
]"####,
    ),
    (r####"[10, 20, 30, 40][2 to 10]"####, r####"null"####),
    (r####""abcd"[1 to 2]"####, r####""bc""####),
    (r####"do { var x = 2 --- x + 1 }"####, r####"3"####),
    (
        r####"[1, 2, 3] match { case [] -> 0 case [h ~ t] -> h }"####,
        r####"1"####,
    ),
    (r####"1 is Number"####, r####"true"####),
    (r####""a" is Number"####, r####"false"####),
    (
        r####"{ ("a" ++ "b"): 1 }"####,
        r####"{
  "ab": 1
}"####,
    ),
    (
        r####"{ d: [{a: 1}.a?, {a: 1}.b?, {a: null}.a?, [1][0]?, [1][5]?, null.a?] }"####,
        r####"{
  "d": [
    true,
    false,
    true,
    true,
    false,
    false
  ]
}"####,
    ),
    (r####""a$(1)b $(true)""####, r####""a1b true""####),
    (
        r####"["x"] map "v=$( $ )""####,
        r####"[
  "v=x"
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1, 2, 3,]"####,
        r####"[
  1,
  2,
  3
]"####,
    ),
    (r####"do { var x: Number = 2 --- x + 1 }"####, r####"3"####),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
