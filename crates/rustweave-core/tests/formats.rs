// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for CSV, XML, and YAML formats.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[])
}

fn json_text(text: &str) -> String {
    rustweave_core::json::write(&rustweave_core::Value::String(text.to_string()))
}

#[test]
fn csv_read_options() {
    let script = r####"%dw 2.0
output application/json
---
{
  quoted: read("code,note\nG-17,\"calm, grey\"\nG-18,\"line one\nline two\"\n", "Application/CSV; charset=UTF-8"),
  noHeader: read("1;2\n3;4\n", "text/csv", {header: false, separator: ";"}),
  padded: read("a,b,c\n1,2\n", "application/csv", {nullValue: "null"}),
  nulled: read("a,b\n1,null\n", "application/csv", {nullValue: "null"}),
  blanks: read("a,b\n1,2\n\n3,4\n", "application/csv", {skipEmptyLine: false})
}"####;
    let expected = r####"{
  "quoted": [
    {
      "code": "G-17",
      "note": "calm, grey"
    },
    {
      "code": "G-18",
      "note": "line one\nline two"
    }
  ],
  "noHeader": [
    [
      "1",
      "2"
    ],
    [
      "3",
      "4"
    ]
  ],
  "padded": [
    {
      "a": "1",
      "b": "2",
      "c": ""
    }
  ],
  "nulled": [
    {
      "a": "1",
      "b": null
    }
  ],
  "blanks": [
    {
      "a": "1",
      "b": "2"
    },
    {
      "a": "",
      "b": ""
    },
    {
      "a": "3",
      "b": "4"
    }
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn csv_write_options() {
    let script = r####"%dw 2.0
output application/json
---
{
  objects: write([{code: "G-17", note: "calm, grey"}, {code: "G-18", note: "line one\nline two"}], "application/csv"),
  arrays: write([["G-17", "4"], ["G-18", "5"]], "application/csv"),
  custom: write([{a: "x;y", b: "q"}], "application/csv", {separator: ";", quote: "|"})
}"####;
    let expected = r####"{
  "objects": "code,note\nG-17,\"calm, grey\"\nG-18,\"line one\nline two\"\n",
  "arrays": "G-17,4\nG-18,5\n",
  "custom": "a;b\n|x;y|;q\n"
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn csv_output_formats() {
    let script = r####"%dw 2.0
output application/csv
---
[{code: "G-17", note: "calm, grey"}, {code: "G-18", note: "line one\nline two"}]"####;
    let expected = "code,note\nG-17,\"calm, grey\"\nG-18,\"line one\nline two\"\n";
    assert_eq!(run(script), expected);

    let script = r####"%dw 2.0
output application/csv header=false, separator=";"
---
[{a: "x;y", b: 2}, {a: "z", b: 3}]"####;
    assert_eq!(run(script), "\"x;y\";2\nz;3\n");
}

#[test]
fn csv_errors() {
    for script in [
        r####"read("a,a\n1,2\n", "application/csv")"####,
        r####"read("a,b\n1,2,3\n", "application/csv")"####,
        r####"read("a,b\n1,2\n", "application/csv", {separator: "::"})"####,
        r####"write([[["nested"]]], "application/csv")"####,
        r####"write([{a: {b: 1}}], "application/csv")"####,
    ] {
        assert!(run_result(script).is_err(), "expected error: {script}");
    }
}

#[test]
fn unknown_mime_errors() {
    for script in [
        r####"read("x", "application/nope")"####,
        r####"write({a: 1}, "application/nope")"####,
    ] {
        assert!(run_result(script).is_err(), "expected error: {script}");
    }
}

#[test]
fn read_url_uses_extension() {
    let base = std::env::temp_dir();
    let id = std::process::id();
    let cases = [
        (
            format!("rustweave-{id}-records.json"),
            r#"{"ok":true}"#,
            "{\n  \"ok\": true\n}",
        ),
        (
            format!("rustweave-{id}-records.csv"),
            "code,count\nG-17,4\n",
            "[\n  {\n    \"code\": \"G-17\",\n    \"count\": \"4\"\n  }\n]",
        ),
        (
            format!("rustweave-{id}-records.txt"),
            "plain note",
            "\"plain note\"",
        ),
    ];
    for (name, content, expected) in cases {
        let path = base.join(name);
        std::fs::write(&path, content).expect("write temp file");
        let script = format!(r####"readUrl("file://{}", null)"####, path.display());
        let result = run(&script);
        let _ = std::fs::remove_file(&path);
        assert_eq!(result, expected, "file: {}", path.display());
    }
}
