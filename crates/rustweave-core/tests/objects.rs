// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Objects module.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"{ alpha: 10, beta: 20 } pluck $$"####,
        r####"[
  "alpha",
  "beta"
]"####,
    ),
    (
        r####"{ alpha: 10, beta: 20 } mapObject { ($$): $ + 5 }"####,
        r####"{
  "alpha": 15,
  "beta": 25
}"####,
    ),
    (r####"sizeOf({ x: 1, y: 2, z: 3 })"####, r####"3"####),
    (
        r####"entriesOf({ color: "teal" })"####,
        r####"[
  {
    "key": "color",
    "value": "teal",
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
{ keys: keySet({ z: 8, y: 9 }), every: { x: 4, y: 5 } everyEntry ($ > 0), some: { x: 4, y: 8 } someEntry ($ == 8), remove: { x: 4, y: 5, z: 6 } -- ["y"], merge: mergeWith({ x: 4, y: 5 }, { y: 9, z: 10 }), div: divideBy({ x: 4, y: 5, z: 6 }, 2), take: { x: 4, y: 5, z: 6 } takeWhile $ < 6 }"####,
        r####"{
  "keys": [
    "z",
    "y"
  ],
  "every": true,
  "some": true,
  "remove": {
    "x": 4,
    "z": 6
  },
  "merge": {
    "x": 4,
    "y": 9,
    "z": 10
  },
  "div": [
    {
      "x": 4,
      "y": 5
    },
    {
      "z": 6
    }
  ],
  "take": {
    "x": 4,
    "y": 5
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Objects
output application/json
---
{ es: entrySet({a: 11}), names: nameSet({a: 11}), vals: valueSet({a: 11}), ee: ({a: 11, b: 12} everyEntry ($ > 10)), se: ({a: 11, b: 12} someEntry ($ == 12)), mw: mergeWith({a: 11, b: 12}, {b: 13, c: 14}), div: divideBy({a: 11, b: 12, c: 13}, 2), tw: ({a: 11, b: 12, c: 13} takeWhile ($ < 13)) }"####,
        r####"{
  "es": [
    {
      "key": "a",
      "value": 11,
      "attributes": {
        
      }
    }
  ],
  "names": [
    "a"
  ],
  "vals": [
    11
  ],
  "ee": true,
  "se": true,
  "mw": {
    "a": 11,
    "b": 13,
    "c": 14
  },
  "div": [
    {
      "a": 11,
      "b": 12
    },
    {
      "c": 13
    }
  ],
  "tw": {
    "a": 11,
    "b": 12
  }
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
