// SPDX-License-Identifier: MIT OR Apache-2.0
//! Offline tests for the standard-library functions.
//!
//! These tests need no external runtime and run as part of `cargo test`.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CORE_COLLECTION_HELPERS: &[(&str, &str)] = &[(
    r####"output application/json
---
{
  chars: sizeOf("harbor"),
  fields: sizeOf({x: 1, y: 2, z: 3}),
  digits: sizeOf(98765),
  emptyText: isEmpty(""),
  emptyObject: isEmpty({}),
  firstFind: indexOf("satellite", "ll"),
  lastFind: lastIndexOf("satellite", "ll"),
  increment: 8 then ($ + 3),
  fallback: null onNull () -> "default",
  blanks: [isBlank(null), isBlank("\t "), isBlank("z")],
  smallest: [{v: 9}, {v: 4}, {v: 7}] minBy $.v,
  largest: [{v: 9}, {v: 4}, {v: 7}] maxBy $.v,
  mean: avg([2, 4, 6, 8])
}"####,
    r####"{
  "chars": 6,
  "fields": 3,
  "digits": 5,
  "emptyText": true,
  "emptyObject": true,
  "firstFind": 4,
  "lastFind": 4,
  "increment": 11,
  "fallback": "default",
  "blanks": [
    true,
    true,
    false
  ],
  "smallest": {
    "v": 4
  },
  "largest": {
    "v": 9
  },
  "mean": 5
}"####,
)];

#[test]
fn core_collection_helpers() {
    for (script, expected) in CORE_COLLECTION_HELPERS {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}

const ARRAY_WINDOW_HELPERS: &[(&str, &str)] = &[(
    r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{
  take: ([2, 4, 6] take 5),
  drop: ([2, 4, 6] drop 5),
  slice: slice([5, 10, 15, 20], 1, 3),
  index: ([3, 6, 9] indexWhere ($ >= 6)),
  dropWhile: ([2, 4, 6] dropWhile ($ < 6)),
  parts: ([1, 2, 3, 4, 5] partition (($ mod 2) == 0)),
  first: ([2, 4, 6] firstWith ($ > 3)),
  groups: divideBy([2, 4, 6, 8, 10], 2),
  split: splitWhere([2, 4, 6, 8], (x) -> x > 4),
  absent: ([3, 6, 9] indexWhere ($ > 20))
}"####,
    r####"{
  "take": [
    2,
    4,
    6
  ],
  "drop": [
    
  ],
  "slice": [
    10,
    15
  ],
  "index": 1,
  "dropWhile": [
    6
  ],
  "parts": {
    "success": [
      2,
      4
    ],
    "failure": [
      1,
      3,
      5
    ]
  },
  "first": 4,
  "groups": [
    [
      2,
      4
    ],
    [
      6,
      8
    ],
    [
      10
    ]
  ],
  "split": {
    "l": [
      2,
      4
    ],
    "r": [
      6,
      8
    ]
  },
  "absent": -1
}"####,
)];

