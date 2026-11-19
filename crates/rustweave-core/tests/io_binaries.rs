// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for read/write, JSON options, and binaries.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn read_and_write_json() {
    let script = r#"{
  a: read("{ \"hello\" : \"world\" }", "application/json"),
  b: write({hello: "world"}, "application/json"),
  c: read("1 + 1", "application/dw"),
  d: write("world", "text/plain"),
  e: read("plain text", "text/plain")
}"#;
    let expected = r#"{
  "a": {
    "hello": "world"
  },
  "b": "{\n  \"hello\": \"world\"\n}",
  "c": 2,
  "d": "world",
  "e": "plain text"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn json_writer_options() {
    let script = r#"{
  compact: write({a: [], b: ["h","o","l","a"]}, "application/json", {indent: false}),
  skipObjects: write({a: null, b: 1}, "application/json", {skipNullOn: "objects"}),
  skipArrays: write([1, null, 2], "application/json", {skipNullOn: "arrays"}),
  skipEverywhere: write({a: [1, null], b: null}, "application/json", {skipNullOn: "everywhere"}),
  duplicates: write({a: 1, a: 2}, "application/json", {duplicateKeyAsArray: true, indent: false})
}"#;
    let expected = r#"{
  "compact": "{\"a\": [],\"b\": [\"h\",\"o\",\"l\",\"a\"]}",
  "skipObjects": "{\n  \"b\": 1\n}",
  "skipArrays": "[\n  1,\n  2\n]",
  "skipEverywhere": "{\n  \"a\": [\n    1\n  ]\n}",
  "duplicates": "{\"a\": [1,2]}"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn binary_values_and_helpers() {
    let script = r#"%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  literalType: typeOf(|4D756C65|),
  literalHex: toHex(|4D756C65|),
  hex: toHex(fromHex("4D756C65")),
  base64: toBase64(fromHex("4D756C65")),
  fromBase64: toHex(fromBase64("TXVsZQ==")),
  concat: toHex(fromHex("CAFE") concatWith fromHex("ABCD")),
  nullLeft: toHex(concatWith(null, fromHex("CAFE"))),
  nullRight: toHex(fromHex("CAFE") concatWith null)
}"#;
    let expected = r#"{
  "literalType": "Binary",
  "literalHex": "4D756C65",
  "hex": "4D756C65",
  "base64": "TXVsZQ==",
  "fromBase64": "4D756C65",
  "concat": "CAFEABCD",
  "nullLeft": "CAFE",
  "nullRight": "CAFE"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn binary_lines() {
    let script = r#"%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  lines: fromHex("4C696E6520310A4C696E6520320A4C696E6520330A") readLinesWith "UTF-8",
  written: toHex(writeLinesWith(["Line 1", "Line 2", "Line 3"], "UTF-8")),
  binary: toHex(read("Line 1\nLine 2\n", "application/octet-stream")),
  type: typeOf(read("Line 1\nLine 2\n", "application/octet-stream"))
}"#;
    let expected = r#"{
  "lines": [
    "Line 1",
    "Line 2",
    "Line 3"
  ],
  "written": "4C696E6520310A4C696E6520320A4C696E6520330A",
  "binary": "4C696E6520310A4C696E6520320A",
  "type": "Binary"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn read_url_from_file() {
    let path = std::env::temp_dir().join(format!("rustweave-{}.json", std::process::id()));
    std::fs::write(&path, r#"{"ok":true}"#).expect("write temp file");
    let url = format!("file://{}", path.display());
    let script = format!(r#"readUrl("{}", "application/json").ok"#, url);
    let result = run(&script);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result, "true");
}
