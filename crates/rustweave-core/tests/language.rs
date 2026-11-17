fn run(body: &str) -> String {
    let script =
        format!("%dw 2.0\nimport * from dw::core::Arrays\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[]).unwrap_or_else(|e| panic!("{e}"))
}

fn run_result(body: &str) -> Result<String, rustweave_core::DwError> {
    let script =
        format!("%dw 2.0\nimport * from dw::core::Arrays\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[])
}

#[test]
fn tight_operators_bind_inside_the_placeholder_lambda() {
    assert_eq!(run("[1,2,3] map $ * 2"), "[\n  2,\n  4,\n  6\n]");
    assert_eq!(run("[1,2,3] map $ + 1"), "[\n  2,\n  3,\n  4\n]");
    assert_eq!(run("[1,2,3] filter $ > 1"), "[\n  2,\n  3\n]");
    assert_eq!(run("[1,2,3] countBy $ > 1"), "2");
    assert_eq!(run("[1,2,3] reduce $ + $$"), "6");
    assert_eq!(
        run("[1,2,3] map $ == 1"),
        "[\n  true,\n  false,\n  false\n]"
    );
    assert_eq!(run("[true,false] map $ and true"), "[\n  true,\n  false\n]");
    assert_eq!(run("[true,false] map $ or false"), "[\n  true,\n  false\n]");
    assert_eq!(run("[\"1\",\"2\"] map $ as Number"), "[\n  1,\n  2\n]");
}

#[test]
fn loose_operators_apply_to_the_call_result() {
    assert_eq!(run("[1,2,3] map $ ++ [4]"), "[\n  1,\n  2,\n  3,\n  4\n]");
    assert_eq!(
        run("[1,2,3] map $ + 1 ++ [3]"),
        "[\n  2,\n  3,\n  4,\n  3\n]"
    );
    assert_eq!(
        run(r#"[["a"],["b"]] map $ ++ ["x"]"#),
        "[\n  [\n    \"a\"\n  ],\n  [\n    \"b\"\n  ],\n  \"x\"\n]"
    );
    assert_eq!(run("[null,1] map $ default 0"), "[\n  null,\n  1\n]");
    assert_eq!(run("[null,1] map ($ default 0)"), "[\n  0,\n  1\n]");
    assert!(run_result("[1,2,3] map $ ++ \"x\"").is_err());
    assert!(run_result("[\"a\",\"b\",\"c\"] reduce $ ++ $$").is_err());
}

#[test]
fn arithmetic_order_is_preserved_inside_the_lambda() {
    assert_eq!(run("[1,2,3] map $ * 2 + 1"), "[\n  3,\n  5,\n  7\n]");
    assert_eq!(run("[1,2,3] map $ + 1 * 2"), "[\n  3,\n  4,\n  5\n]");
}

#[test]
fn parenthesized_placeholders_still_work() {
    assert_eq!(run("[1,2,3] map ($ * 2)"), "[\n  2,\n  4,\n  6\n]");
    assert_eq!(run("[1,2,3] filter ($ > 1)"), "[\n  2,\n  3\n]");
    assert_eq!(run("[1,2,3] countBy ($ > 1)"), "2");
    assert_eq!(run("[1,2,3] reduce ($ + $$)"), "6");
    assert_eq!(run("[{a:1},{a:2}] map $.a"), "[\n  1,\n  2\n]");
    assert!(run_result("[1,2,3] map ($ ++ [4])").is_err());
}

#[test]
fn pluck_requires_an_object() {
    assert!(run_result("[{a:1},{a:2}] pluck $.a").is_err());
}
