// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the `dw::Runtime` error-handling subset.

fn run(body: &str) -> String {
    let script =
        format!("%dw 2.0\nimport * from dw::Runtime\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(body: &str) -> Result<String, rustweave_core::DwError> {
    let script =
        format!("%dw 2.0\nimport * from dw::Runtime\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[])
}

#[test]
fn fail_raises_a_user_error() {
    assert!(run_result("fail(\"halt\")").is_err());
    assert!(run_result("fail()").is_err());
}

#[test]
fn fail_if_checks_the_predicate() {
    assert_eq!(run("failIf(4, (n) -> n > 9)"), "4");
    assert_eq!(run("failIf(4, (n) -> n > 9) + 6"), "10");
    assert!(run_result("failIf(12, (n) -> n > 9)").is_err());
}

#[test]
fn try_captures_success() {
    assert_eq!(
        run("try(() -> 3 * 4)"),
        "{\n  \"success\": true,\n  \"result\": 12\n}"
    );
    assert_eq!(run("try(() -> 3).success"), "true");
    assert_eq!(run("try(() -> 3).result"), "3");
    assert_eq!(run("try(() -> 3).error"), "null");
}

#[test]
fn try_captures_user_and_runtime_errors() {
    assert_eq!(run("try(() -> fail(\"halt\")).success"), "false");
    assert_eq!(
        run("try(() -> fail(\"halt\")).error.kind"),
        "\"UserException\""
    );
    assert_eq!(run("try(() -> fail(\"halt\")).error.message"), "\"halt\"");
    assert_eq!(run("try(() -> 1 / 0).success"), "false");
    assert_eq!(
        run("try(() -> 1 / 0).error.kind"),
        "\"DivisionByZeroException\""
    );
}

#[test]
fn try_nests_and_rejects_non_functions() {
    assert_eq!(
        run("try(() -> try(() -> fail(\"halt\"))).result.success"),
        "false"
    );
    assert!(run_result("try(7)").is_err());
}

#[test]
fn or_else_returns_the_primary_or_fallback() {
    assert_eq!(run("orElse(try(() -> 3), () -> 8)"), "3");
    assert_eq!(run("orElse(try(() -> fail(\"halt\")), () -> 8)"), "8");
    assert!(run_result("orElse(try(() -> fail(\"halt\")), () -> fail(\"again\"))").is_err());
}

#[test]
fn or_else_try_wraps_fallback_outcomes() {
    assert_eq!(
        run("orElseTry(try(() -> 3), () -> 8)"),
        "{\n  \"success\": true,\n  \"result\": 3\n}"
    );
    assert_eq!(
        run("orElseTry(try(() -> fail(\"halt\")), () -> 8)"),
        "{\n  \"success\": true,\n  \"result\": 8\n}"
    );
    assert_eq!(
        run("orElseTry(try(() -> fail(\"halt\")), () -> fail(\"again\")).success"),
        "false"
    );
    assert_eq!(
        run("orElseTry(try(() -> fail(\"halt\")), () -> fail(\"again\")).error.kind"),
        "\"UserException\""
    );
    assert_eq!(
        run("orElseTry(try(() -> fail(\"halt\")), () -> fail(\"again\")).error.message"),
        "\"again\""
    );
}
