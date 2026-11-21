// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn title_and_strip() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  words: title("quiet HARBOR"),
  punctuation: title("tide-pool STORIES"),
  unicode: title("élan vital"),
  apostrophe: title("o'neill"),
  mixed: title("mixed_CASE words"),
  leading: lstrip("  ...note  "),
  trailing: rstrip("  ...note  "),
  leadingChars: lstrip("xxxyvalue", "xy"),
  trailingChars: rstrip("valuexyx", "xy"),
  noChars: lstrip("keep", ""),
  nullChars: rstrip("keep  ", null)
}"####;
    let expected = r####"{
  "words": "Quiet Harbor",
  "punctuation": "Tide-Pool Stories",
  "unicode": "Élan Vital",
  "apostrophe": "O'Neill",
  "mixed": "Mixed_Case Words",
  "leading": "...note  ",
  "trailing": "  ...note",
  "leadingChars": "value",
  "trailingChars": "value",
  "noChars": "keep",
  "nullChars": "keep"
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn zfill_and_partition() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  positive: zfill("42", 5),
  negative: zfill("-42", 5),
  signed: zfill("+7", 4),
  alreadyLong: zfill("tool", 2),
  empty: zfill("", 3),
  zeroWidth: zfill("x", 0),
  first: partition("red|green|blue", "|"),
  firstAtStart: partition("|red|green", "|"),
  firstAtEnd: partition("red|green|", "|"),
  missing: partition("plain", "|"),
  last: rpartition("red|green|blue", "|"),
  lastMissing: rpartition("plain", "|")
}"####;
    let expected = r####"{
  "positive": "00042",
  "negative": "-0042",
  "signed": "+007",
  "alreadyLong": "tool",
  "empty": "000",
  "zeroWidth": "x",
  "first": [
    "red",
    "|",
    "green|blue"
  ],
  "firstAtStart": [
    "",
    "|",
    "red|green"
  ],
  "firstAtEnd": [
    "red",
    "|",
    "green|"
  ],
  "missing": [
    "plain",
    "",
    ""
  ],
  "last": [
    "red|green",
    "|",
    "blue"
  ],
  "lastMissing": [
    "",
    "",
    "plain"
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn split_once_and_strip_affixes() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  first: splitOnce("alpha/beta/gamma", "/"),
  firstAtStart: splitOnce("/alpha/beta", "/"),
  firstMissing: splitOnce("plain", "/"),
  last: rsplitOnce("alpha/beta/gamma", "/"),
  lastAtEnd: rsplitOnce("alpha/beta/", "/"),
  lastMissing: rsplitOnce("plain", "/"),
  prefix: stripPrefix("v1.2.3", "v"),
  prefixMissing: stripPrefix("1.2.3", "v"),
  emptyPrefix: stripPrefix("keep", ""),
  suffix: stripSuffix("report.csv", ".csv"),
  suffixMissing: stripSuffix("report.txt", ".csv"),
  emptySuffix: stripSuffix("keep", ""),
  unicode: stripPrefix("café", "ca")
}"####;
    let expected = r####"{
  "first": [
    "alpha",
    "beta/gamma"
  ],
  "firstAtStart": [
    "",
    "alpha/beta"
  ],
  "firstMissing": null,
  "last": [
    "alpha/beta",
    "gamma"
  ],
  "lastAtEnd": [
    "alpha/beta",
    ""
  ],
  "lastMissing": null,
  "prefix": "1.2.3",
  "prefixMissing": null,
  "emptyPrefix": "keep",
  "suffix": "report",
  "suffixMissing": null,
  "emptySuffix": "keep",
  "unicode": "fé"
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn string_argument_errors() {
    for script in [
        r####"title(null)"####,
        r####"title(1)"####,
        r####"lstrip(null)"####,
        r####"lstrip(1, "x")"####,
        r####"rstrip(null)"####,
        r####"rstrip("a", 1)"####,
        r####"zfill("1", -1)"####,
        r####"zfill("1", 1.5)"####,
        r####"zfill(null, 2)"####,
        r####"partition("a", "")"####,
        r####"rpartition("a", "")"####,
        r####"partition(null, "a")"####,
        r####"rpartition("a", null)"####,
        r####"splitOnce(null, "/")"####,
        r####"splitOnce("a", null)"####,
        r####"splitOnce("a", "")"####,
        r####"rsplitOnce("a", "")"####,
        r####"rsplitOnce(null, "/")"####,
        r####"stripPrefix(null, "a")"####,
        r####"stripPrefix("a", null)"####,
        r####"stripSuffix(null, "a")"####,
        r####"stripSuffix("a", null)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}

#[test]
fn array_partition_is_not_shadowed() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
import * from dw::core::Arrays
output application/json
---
partition([1, 2, 3, 4], (($ mod 2) == 0))"####;
    let expected = r####"{
  "success": [
    2,
    4
  ],
  "failure": [
    1,
    3
  ]
}"####;
    assert_eq!(run(script), expected);
}
