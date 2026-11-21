// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn unwrap_and_map_null_handling() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  fallback: unwrapOr(null, "default"),
  keeps: unwrapOr(0, 7),
  keepsFalse: unwrapOr(false, true),
  eager: unwrapOrElse(null, () -> 40 + 2),
  keepsElse: unwrapOrElse("value", () -> "unused"),
  lazy: unwrapOrElse("kept", () -> 1 / 0),
  mappedNull: mapOr(null, 99, (n) -> n * 2),
  mappedValue: mapOr(6, 99, (n) -> n * 2),
  chainNull: andThen(null, (n) -> n + 1),
  lazyAndThen: andThen(null, (n) -> 1 / 0),
  chainValue: andThen(6, (n) -> n + 1),
  elseNull: orElse(null, "replacement"),
  elseValue: orElse("kept", "replacement"),
  nested: unwrapOr(unwrapOr(null, null), "last")
}"####;
    let expected = r####"{
  "fallback": "default",
  "keeps": 0,
  "keepsFalse": false,
  "eager": 42,
  "keepsElse": "value",
  "lazy": "kept",
  "mappedNull": 99,
  "mappedValue": 12,
  "chainNull": null,
  "lazyAndThen": null,
  "chainValue": 7,
  "elseNull": "replacement",
  "elseValue": "kept",
  "nested": "last"
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn null_handling_argument_errors() {
    for script in [
        r####"unwrapOr(1)"####,
        r####"unwrapOrElse(1)"####,
        r####"unwrapOrElse(null, 1)"####,
        r####"unwrapOrElse(null, "x")"####,
        r####"mapOr(1, 2)"####,
        r####"mapOr(1, 2, 3)"####,
        r####"mapOr(null, 1, 3)"####,
        r####"andThen(1)"####,
        r####"andThen(1, 2)"####,
        r####"andThen(null, 2)"####,
        r####"orElse(1)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}
