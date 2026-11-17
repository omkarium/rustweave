//! Offline tests for the Rust implementations of the standard-library functions.
//!
//! These tests need no external runtime and run as part of `cargo test`.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn core_library_extras() {
    let script = r#"output application/json
---
{ s: sizeOf("abc"), o: sizeOf({a:1,b:2}), n: sizeOf(123), es: isEmpty(""), eo: isEmpty({}), io: indexOf("abcabc","bc"), lio: lastIndexOf("abcabc","bc"), then: 5 then ($+1), on: null onNull () -> "x", blank: [isBlank(null), isBlank("  "), isBlank("x")], minBy: [{n:2},{n:1}] minBy $.n, maxBy: [{n:2},{n:1}] maxBy $.n, avg: avg([1,2,3]) }"#;
    let expected = r#"{
  "s": 3,
  "o": 2,
  "n": 3,
  "es": true,
  "eo": true,
  "io": 1,
  "lio": 4,
  "then": 6,
  "on": "x",
  "blank": [
    true,
    true,
    false
  ],
  "minBy": {
    "n": 1
  },
  "maxBy": {
    "n": 2
  },
  "avg": 2
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn array_library_extras() {
    let script = r#"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{ take: ([1,2,3] take 5), drop: ([1,2,3] drop 5), slice: slice([1,2,3,4],1,3), iw: ([1,2,3] indexWhere ($ > 1)), dw: ([1,2,3] dropWhile ($ < 3)), p: ([1,2,3,4] partition (($ mod 2) == 0)), fw: ([1,2,3] firstWith ($ > 1)), div: divideBy([1,2,3,4,5],2), sw: splitWhere([1,2,3,4], (x) -> x>2), none: ([1,2,3] indexWhere ($ > 9)) }"#;
    let expected = r#"{
  "take": [
    1,
    2,
    3
  ],
  "drop": [
    
  ],
  "slice": [
    2,
    3
  ],
  "iw": 1,
  "dw": [
    3
  ],
  "p": {
    "success": [
      2,
      4
    ],
    "failure": [
      1,
      3
    ]
  },
  "fw": 2,
  "div": [
    [
      1,
      2
    ],
    [
      3,
      4
    ],
    [
      5
    ]
  ],
  "sw": {
    "l": [
      1,
      2
    ],
    "r": [
      3,
      4
    ]
  },
  "none": -1
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn string_library_extras() {
    let script = r#"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ lp: leftPad("ab",5,"."), rp: rightPad("ab",5,"."), rep0: "ab" repeat 0, sbl: substringBeforeLast("a-b-c","-"), sal: substringAfterLast("a-b-c","-"), rev: reverse("abc"), sub: substring("abcdef",1,4), fst: first("abc",2), lst: last("abc",2), rm: remove("abca","a"), un: unwrap("[abc]","["), se: substringEvery("abcdefg",2), col: collapse("aabbc"), cca: charCodeAt("abc",1), w: wrapWith("a","*"), wim: wrapIfMissing("ab","*"), pre: prependIfMissing("abc","x"), app: appendIfMissing("abc","x"), max: withMaxSize("abcdef",3), ra: replaceAll("aaa","a","b"), cm: countMatches("abab","ab"), ccb: countCharactersBy("a1b2", (c) -> isNumeric(c)), ec: everyCharacter("ab", (c) -> c != ""), sc: someCharacter("ab", (c) -> c == "b"), ms: mapString("ab", (c,i) -> c ++ i), sb: substringBy("a,b,c", (c,i) -> c == ","), ham: hammingDistance("abc","abd"), lev: levenshteinDistance("kitten","sitting"), wd: words(" a  b "), ln: lines("a\r\nb\nc") }"#;
    let expected = r#"{
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
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn object_library_extras() {
    let script = r#"%dw 2.0
import * from dw::core::Objects
output application/json
---
{ es: entrySet({a:1}), names: nameSet({a:1}), vals: valueSet({a:1}), ee: ({a:1,b:2} everyEntry ($ > 0)), se: ({a:1,b:2} someEntry ($ == 2)), mw: mergeWith({a:1,b:2},{b:3,c:4}), div: divideBy({a:1,b:2,c:3},2), tw: ({a:1,b:2,c:3} takeWhile ($ < 3)) }"#;
    let expected = r#"{
  "es": [
    {
      "key": "a",
      "value": 1,
      "attributes": {
        
      }
    }
  ],
  "names": [
    "a"
  ],
  "vals": [
    1
  ],
  "ee": true,
  "se": true,
  "mw": {
    "a": 1,
    "b": 3,
    "c": 4
  },
  "div": [
    {
      "a": 1,
      "b": 2
    },
    {
      "c": 3
    }
  ],
  "tw": {
    "a": 1,
    "b": 2
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn string_inflector_helpers() {
    let script = r#"%dw 2.0
import * from dw::core::Strings
output application/json
---
{ plural: ["cat", "bus", "city", "person", "goose", "sheep"] map pluralize($), singular: ["cats", "buses", "cities", "people", "geese", "sheep"] map singularize($), camel: ["hello_world", "XMLHttpRequest"] map camelize($), under: ["HelloWorld", "hello-world"] map underscore($), dash: ["HelloWorld", "hello world"] map dasherize($), ord: [1, 2, 3, 4, 11, 21, 1.5, -1] map ordinalize($) }"#;
    let expected = r#"{
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
}"#;
    assert_eq!(run(script), expected);
}
