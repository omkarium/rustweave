// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for `using`, object head-tail spread, and value metadata.

fn run(body: &str) -> String {
    let script = format!("%dw 2.0\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(body: &str) -> Result<String, rustweave_core::DwError> {
    let script = format!("%dw 2.0\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[])
}

#[test]
fn using_binds_names_left_to_right() {
    assert_eq!(run("using (n = 4) n + 1"), "5");
    assert_eq!(
        run("using (a = 2, b = a * 3, c = b - a) [a, b, c]"),
        "[\n  2,\n  6,\n  4\n]"
    );
    assert_eq!(
        run("using (word = \"oak\") {label: word ++ \" tree\"}"),
        "{\n  \"label\": \"oak tree\"\n}"
    );
}

#[test]
fn using_shadows_and_nests() {
    let script = r####"%dw 2.0
output application/json
var n = 100
---
using (n = 3) (using (n = 8) n) + n"####;
    assert_eq!(rustweave_core::run(script, &[]).unwrap(), "11");
    assert_eq!(run_result("using (n = 3) missing").is_err(), true);
    assert_eq!(run_result("using (a = b, b = 2) a").is_err(), true);
    assert_eq!(run_result("using (a = 1, a = 2) a").is_err(), true);
    assert_eq!(run_result("using () 1").is_err(), true);
}

#[test]
fn using_works_in_headers_arguments_and_collections() {
    let script = r####"%dw 2.0
output application/json
var computed = using (base = 6) base * 2
---
{
  computed: computed,
  mapped: using (step = 10) [1, 2] map $ + step,
  nested: using (x = 1) [using (y = x + 1) x + y],
  fallback: using (x = null) x default "none"
}"####;
    let expected = r####"{
  "computed": 12,
  "mapped": [
    11,
    12
  ],
  "nested": [
    3
  ],
  "fallback": "none"
}"####;
    assert_eq!(rustweave_core::run(script, &[]).unwrap(), expected);
}

#[test]
fn object_spread_emits_head_then_tail() {
    assert_eq!(
        run("{ id: 7 ~ {name: \"kit\", active: true} }"),
        "{\n  \"id\": 7,\n  \"name\": \"kit\",\n  \"active\": true\n}"
    );
    let with_var = r####"%dw 2.0
output application/json
var extra = {b: 2}
---
{a: 1 ~ extra}"####;
    assert_eq!(
        rustweave_core::run(with_var, &[]).unwrap(),
        "{\n  \"a\": 1,\n  \"b\": 2\n}"
    );
    let dynamic = r####"%dw 2.0
output application/json
var field = "code"
---
{ (field): "Z" ~ {size: 3} }"####;
    assert_eq!(
        rustweave_core::run(dynamic, &[]).unwrap(),
        "{\n  \"code\": \"Z\",\n  \"size\": 3\n}"
    );
    assert_eq!(
        run("{a: 1 ~ {a: 9}}"),
        "{\n  \"a\": 1,\n  \"a\": 9\n}"
    );
}

#[test]
fn object_spread_rejects_bad_tails_and_positions() {
    assert!(run_result("{a: 1 ~ null}").is_err());
    assert!(run_result("{a: 1 ~ [2]}").is_err());
    assert!(run_result("{a: 1 ~ {b: 2}, c: 3}").is_err());
    assert!(run_result("{a: 1, b: 2 ~ {c: 3}}").is_err());
}

#[test]
fn metadata_attaches_without_changing_the_value() {
    let script = r####"%dw 2.0
output application/json
var label = "cedar" as String {group: "tree", rank: 2}
---
{value: label, group: label.^group, rank: label.^rank, absent: label.^missing}"####;
    let expected = r####"{
  "value": "cedar",
  "group": "tree",
  "rank": 2,
  "absent": null
}"####;
    assert_eq!(rustweave_core::run(script, &[]).unwrap(), expected);
}

#[test]
fn metadata_all_and_quoted_keys() {
    assert_eq!(
        run("keysOf((3 as Number {first: 1, second: 2}).^)"),
        "[\n  \"first\",\n  \"second\"\n]"
    );
    assert_eq!(run("sizeOf((3).^) == 0"), "true");
    assert_eq!(run("(3 as Number {\"custom-key\": 9}).^\"custom-key\""), "9");
}

#[test]
fn metadata_is_not_kept_by_computed_operations() {
    assert_eq!(run("((2 as Number {tag: 1}) + 3).^tag"), "null");
    assert_eq!(run("(\"ab\" as String {tag: 1} ++ \"c\").^tag"), "null");
    assert_eq!(run("({a: 1} as Object {tag: 1}).a"), "1");
}
