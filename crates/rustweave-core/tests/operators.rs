//! Behaviour tests for operators.

mod common;

const CASES: &[(&str, &str)] = &[
    (r####"1 ++ 2"####, r####""12""####),
    (r####"true ++ false"####, r####""truefalse""####),
    (r####"1 ++ true"####, r####""1true""####),
    (r####"true ++ 1"####, r####""true1""####),
    (r####"1.5 ++ 2"####, r####""1.52""####),
    (r####"1 ++ 1.50"####, r####""11.5""####),
    (r####""a" ++ 1"####, r####""a1""####),
    (r####"1 ++ "a""####, r####""1a""####),
    (r####""a" ++ true"####, r####""atrue""####),
    (r####"true ++ "a""####, r####""truea""####),
    (
        r####"[1] ++ [2]"####,
        r####"[
  1,
  2
]"####,
    ),
    (
        r####"[] ++ []"####,
        r####"[
  
]"####,
    ),
    (
        r####"{a:1} ++ {b:2}"####,
        r####"{
  "a": 1,
  "b": 2
}"####,
    ),
    (
        r####"{} ++ {}"####,
        r####"{
  
}"####,
    ),
    (r####"1 + 2"####, r####"3"####),
    (r####"1.5 + 2"####, r####"3.5"####),
    (r####""1" + 2"####, r####"3"####),
    (r####"1 + "2""####, r####"3"####),
    (
        r####"[1] + 2"####,
        r####"[
  1,
  2
]"####,
    ),
    (
        r####"[1] + [2]"####,
        r####"[
  1,
  [
    2
  ]
]"####,
    ),
    (
        r####"[1] + null"####,
        r####"[
  1,
  null
]"####,
    ),
    (r####"3 - 1"####, r####"2"####),
    (r####"1 - 3"####, r####"-2"####),
    (
        r####"[1,2] - 1"####,
        r####"[
  2
]"####,
    ),
    (
        r####"[1,2] - [1]"####,
        r####"[
  1,
  2
]"####,
    ),
    (r####"3 * 2"####, r####"6"####),
    (r####""3" * 2"####, r####"6"####),
    (r####"3 / 2"####, r####"1.5"####),
    (r####""3" / 2"####, r####"1.5"####),
    (r####"1 < 2"####, r####"true"####),
    (r####""a" < "b""####, r####"true"####),
    (r####"1 < "2""####, r####"true"####),
    (r####"true < false"####, r####"false"####),
    (r####"1 == 1.0"####, r####"true"####),
    (r####"1 == "1""####, r####"false"####),
    (r####"true == 1"####, r####"false"####),
    (r####"null == null"####, r####"true"####),
    (r####"[1] == [1]"####, r####"true"####),
    (r####"{a:1} == {a:1}"####, r####"true"####),
    (r####"[1] == [1.0]"####, r####"true"####),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ * 2"####,
        r####"[
  2,
  4,
  6
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ + 1"####,
        r####"[
  2,
  3,
  4
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] filter $ > 1"####,
        r####"[
  2,
  3
]"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
[1,2,3] countBy $ > 1"####,
        r####"2"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] reduce $ + $$"####,
        r####"6"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ == 1"####,
        r####"[
  true,
  false,
  false
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[true,false] map $ and true"####,
        r####"[
  true,
  false
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[true,false] map $ or false"####,
        r####"[
  true,
  false
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
["1","2"] map $ as Number"####,
        r####"[
  1,
  2
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ ++ [4]"####,
        r####"[
  1,
  2,
  3,
  4
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ + 1 ++ [3]"####,
        r####"[
  2,
  3,
  4,
  3
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[["a"],["b"]] map $ ++ ["x"]"####,
        r####"[
  [
    "a"
  ],
  [
    "b"
  ],
  "x"
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[null,1] map $ default 0"####,
        r####"[
  null,
  1
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[null,1] map ($ default 0)"####,
        r####"[
  0,
  1
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ * 2 + 1"####,
        r####"[
  3,
  5,
  7
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map $ + 1 * 2"####,
        r####"[
  3,
  4,
  5
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] map ($ * 2)"####,
        r####"[
  2,
  4,
  6
]"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] filter ($ > 1)"####,
        r####"[
  2,
  3
]"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
[1,2,3] countBy ($ > 1)"####,
        r####"2"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[1,2,3] reduce ($ + $$)"####,
        r####"6"####,
    ),
    (
        r####"%dw 2.0
output application/json
---
[{a:1},{a:2}] map $.a"####,
        r####"[
  1,
  2
]"####,
    ),
];

const ERROR_CASES: &[&str] = &[
    r####""a" ++ null"####,
    r####"null ++ "a""####,
    r####"1 ++ null"####,
    r####"null ++ 1"####,
    r####"true ++ null"####,
    r####"null ++ true"####,
    r####"null ++ null"####,
    r####"[1] ++ "x""####,
    r####""x" ++ [1]"####,
    r####"{a:1} ++ "x""####,
    r####""x" ++ {a:1}"####,
    r####"[1] ++ 2"####,
    r####"2 ++ [1]"####,
    r####"{a:1} ++ 2"####,
    r####"2 ++ {a:1}"####,
    r####"[1] ++ {a:1}"####,
    r####"{a:1} ++ [1]"####,
    r####"[1] ++ null"####,
    r####"null ++ [1]"####,
    r####"{a:1} ++ null"####,
    r####"null ++ {a:1}"####,
    r####""a" + "b""####,
    r####"true + 1"####,
    r####"1 + true"####,
    r####"2 + [1]"####,
    r####"{a:1} + {b:2}"####,
    r####"{a:1} + 1"####,
    r####"1 + {a:1}"####,
    r####"null + 1"####,
    r####"1 + null"####,
    r####"null + null"####,
    r####""a" + null"####,
    r####"{a:1} + null"####,
    r####""a" - "b""####,
    r####"{a:1} - {a:1}"####,
    r####"null - 1"####,
    r####"[1] * 2"####,
    r####"true * 2"####,
    r####"null * 2"####,
    r####"1 / 0"####,
    r####"[1] / 2"####,
    r####"null / 2"####,
    r####"[1] < [2]"####,
    r####"null < 1"####,
    r####"null < null"####,
    r####"%dw 2.0
output application/json
---
[1,2,3] map $ ++ "x""####,
    r####"%dw 2.0
output application/json
---
["a","b","c"] reduce $ ++ $$"####,
    r####"%dw 2.0
output application/json
---
[1,2,3] map ($ ++ [4])"####,
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
