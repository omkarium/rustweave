// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Strings module.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####""banana" find "na""####,
        r####"[
  2,
  4
]"####,
    ),
    (
        r####""aaaa" find "aa""####,
        r####"[
  0,
  2
]"####,
    ),
    (
        r####""pepper" replace "p" with "P""####,
        r####""PePPer""####,
    ),
    (
        r####""mango" splitBy "n""####,
        r####"[
  "ma",
  "go"
]"####,
    ),
    (
        r####"upper("Kiwi") ++ lower("PLUM")"####,
        r####""KIWIplum""####,
    ),
    (r####"trim("  lime  ")"####, r####""lime""####),
    (r####""harbor" startsWith "har""####, r####"true"####),
    (r####"sizeOf("peach")"####, r####"5"####),
    (r####"isBlank("\t ")"####, r####"true"####),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ cap: capitalize("quiet harbor"), pad: leftPad("xy", 4), rep: "go" repeat 3, rev: reverse("lamp"), before: substringBefore("red|green|blue", "|"), after: substringAfter("red|green|blue", "|"), rm: remove("banana", "an"), col: collapse("aabbbcc") }"####,
        r####"{
  "cap": "Quiet Harbor",
  "pad": "  xy",
  "rep": "gogogo",
  "rev": "pmal",
  "before": "red",
  "after": "green|blue",
  "rm": "ba",
  "col": [
    "aa",
    "bbb",
    "cc"
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ plural: ["lamp", "branch", "puppy", "box", "church", "bell"] map pluralize($), singular: ["lamps", "branches", "puppies", "boxes", "churches", "bells"] map singularize($), camel: ["field_name", "http_server"] map camelize($), under: ["FieldName", "field-name"] map underscore($), dash: ["FieldName", "field name"] map dasherize($), ord: [1, 2, 3, 4, 12, 22, 2.5, 0] map ordinalize($) }"####,
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
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ wrap: wrapWith("note", "~"), wim: [wrapIfMissing("note", "*"), wrapIfMissing("*note*", "*")], pre: prependIfMissing("note", "pre-"), app: appendIfMissing("note", "-end"), max: withMaxSize("harbor", 4), all: replaceAll("banana", "na", "NA"), matches: countMatches("banana", "na") }"####,
        r####"{
  "wrap": "~note~",
  "wim": [
    "*note*",
    "*note*"
  ],
  "pre": "pre-note",
  "app": "note-end",
  "max": "harb",
  "all": "baNANA",
  "matches": 2
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ count: "a1b2c3d4" countCharactersBy isNumeric($), every: "1 2 3" everyCharacter $ == " " or isNumeric($), some: "Harbor" someCharacter isUpperCase($), map: "a1b2" mapString (if (isNumeric($)) "#" else $), by: "red,green|blue_cyan" substringBy $ == "," or $ == "|" or $ == "_" }"####,
        r####"{
  "count": 4,
  "every": true,
  "some": true,
  "map": "a#b#",
  "by": [
    "red",
    "green",
    "blue",
    "cyan"
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ ham: hammingDistance("solar", "polar"), bad: hammingDistance("a", "xyz"), lev: levenshteinDistance("harbor", "harbour"), cc: charCode("Q"), ccat: charCodeAt("harbor", 2), fcc: fromCharCode(82), ws: [isWhitespace(""), isWhitespace("\t "), isWhitespace("q")] }"####,
        r####"{
  "ham": 1,
  "bad": null,
  "lev": 1,
  "cc": 81,
  "ccat": 114,
  "fcc": "R",
  "ws": [
    true,
    true,
    false
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ lp: leftPad("xy", 4, "0"), rp: rightPad("xy", 4, "0"), rep0: "go" repeat 0, sbl: substringBeforeLast("red|green|blue", "|"), sal: substringAfterLast("red|green|blue", "|"), sub: substring("harbor", 1, 4), fst: first("harbor", 3), lst: last("harbor", 3), un: unwrap("<note>", "<"), se: substringEvery("abcdefgh", 3), wd: words("  red  green  "), ln: lines("one\r\ntwo\nthree") }"####,
        r####"{
  "lp": "00xy",
  "rp": "xy00",
  "rep0": "",
  "sbl": "red|green",
  "sal": "blue",
  "sub": "arb",
  "fst": "har",
  "lst": "bor",
  "un": "<note>",
  "se": [
    "abc",
    "def",
    "gh"
  ],
  "wd": [
    "red",
    "green"
  ],
  "ln": [
    "one",
    "two",
    "three"
  ]
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
