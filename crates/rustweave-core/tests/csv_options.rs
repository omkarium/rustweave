// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for CSV reader and writer options.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn reader_names_columns_without_a_header() {
    assert_eq!(
        run(r#"%dw 2.0
output application/json
---
read("10,20\n30,40", "application/csv", {header: false})"#),
        "[\n  {\n    \"column_0\": \"10\",\n    \"column_1\": \"20\"\n  },\n  {\n    \"column_0\": \"30\",\n    \"column_1\": \"40\"\n  }\n]"
    );
}

#[test]
fn reader_takes_the_header_from_its_line_number() {
    assert_eq!(
        run(r#"%dw 2.0
output application/json
---
read("skip this\ncode,qty\nK-1,4", "application/csv", {headerLineNumber: 2})"#),
        "[\n  {\n    \"code\": \"K-1\",\n    \"qty\": \"4\"\n  }\n]"
    );
}

#[test]
fn writer_escapes_separators_instead_of_quoting() {
    assert_eq!(
        run(r#"%dw 2.0
output application/csv escape="\\"
---
[{note: "left,right"}]"#),
        "note\nleft\\,right\n"
    );
    assert_eq!(
        run(r#"%dw 2.0
output application/csv quoteValues=false
---
[{note: "left,right"}]"#),
        "note\nleft\\,right\n"
    );
}

#[test]
fn writer_emits_an_empty_line_for_an_empty_object() {
    assert_eq!(
        run("%dw 2.0\noutput application/csv header=false\n---\n[{a: 1}, {}, {a: 3}]"),
        "1\n\n3\n"
    );
}
