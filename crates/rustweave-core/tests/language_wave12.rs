// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for selectors, match patterns, and expression operators.

fn run(body: &str) -> String {
    let script = format!("%dw 2.0\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(body: &str) -> Result<String, rustweave_core::DwError> {
    let script = format!("%dw 2.0\noutput application/json\n---\n{body}");
    rustweave_core::run(&script, &[])
}

#[test]
fn multi_value_selection_collects_each_match() {
    assert_eq!(run("{tag: 1, tag: 3}.*tag"), "[\n  1,\n  3\n]");
    assert_eq!(run("[{id: 2}, {id: 5}].*id"), "[\n  2,\n  5\n]");
    assert_eq!(run("{tag: 1}.*missing"), "null");
}

#[test]
fn descendant_selection_walks_nested_values() {
    assert_eq!(
        run("{root: {leaf: 1, branch: {leaf: 2}, twig: [{leaf: 3}]}}.root..leaf"),
        "[\n  1,\n  2,\n  3\n]"
    );
    assert_eq!(run("[{leaf: 1}, {other: 2}]..leaf"), "[\n  1\n]");
    assert_eq!(run("{leaf: 1}..missing"), "null");
    assert!(run_result("null..a").is_err());
}

#[test]
fn key_value_selection_keeps_pairs() {
    assert_eq!(run("{code: 7, size: 9}.&code"), "{\n  \"code\": 7\n}");
    assert_eq!(
        run("[{code: 1}, {code: 2}].&code"),
        "[\n  {\n    \"code\": 1\n  },\n  {\n    \"code\": 2\n  }\n]"
    );
    assert_eq!(run("{code: 7}.&size"), "null");
}

#[test]
fn xml_attribute_selection_reads_and_asserts() {
    assert_eq!(
        run(r#"read("<item ref=\"9\"/>", "application/xml").item.@ref"#),
        "\"9\""
    );
    assert_eq!(
        run(r#"read("<item ref=\"9\" unit=\"kg\"/>", "application/xml").item.@"unit""#),
        "\"kg\""
    );
    assert_eq!(
        run(r#"read("<item ref=\"9\"/>", "application/xml").item.@"missing"?"#),
        "false"
    );
    assert!(
        run_result(r#"read("<item ref=\"9\"/>", "application/xml").item.@"missing"!"#).is_err()
    );
}

#[test]
fn filter_selection_uses_dollar_for_the_element() {
    assert_eq!(
        run("[{qty: 1}, {qty: 4}][?($.qty > 2)]"),
        "[\n  {\n    \"qty\": 4\n  }\n]"
    );
    assert_eq!(run("{low: 1, high: 8}[?($ > 3)]"), "{\n  \"high\": 8\n}");
    assert_eq!(run("[1, 2][?($ > 9)]"), "null");
    assert!(run_result("(3)[?($ > 1)]").is_err());
}

#[test]
fn postfix_assertion_rejects_missing_selections() {
    assert_eq!(run("{value: 1}.value!"), "1");
    assert_eq!(run("{value: null}.value!"), "null");
    assert_eq!(run("[4, 6][1]!"), "6");
    assert!(run_result("{value: 1}.other!").is_err());
    assert!(run_result("[4, 6][9]!").is_err());
}

#[test]
fn match_supports_types_and_named_literals() {
    assert_eq!(
        run("true match { case is Boolean -> \"flag\" else -> \"other\" }"),
        "\"flag\""
    );
    assert_eq!(
        run("\"go\" match { case word is String -> word ++ \"!\" else -> \"no\" }"),
        "\"go!\""
    );
    assert_eq!(
        run("\"go\" match { case word: \"go\" -> word ++ \"!\" case word: \"stop\" -> word }"),
        "\"go!\""
    );
}

#[test]
fn match_supports_guards_and_regex_captures() {
    assert_eq!(
        run("12 match { case n if (n > 9) -> \"large\" else -> \"small\" }"),
        "\"large\""
    );
    assert_eq!(
        run("\"b7\" match { case parts matches /([a-z])([0-9])/ -> parts[1] ++ parts[2] }"),
        "\"b7\""
    );
    assert_eq!(
        run("\"42\" match { case parts matches /[0-9]+/ -> parts }"),
        "[\n  \"42\"\n]"
    );
}

#[test]
fn match_supports_object_deconstruction() {
    assert_eq!(
        run("{alpha: 10, beta: 20} match { case {key: value ~ rest} -> {key: key, value: value, rest: rest} case {} -> \"empty\" }"),
        "{\n  \"key\": \"alpha\",\n  \"value\": 10,\n  \"rest\": {\n    \"beta\": 20\n  }\n}"
    );
    assert_eq!(
        run("{} match { case {key: value ~ rest} -> value case {} -> \"empty\" }"),
        "\"empty\""
    );
}

#[test]
fn match_else_body_can_use_the_subject() {
    assert_eq!(
        run("\"plain\" match { case \"other\" -> \"other\" else -> $ }"),
        "\"plain\""
    );
    assert_eq!(
        run("1 match { case 1 -> 2 match { case 2 -> \"inner\" else -> \"miss\" } else -> \"outer\" }"),
        "\"inner\""
    );
}

#[test]
fn prepend_adds_one_element_and_converts_zones() {
    assert_eq!(run("4 >> [5, 6]"), "[\n  4,\n  5,\n  6\n]");
    assert_eq!(run("[4] >> [5]"), "[\n  [\n    4\n  ],\n  5\n]");
    assert_eq!(
        run("4 >> [5] >> [6]"),
        "[\n  [\n    4,\n    5\n  ],\n  6\n]"
    );
    assert_eq!(
        run("(|2021-06-01T12:00:00Z| >> |+02:00|) as String"),
        "\"2021-06-01T14:00:00+02:00\""
    );
    assert!(run_result("4 >> 5").is_err());
}

#[test]
fn coercing_equality_compares_across_compatible_types() {
    assert_eq!(run("'8' ~= 8"), "true");
    assert_eq!(run("8 ~= '8'"), "true");
    assert_eq!(run("'FALSE' ~= false"), "true");
    assert_eq!(run("[1, [2]] ~= ['1', ['2']]"), "true");
    assert_eq!(run("{x: 1, y: 2} ~= {x: '1', y: '2'}"), "true");
    assert_eq!(run("{x: 1, y: 2} ~= {y: '2', x: '1'}"), "false");
    assert_eq!(run("'word' ~= 8"), "false");
}

#[test]
fn not_applies_to_the_whole_following_expression() {
    assert_eq!(run("(not false or false)"), "true");
    assert_eq!(run("(not false and true)"), "true");
    assert_eq!(run("(not 3 == 3)"), "false");
    assert_eq!(run("(not not false)"), "false");
    assert_eq!(run("((not false) or false)"), "true");
    assert_eq!(run("(!false or false)"), "true");
}
