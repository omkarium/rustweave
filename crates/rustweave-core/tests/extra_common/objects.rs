// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn from_entries_and_assign() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  mixed: fromEntries([{key: "left", value: 1}, ["right", 2], {key: "left", value: 3}]),
  empty: fromEntries([]),
  assign: assign({a: 1, b: 2}, {b: 9, c: 3}),
  emptyAssign: assign({}, {}),
  duplicateTarget: assign({x: 1, x: 2}, {x: 9})
}"####;
    let expected = r####"{
  "mixed": {
    "left": 3,
    "right": 2
  },
  "empty": {
    
  },
  "assign": {
    "a": 1,
    "b": 9,
    "c": 3
  },
  "emptyAssign": {
    
  },
  "duplicateTarget": {
    "x": 9
  }
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn pick_omit_has_own() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  pick: pick({a: 1, b: 2, c: 3, d: 4}, ["d", "b", "b", "missing"]),
  pickEmpty: pick({a: 1}, []),
  omit: omit({a: 1, b: 2, c: 3, d: 4}, ["b", "missing"]),
  omitEmpty: omit({a: 1}, []),
  hasNull: hasOwn({maybe: null, other: 0}, "maybe"),
  hasMissing: hasOwn({maybe: null}, "Maybe"),
  hasEmpty: hasOwn({}, "x")
}"####;
    let expected = r####"{
  "pick": {
    "b": 2,
    "d": 4
  },
  "pickEmpty": {
    
  },
  "omit": {
    "a": 1,
    "c": 3,
    "d": 4
  },
  "omitEmpty": {
    "a": 1
  },
  "hasNull": true,
  "hasMissing": false,
  "hasEmpty": false
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn object_argument_errors() {
    for script in [
        r####"fromEntries(null)"####,
        r####"fromEntries([["only-one"]])"####,
        r####"fromEntries([[1, "value"]])"####,
        r####"fromEntries([{key: "a"}])"####,
        r####"assign(null, {})"####,
        r####"assign({}, null)"####,
        r####"pick(null, [])"####,
        r####"pick({}, [1])"####,
        r####"omit(null, [])"####,
        r####"hasOwn(null, "a")"####,
        r####"hasOwn({}, 1)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}
