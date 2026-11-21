// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for declared types, namespace prefixes, and output forms.

fn run(header: &str, body: &str) -> String {
    let script = format!("%dw 2.0\n{header}\n---\n{body}");
    rustweave_core::run(&script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(header: &str, body: &str) -> Result<String, rustweave_core::DwError> {
    let script = format!("%dw 2.0\n{header}\n---\n{body}");
    rustweave_core::run(&script, &[])
}

#[test]
fn scalar_aliases_coerce_and_test() {
    let header = "output application/json\ntype Code = String\ntype Count = Number";
    assert_eq!(run(header, "'x' as Code"), "\"x\"");
    assert_eq!(run(header, "'x' is Code"), "true");
    assert_eq!(run(header, "7 is Code"), "false");
    assert_eq!(run(header, "9 as Count"), "9");
}

#[test]
fn object_types_check_required_fields_and_keep_extras() {
    let header = "output application/json\ntype Row = {id: Number, note?: String}";
    assert_eq!(run(header, "{id: 4} as Row"), "{\n  \"id\": 4\n}");
    assert_eq!(
        run(header, "{id: 4, extra: true} as Row"),
        "{\n  \"id\": 4,\n  \"extra\": true\n}"
    );
    assert!(run_result(header, "{id: 'four'} as Row").is_err());
    assert!(run_result(
        "output application/json\ntype Pair = {a: Number, b: Number}",
        "{a: 1} as Pair"
    )
    .is_err());
}

#[test]
fn array_element_types_are_not_enforced() {
    let header = "output application/json\ntype Counts = Array<Number>";
    assert_eq!(run(header, "[2, 3] as Counts"), "[\n  2,\n  3\n]");
    assert_eq!(run(header, "['two'] as Counts"), "[\n  \"two\"\n]");
    assert_eq!(
        run(header, "[[1] is Counts, ['two'] is Counts]"),
        "[\n  true,\n  true\n]"
    );
}

#[test]
fn unions_choose_members_and_intersections_merge() {
    let union = "output application/json\ntype Scalar = String | Number";
    assert_eq!(
        run(union, "['x' as Scalar, 2 as Scalar]"),
        "[\n  \"x\",\n  2\n]"
    );
    assert_eq!(run(union, "false as Scalar"), "\"false\"");
    assert_eq!(
        run(union, "['x' is Scalar, 2 is Scalar, false is Scalar]"),
        "[\n  true,\n  true,\n  false\n]"
    );

    let both = "output application/json\ntype Labeled = {name: String} & {code: Number}";
    assert_eq!(
        run(both, "{name: 'bolt', code: 8} as Labeled"),
        "{\n  \"name\": \"bolt\",\n  \"code\": 8\n}"
    );
    assert_eq!(
        run(
            both,
            "[{name: 'bolt', code: 8} is Labeled, {name: 'bolt'} is Labeled]"
        ),
        "[\n  true,\n  false\n]"
    );
}

#[test]
fn aliases_chain_and_resolve_forward_references() {
    let chain = "output application/json\ntype Inner = {size: Number}\ntype Outer = Inner";
    assert_eq!(run(chain, "{size: 3} as Outer"), "{\n  \"size\": 3\n}");

    let forward = "output application/json\ntype Outer = Inner\ntype Inner = {size: Number}";
    assert_eq!(run(forward, "{size: 3} as Outer"), "{\n  \"size\": 3\n}");
}

#[test]
fn nested_types_any_and_duplicate_names() {
    let nested = "output application/json\ntype Catalog = Array<{sku: String}>";
    assert_eq!(
        run(nested, "[{sku: 'A-1'}] as Catalog"),
        "[\n  {\n    \"sku\": \"A-1\"\n  }\n]"
    );

    let any = "output application/json\ntype Free = Any";
    assert_eq!(run(any, "5 as Free"), "5");

    let duplicate = "output application/json\ntype Pick = String\ntype Pick = Number";
    assert!(run_result(duplicate, "1 as Pick").is_err());
}

#[test]
fn value_metadata_attaches_after_an_alias() {
    let header = "output application/json\ntype Code = String";
    assert_eq!(run(header, "('x' as Code {tag: 'cold'}).^tag"), "\"cold\"");
}

#[test]
fn namespace_prefixes_are_dropped_for_json() {
    assert_eq!(
        run(
            "ns g http://example.test/g\noutput application/json",
            "{g#code: 1}"
        ),
        "{\n  \"code\": 1\n}"
    );
    assert_eq!(
        run(
            "output application/json",
            "do { ns g http://example.test/g --- {g#code: 1} }"
        ),
        "{\n  \"code\": 1\n}"
    );
}

#[test]
fn namespace_prefixes_declare_xml_namespaces() {
    assert_eq!(
        run(
            "ns g http://example.test/g\noutput application/xml writeDeclaration=false",
            "{g#code: 1}"
        ),
        "<g:code xmlns:g=\"http://example.test/g\">1</g:code>"
    );
    assert!(run_result(
        "ns g http://a.example\nns g http://b.example\noutput application/xml",
        "{g#code: 1}"
    )
    .is_err());
}

#[test]
fn output_format_ids_select_writers() {
    assert_eq!(run("output json", "{a: 1}"), "{\n  \"a\": 1\n}");
    assert_eq!(run("output csv", "[{a: 1}]"), "a\n1\n");
    assert_eq!(
        run("output xml writeDeclaration=false", "{a: 1}"),
        "<a>1</a>"
    );
    assert_eq!(run("output dw", "{a: 1}"), "{\n  a: 1\n}");
    assert_eq!(run("output text", "'x'"), "x");
}

#[test]
fn with_selects_a_writer_and_keeps_properties() {
    assert_eq!(
        run("output application/problem+json with json", "{a: 1}"),
        "{\n  \"a\": 1\n}"
    );
    assert_eq!(
        run("output application/json with csv", "[{a: 1}]"),
        "a\n1\n"
    );
    assert_eq!(
        run("output application/csv with json", "[{a: 1}]"),
        "[\n  {\n    \"a\": 1\n  }\n]"
    );
    assert_eq!(run("output json indent=false", "{a: 1}"), "{\"a\": 1}");
    assert_eq!(
        run(
            "output application/problem+json with json indent=false",
            "{a: 1}"
        ),
        "{\"a\": 1}"
    );
    assert!(run_result("output application/json with yaml", "{a: 1}").is_err());
}
