// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn identity_pipe_and_compose() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
var steps = [(n) -> n + 1, (n) -> n * n]
output application/json
---
{
  same: identity({a: 1}),
  none: identity(null),
  piped: pipe([(n) -> n + 2, (n) -> n * 3])(4),
  composed: compose([(n) -> n + 2, (n) -> n * 3])(4),
  emptyPipe: pipe([])(5),
  emptyCompose: compose([])(5),
  variablePipe: pipe(steps)(3),
  nested: pipe([identity, (n) -> n + 1])(3)
}"####;
    let expected = r####"{
  "same": {
    "a": 1
  },
  "none": null,
  "piped": 18,
  "composed": 14,
  "emptyPipe": 5,
  "emptyCompose": 5,
  "variablePipe": 16,
  "nested": 4
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn functional_argument_errors() {
    for script in [
        r####"pipe(1)"####,
        r####"pipe([1, (n) -> n])"####,
        r####"pipe(null)"####,
        r####"compose(1)"####,
        r####"compose([1, (n) -> n])"####,
        r####"compose(null)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}

#[test]
fn url_compose_is_not_shadowed() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
import * from dw::core::URL
output application/json
---
compose(["https://sample.test/", "/page.html"], ["content folder"])"####;
    assert_eq!(
        run(script),
        r####""https://sample.test/content%20folder/page.html""####
    );
}
