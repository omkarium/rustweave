//! Behaviour tests for the Strings module.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####""abba" find "b""####,
        r####"[
  1,
  2
]"####,
    ),
    (
        r####""aaa" find "aa""####,
        r####"[
  0
]"####,
    ),
    (r####""hello" replace "l" with "L""####, r####""heLLo""####),
    (
        r####""hell" splitBy "l""####,
        r####"[
  "he"
]"####,
    ),
    (r####"upper("Ab") ++ lower("Cd")"####, r####""ABcd""####),
    (r####"trim("  x  ")"####, r####""x""####),
    (r####""hi" startsWith "h""####, r####"true"####),
    (r####"sizeOf("ab")"####, r####"2"####),
    (r####"isBlank("  ")"####, r####"true"####),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { cap: capitalize("hello world"), pad: leftPad("ab", 5), rep: "ab" repeat 2,        rev: reverse("ab"), before: substringBefore("a-b-c", "-"),        after: substringAfter("a-b-c", "-"), rm: remove("abca", "a"),        col: collapse("aabbc") }"####,
        r####"{
  "cap": "Hello World",
  "pad": "   ab",
  "rep": "abab",
  "rev": "ba",
  "before": "a",
  "after": "b-c",
  "rm": "bc",
  "col": [
    "aa",
    "bb",
    "c"
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { plural: ["cat", "bus", "city", "person", "goose", "sheep"] map pluralize($),        singular: ["cats", "buses", "cities", "people", "geese", "sheep"] map singularize($),        camel: ["hello_world", "XMLHttpRequest"] map camelize($),        under: ["HelloWorld", "hello-world"] map underscore($),        dash: ["HelloWorld", "hello world"] map dasherize($),        ord: [1, 2, 3, 4, 11, 21, 1.5, -1] map ordinalize($) }"####,
        r####"{
  "plural": [
    "cats",
    "buses",
    "cities",
    "people",
    "geese",
    "sheep"
  ],
  "singular": [
    "cat",
    "bus",
    "city",
    "person",
    "goose",
    "sheep"
  ],
  "camel": [
    "helloWorld",
    "xMLHttpRequest"
  ],
  "under": [
    "hello_world",
    "hello_world"
  ],
  "dash": [
    "hello-world",
    "hello-world"
  ],
  "ord": [
    "1st",
    "2nd",
    "3rd",
    "4th",
    "11th",
    "21st",
    "1st",
    "-1th"
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { wrap: wrapWith("ab", "*"), wim: [wrapIfMissing("ab", "x"), wrapIfMissing("xabx", "x")],        pre: prependIfMissing("abc", "xyz"), app: appendIfMissing("abc", "xyz"),        max: withMaxSize("123", 2), all: replaceAll("AAAA", "AAA", "B"),        matches: countMatches("hello worlo!", "lo") }"####,
        r####"{
  "wrap": "*ab*",
  "wim": [
    "xabx",
    "xabx"
  ],
  "pre": "xyzabc",
  "app": "abcxyz",
  "max": "12",
  "all": "BA",
  "matches": 2
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { count: "42 = 11 * 2 + 20" countCharactersBy isNumeric($),        every: "12 34  56" everyCharacter $ == " " or isNumeric($),        some: "someCharacter" someCharacter isUpperCase($),        map: "x234" mapString (if (isNumeric($)) "~" else $),        by: "hello~world=here_data-weave" substringBy $ == "~" or $ == "=" or $ == "_" }"####,
        r####"{
  "count": 7,
  "every": true,
  "some": true,
  "map": "x~~~",
  "by": [
    "hello",
    "world",
    "here",
    "data-weave"
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Strings
output application/json
---
     { ham: hammingDistance("holu", "chau"), bad: hammingDistance("a", "bb"),        lev: levenshteinDistance("kitten", "sitting"), cc: charCode("M"),        ccat: charCodeAt("DataWeave", 1), fcc: fromCharCode(77),        ws: [isWhitespace(""), isWhitespace("  "), isWhitespace("a")] }"####,
        r####"{
  "ham": 3,
  "bad": null,
  "lev": 3,
  "cc": 77,
  "ccat": 97,
  "fcc": "M",
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
{ lp: leftPad("ab",5,"."), rp: rightPad("ab",5,"."), rep0: "ab" repeat 0, sbl: substringBeforeLast("a-b-c","-"), sal: substringAfterLast("a-b-c","-"), rev: reverse("abc"), sub: substring("abcdef",1,4), fst: first("abc",2), lst: last("abc",2), rm: remove("abca","a"), un: unwrap("[abc]","["), se: substringEvery("abcdefg",2), col: collapse("aabbc"), cca: charCodeAt("abc",1), w: wrapWith("a","*"), wim: wrapIfMissing("ab","*"), pre: prependIfMissing("abc","x"), app: appendIfMissing("abc","x"), max: withMaxSize("abcdef",3), ra: replaceAll("aaa","a","b"), cm: countMatches("abab","ab"), ccb: countCharactersBy("a1b2", (c) -> isNumeric(c)), ec: everyCharacter("ab", (c) -> c != ""), sc: someCharacter("ab", (c) -> c == "b"), ms: mapString("ab", (c,i) -> c ++ i), sb: substringBy("a,b,c", (c,i) -> c == ","), ham: hammingDistance("abc","abd"), lev: levenshteinDistance("kitten","sitting"), wd: words(" a  b "), ln: lines("a\r\nb\nc") }"####,
        r####"{
  "lp": "...ab",
  "rp": "ab...",
  "rep0": "",
  "sbl": "a-b",
  "sal": "c",
  "rev": "cba",
  "sub": "bcd",
  "fst": "ab",
  "lst": "bc",
  "rm": "bc",
  "un": "[abc]",
  "se": [
    "ab",
    "cd",
    "ef",
    "g"
  ],
  "col": [
    "aa",
    "bb",
    "c"
  ],
  "cca": 98,
  "w": "*a*",
  "wim": "*ab*",
  "pre": "xabc",
  "app": "abcx",
  "max": "abc",
  "ra": "bbb",
  "cm": 2,
  "ccb": 2,
  "ec": true,
  "sc": true,
  "ms": "a0b1",
  "sb": [
    "a",
    "b",
    "c"
  ],
  "ham": 1,
  "lev": 3,
  "wd": [
    "a",
    "b"
  ],
  "ln": [
    "a",
    "b",
    "c"
  ]
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
