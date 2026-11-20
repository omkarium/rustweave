// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for read/write, JSON options, and binaries.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"{
  a: read("{ \"item\" : \"widget\" }", "application/json"),
  b: write({item: "widget"}, "application/json"),
  c: read("2 * 3", "application/dw"),
  d: write("note", "text/plain"),
  e: read("plain note", "text/plain")
}"####,
        r####"{
  "a": {
    "item": "widget"
  },
  "b": "{\n  \"item\": \"widget\"\n}",
  "c": 6,
  "d": "note",
  "e": "plain note"
}"####,
    ),
    (
        r####"{
  compact: write({k: [], v: ["r", "i", "v", "e", "r"]}, "application/json", {indent: false}),
  skipObjects: write({a: null, b: 2}, "application/json", {skipNullOn: "objects"}),
  skipArrays: write([3, null, 4], "application/json", {skipNullOn: "arrays"}),
  skipEverywhere: write({a: [2, null], b: null}, "application/json", {skipNullOn: "everywhere"}),
  duplicates: write({a: 3, a: 4}, "application/json", {duplicateKeyAsArray: true, indent: false})
}"####,
        r####"{
  "compact": "{\"k\": [],\"v\": [\"r\",\"i\",\"v\",\"e\",\"r\"]}",
  "skipObjects": "{\n  \"b\": 2\n}",
  "skipArrays": "[\n  3,\n  4\n]",
  "skipEverywhere": "{\n  \"a\": [\n    2\n  ]\n}",
  "duplicates": "{\"a\": [3,4]}"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  literalType: typeOf(|5269766572|),
  literalHex: toHex(|5269766572|),
  hex: toHex(fromHex("5269766572")),
  base64: toBase64(fromHex("5269766572")),
  fromBase64: toHex(fromBase64("Uml2ZXI=")),
  concat: toHex(fromHex("CAFE") concatWith fromHex("BEEF")),
  nullLeft: toHex(concatWith(null, fromHex("BEEF"))),
  nullRight: toHex(fromHex("BEEF") concatWith null)
}"####,
        r####"{
  "literalType": "Binary",
  "literalHex": "5269766572",
  "hex": "5269766572",
  "base64": "Uml2ZXI=",
  "fromBase64": "5269766572",
  "concat": "CAFEBEEF",
  "nullLeft": "BEEF",
  "nullRight": "BEEF"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  lines: fromHex("4E6F7274680A536F7574680A") readLinesWith "UTF-8",
  written: toHex(writeLinesWith(["North", "South"], "UTF-8")),
  binary: toHex(read("North\nSouth\n", "application/octet-stream")),
  type: typeOf(read("North\nSouth\n", "application/octet-stream"))
}"####,
        r####"{
  "lines": [
    "North",
    "South"
  ],
  "written": "4E6F7274680A536F7574680A",
  "binary": "4E6F7274680A536F7574680A",
  "type": "Binary"
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}

#[test]
fn read_url_from_file() {
    let path = std::env::temp_dir().join(format!("rustweave-{}.json", std::process::id()));
    std::fs::write(&path, r#"{"seen":true}"#).expect("write temp file");
    let url = format!("file://{}", path.display());
    let script = format!(r#"readUrl("{}", "application/json").seen"#, url);
    let result = run(&script);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result, "true");
}