#[test]
fn array_window_helpers() {
    for (script, expected) in ARRAY_WINDOW_HELPERS {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}

const STRING_TEXT_HELPERS: &[(&str, &str)] = &[(
    r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{
  left: leftPad("xy", 4, "0"),
  right: rightPad("xy", 4, "0"),
  none: "xy" repeat 0,
  beforeLast: substringBeforeLast("red|green|blue", "|"),
  afterLast: substringAfterLast("red|green|blue", "|"),
  flipped: reverse("straw"),
  middle: substring("strawberry", 2, 6),
  head: first("harbor", 3),
  tail: last("harbor", 3),
  strip: remove("banana", "an"),
  unwrap: unwrap("<note>", "<"),
  every: substringEvery("abcdefgh", 3),
  groups: collapse("aabbbcc"),
  code: charCodeAt("harbor", 2),
  wrapped: wrapWith("done", "~"),
  wrappedIf: wrapIfMissing("done", "~"),
  prefixed: prependIfMissing("done", "pre-"),
  suffixed: appendIfMissing("done", "-end"),
  limited: withMaxSize("harbor", 4),
  all: replaceAll("banana", "na", "NA"),
  count: countMatches("banana", "na"),
  chars: countCharactersBy("a1b2c3", (c) -> isNumeric(c)),
  each: everyCharacter("abc", (c) -> c != ""),
  some: someCharacter("abc", (c) -> c == "b"),
  mapped: mapString("abc", (c, i) -> c ++ i),
  splitPred: substringBy("a,b,c", (c, i) -> c == ","),
  ham: hammingDistance("solar", "polar"),
  lev: levenshteinDistance("harbor", "harbour"),
  words: words("  red  green "),
  lines: lines("one\r\ntwo\nthree")
}"####,
    r####"{
  "left": "00xy",
  "right": "xy00",
  "none": "",
  "beforeLast": "red|green",
  "afterLast": "blue",
  "flipped": "warts",
  "middle": "rawb",
  "head": "har",
  "tail": "bor",
  "strip": "ba",
  "unwrap": "<note>",
  "every": [
    "abc",
    "def",
    "gh"
  ],
  "groups": [
    "aa",
    "bbb",
    "cc"
  ],
  "code": 114,
  "wrapped": "~done~",
  "wrappedIf": "~done~",
  "prefixed": "pre-done",
  "suffixed": "done-end",
  "limited": "harb",
  "all": "baNANA",
  "count": 2,
  "chars": 3,
  "each": true,
  "some": true,
  "mapped": "a0b1c2",
  "splitPred": [
    "a",
    "b",
    "c"
  ],
  "ham": 1,
  "lev": 1,
  "words": [
    "red",
    "green"
  ],
  "lines": [
    "one",
    "two",
    "three"
  ]
}"####,
)];

#[test]
fn string_text_helpers() {
    for (script, expected) in STRING_TEXT_HELPERS {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}

const OBJECT_ENTRY_HELPERS: &[(&str, &str)] = &[(
    r####"%dw 2.0
import * from dw::core::Objects
output application/json
---
{
  entries: entrySet({x: 11}),
  names: nameSet({x: 11}),
  values: valueSet({x: 11}),
  all: ({x: 11, y: 12} everyEntry ($ > 10)),
  any: ({x: 11, y: 12} someEntry ($ == 12)),
  merged: mergeWith({x: 11, y: 12}, {y: 13, z: 14}),
  groups: divideBy({x: 11, y: 12, z: 13}, 2),
  prefix: ({x: 11, y: 12, z: 13} takeWhile ($ < 13))
}"####,
    r####"{
  "entries": [
    {
      "key": "x",
      "value": 11,
      "attributes": {
        
      }
    }
  ],
  "names": [
    "x"
  ],
  "values": [
    11
  ],
  "all": true,
  "any": true,
  "merged": {
    "x": 11,
    "y": 13,
    "z": 14
  },
  "groups": [
    {
      "x": 11,
      "y": 12
    },
    {
      "z": 13
    }
  ],
  "prefix": {
    "x": 11,
    "y": 12
  }
}"####,
)];

#[test]
fn object_entry_helpers() {
    for (script, expected) in OBJECT_ENTRY_HELPERS {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}

const STRING_CASE_HELPERS: &[(&str, &str)] = &[(
    r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{
  plural: ["lamp", "branch", "puppy", "box", "church", "bell"] map pluralize($),
  singular: ["lamps", "branches", "puppies", "boxes", "churches", "bells"] map singularize($),
  camel: ["field_name", "http_server"] map camelize($),
  under: ["FieldName", "field-name"] map underscore($),
  dash: ["FieldName", "field name"] map dasherize($),
  ord: [1, 2, 3, 4, 12, 22, 2.5, 0] map ordinalize($)
}"####,
    r####"{
  "plural": [
    "lamps",
    "branches",
    "puppies",
    "boxes",
    "churches",
    "bells"
  ],
  "singular": [
    "lamp",
    "branch",
    "puppy",
    "box",
    "church",
    "bell"
  ],
  "camel": [
    "fieldName",
    "httpServer"
  ],
  "under": [
    "field_name",
    "field_name"
  ],
  "dash": [
    "field-name",
    "field-name"
  ],
  "ord": [
    "1st",
    "2nd",
    "3rd",
    "4th",
    "12th",
    "22nd",
    "2nd",
    "0th"
  ]
}"####,
)];

#[test]
fn string_case_helpers() {
    for (script, expected) in STRING_CASE_HELPERS {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
