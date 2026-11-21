// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for JSON writer options.

#[test]
fn duplicate_keys_are_grouped_with_reference_indentation() {
    let output = rustweave_core::run(
        "%dw 2.0\noutput application/json duplicateKeyAsArray=true\n---\n{a: 1, a: 2}",
        &[],
    )
    .unwrap();
    assert_eq!(output, "{\n  \"a\": [\n  1,\n  2\n  ]\n}");
}
