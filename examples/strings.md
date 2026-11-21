# Strings functions in one script

One text-processing result object applies casing, padding, trimming, pluralisation, code-point helpers, distance helpers, predicates, and the different substring families to small samples.

This script uses all 47 functions listed for `dw::core::Strings`.

Import `dw::core::Strings` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Strings
output application/json
---
{
  append: appendIfMissing("note", "-end"),
  camel: camelize("field_name"),
  cap: capitalize("quiet harbor"),
  charCode: charCode("Q"),
  charCodeAt: charCodeAt("harbor", 2),
  collapse: collapse("aabbbcc"),
  countChars: "a1b2c3" countCharactersBy isNumeric($),
  countMatches: countMatches("banana", "na"),
  dash: dasherize("FieldName"),
  every: "1 2 3" everyCharacter $ == " " or isNumeric($),
  first: first("harbor", 3),
  fromCharCode: fromCharCode(82),
  ham: hammingDistance("solar", "polar"),
  alpha: [isAlpha("abc"), isAlpha("a1")],
  alnum: [isAlphanumeric("abc1"), isAlphanumeric("a!")],
  lowerCase: isLowerCase("tide"),
  numeric: isNumeric("123"),
  upperCase: isUpperCase("TIDE"),
  whitespace: [isWhitespace(""), isWhitespace("\t "), isWhitespace("q")],
  last: last("harbor", 3),
  leftPad: leftPad("xy", 4, "0"),
  lev: levenshteinDistance("harbor", "harbour"),
  lines: lines("one\r\ntwo\nthree"),
  mapString: "a1b2" mapString (if (isNumeric($)) "#" else $),
  ordinal: [1, 2, 3, 4, 12, 22] map ordinalize($),
  plural: ["lamp", "branch", "puppy", "box"] map pluralize($),
  prepend: prependIfMissing("note", "pre-"),
  remove: remove("banana", "an"),
  repeat: "go" repeat 3,
  replaceAll: replaceAll("banana", "na", "NA"),
  reverse: reverse("lamp"),
  rightPad: rightPad("xy", 4, "0"),
  singular: ["lamps", "branches", "puppies", "boxes"] map singularize($),
  some: "Harbor" someCharacter isUpperCase($),
  substring: substring("harbor", 1, 4),
  after: substringAfter("red|green|blue", "|"),
  afterLast: substringAfterLast("red|green|blue", "|"),
  before: substringBefore("red|green|blue", "|"),
  beforeLast: substringBeforeLast("red|green|blue", "|"),
  by: "red,green|blue_cyan" substringBy $ == "," or $ == "|" or $ == "_",
  everySub: substringEvery("abcdefgh", 3),
  underscore: underscore("FieldName"),
  unwrap: unwrap("<note>", "<"),
  maxSize: withMaxSize("harbor", 4),
  words: words("  red  green  "),
  wrapIfMissing: [wrapIfMissing("note", "~"), wrapIfMissing("~note~", "~")],
  wrap: wrapWith("note", "~")
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/strings.dwl
```

## Expected output

```json
{
  "append": "note-end",
  "camel": "fieldName",
  "cap": "Quiet Harbor",
  "charCode": 81,
  "charCodeAt": 114,
  "collapse": [
    "aa",
    "bbb",
    "cc"
  ],
  "countChars": 3,
  "countMatches": 2,
  "dash": "field-name",
  "every": true,
  "first": "har",
  "fromCharCode": "R",
  "ham": 1,
  "alpha": [
    true,
    false
  ],
  "alnum": [
    true,
    false
  ],
  "lowerCase": true,
  "numeric": true,
  "upperCase": true,
  "whitespace": [
    true,
    true,
    false
  ],
  "last": "bor",
  "leftPad": "00xy",
  "lev": 1,
  "lines": [
    "one",
    "two",
    "three"
  ],
  "mapString": "a#b#",
  "ordinal": [
    "1st",
    "2nd",
    "3rd",
    "4th",
    "12th",
    "22nd"
  ],
  "plural": [
    "lamps",
    "branches",
    "puppies",
    "boxes"
  ],
  "prepend": "pre-note",
  "remove": "ba",
  "repeat": "gogogo",
  "replaceAll": "baNANA",
  "reverse": "pmal",
  "rightPad": "xy00",
  "singular": [
    "lamp",
    "branch",
    "puppy",
    "box"
  ],
  "some": true,
  "substring": "arb",
  "after": "green|blue",
  "afterLast": "blue",
  "before": "red",
  "beforeLast": "red|green",
  "by": [
    "red",
    "green",
    "blue",
    "cyan"
  ],
  "everySub": [
    "abc",
    "def",
    "gh"
  ],
  "underscore": "field_name",
  "unwrap": "<note>",
  "maxSize": "harb",
  "words": [
    "red",
    "green"
  ],
  "wrapIfMissing": [
    "~note~",
    "~note~"
  ],
  "wrap": "~note~"
}
```

## Functions shown

`appendIfMissing`, `camelize`, `capitalize`, `charCode`, `charCodeAt`, `collapse`, `countCharactersBy`, `countMatches`, `dasherize`, `everyCharacter`, `first`, `fromCharCode`, `hammingDistance`, `isAlpha`, `isAlphanumeric`, `isLowerCase`, `isNumeric`, `isUpperCase`, `isWhitespace`, `last`, `leftPad`, `levenshteinDistance`, `lines`, `mapString`, `ordinalize`, `pluralize`, `prependIfMissing`, `remove`, `repeat`, `replaceAll`, `reverse`, `rightPad`, `singularize`, `someCharacter`, `substring`, `substringAfter`, `substringAfterLast`, `substringBefore`, `substringBeforeLast`, `substringBy`, `substringEvery`, `underscore`, `unwrap`, `withMaxSize`, `words`, `wrapIfMissing`, `wrapWith`
