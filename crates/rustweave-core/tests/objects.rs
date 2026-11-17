//! Behaviour tests for the Objects module.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"{ a: 1, b: 2 } pluck $$"####,
        r####"[
  "a",
  "b"
]"####,
    ),
    (
        r####"{ a: 1, b: 2 } mapObject { ($$): $ * 2 }"####,
        r####"{
  "a": 2,
  "b": 4
}"####,
    ),
    (r####"sizeOf({ a: 1, b: 2 })"####, r####"2"####),
    (
        r####"entriesOf({ a: 1 })"####,
        r####"[
  {
    "key": "a",
    "value": 1,
    "attributes": {
      
    }
  }
]"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Objects
output application/json
---
     { keys: keySet({ b: 1, a: 2 }), every: { a: 1, b: 2 } everyEntry ($ > 0),        some: { a: 1, b: 3 } someEntry ($ == 3),        remove: { a: 1, b: 2, c: 3 } -- ["b"],        merge: mergeWith({ a: 1, b: 2 }, { b: 3, c: 4 }),        div: divideBy({ a: 1, b: 2, c: 3 }, 2),        take: { a: 1, b: 2, c: 3 } takeWhile $ < 3 }"####,
        r####"{
  "keys": [
    "b",
    "a"
  ],
  "every": true,
  "some": true,
  "remove": {
    "a": 1,
    "c": 3
  },
  "merge": {
    "a": 1,
    "b": 3,
    "c": 4
  },
  "div": [
    {
      "a": 1,
      "b": 2
    },
    {
      "c": 3
    }
  ],
  "take": {
    "a": 1,
    "b": 2
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Objects
output application/json
---
{ es: entrySet({a:1}), names: nameSet({a:1}), vals: valueSet({a:1}), ee: ({a:1,b:2} everyEntry ($ > 0)), se: ({a:1,b:2} someEntry ($ == 2)), mw: mergeWith({a:1,b:2},{b:3,c:4}), div: divideBy({a:1,b:2,c:3},2), tw: ({a:1,b:2,c:3} takeWhile ($ < 3)) }"####,
        r####"{
  "es": [
    {
      "key": "a",
      "value": 1,
      "attributes": {
        
      }
    }
  ],
  "names": [
    "a"
  ],
  "vals": [
    1
  ],
  "ee": true,
  "se": true,
  "mw": {
    "a": 1,
    "b": 3,
    "c": 4
  },
  "div": [
    {
      "a": 1,
      "b": 2
    },
    {
      "c": 3
    }
  ],
  "tw": {
    "a": 1,
    "b": 2
  }
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
