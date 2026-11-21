// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for XML writer options.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn writer_adds_a_default_namespace() {
    assert_eq!(
        run(r#"%dw 2.0
output application/xml writeDeclaration=false, defaultNamespace="http://sample.test/ns"
---
{part: 7}"#),
        "<part xmlns=\"http://sample.test/ns\">7</part>"
    );
}

#[test]
fn writer_marks_null_elements_with_xsi_nil() {
    assert_eq!(
        run("%dw 2.0\noutput application/xml writeDeclaration=false, writeNilOnNull=true\n---\n{part: null}"),
        "<part xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:nil=\"true\"/>"
    );
}

#[test]
fn writer_escapes_carriage_returns_on_request() {
    assert_eq!(
        run("%dw 2.0\noutput application/xml writeDeclaration=false, escapeCR=true\n---\n{note: \"a\\rb\"}"),
        "<note>a&#xd;b</note>"
    );
}

#[test]
fn writer_skips_null_elements_on_request() {
    assert_eq!(
        run(r#"%dw 2.0
output application/xml writeDeclaration=false, skipNullOn="elements"
---
{empty: null, part: 7}"#),
        "<part>7</part>"
    );
}
