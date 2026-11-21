# Extension Common functions in one script

Every general-purpose helper in the extension module appears once. Collection, object, number,
string, function, and null helpers are grouped by the kind of value they return. The crypto helpers
have their own guide in [extension-crypto.md](extension-crypto.md).

This script uses the 50 general-purpose functions in `dw::extra::Common`.

Import `dw::extra::Common` to use the module functions. `null` is the only value treated as absent.

## Script

```dataweave
%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  enumerate: enumerate(["map", "compass"], 1),
  range: range(2, 9, 2),
  sorted: sorted([{name: "sail", rank: 2}, {name: "hull", rank: 1}], (item) -> item.rank),
  reversed: reversed([3, 5, 7]),
  chunk: chunk([1, 2, 3, 4, 5], 2),
  compact: compact([null, "kept", null, false, 0]),
  unique: unique([{code: "a"}, {code: "a"}, {code: "b"}]),
  fold: fold([2, 3, 4], 10, (item, acc) -> acc + item),
  filterMap: filterMap([1, 2, 3, 4, 5], (n) -> if ((n mod 2) == 0) n * 10 else null),
  stepBy: stepBy(["zero", "one", "two", "three", "four"], 2),
  position: position([5, 8, 13, 21], (n) -> (n mod 2) == 0),
  rposition: rposition([5, 8, 13, 21], (n) -> (n mod 2) != 0),
  dedup: dedup(["north", "north", "south", "north"]),
  windows: windows([10, 20, 30, 40], 2),
  chunksExact: chunksExact([1, 2, 3, 4, 5], 2),
  binarySearch: binarySearch([2, 4, 6, 8], 6),
  binaryInsert: binarySearch([2, 4, 6, 8], 5),
  rotateLeft: rotateLeft(["a", "b", "c", "d"], 1),
  rotateRight: rotateRight(["a", "b", "c", "d"], 1),
  swap: swap(["first", "second", "third"], 0, 2),
  fill: fill([0, 0, 0, 0], 9, 1, 3),
  truncate: truncate(["a", "b", "c"], 2),
  fromEntries: fromEntries([{key: "sku", value: "G-17"}, ["count", 4], ["sku", "G-18"]]),
  assign: assign({sku: "G-17", count: 4}, {count: 5, note: "ready"}),
  pick: pick({sku: "G-17", count: 4, note: null}, ["sku", "note"]),
  omit: omit({sku: "G-17", count: 4, note: null}, ["note"]),
  hasOwnNull: hasOwn({note: null}, "note"),
  hasOwnMissing: hasOwn({note: null}, "Note"),
  parseInt: parseInt("  -2048px"),
  parseIntRadix: parseInt("+2fZ", 16),
  parseFloat: parseFloat("  12.75e2rest"),
  parseFloatMissing: parseFloat("no digits"),
  toFixed: toFixed(2.345, 2),
  toFixedWhole: toFixed(7, 2),
  clampLow: clamp(-4, -1, 9),
  clampHigh: clamp(12, -1, 9),
  truncPositive: trunc(3.9),
  truncNegative: trunc(-3.9),
  signNegative: sign(-8),
  signZero: sign(0),
  signPositive: sign(3),
  title: title("quiet HARBOR notes"),
  lstrip: lstrip("  ...draft  "),
  lstripChars: lstrip("xxcodexx", "x"),
  rstrip: rstrip("  ...draft  "),
  rstripChars: rstrip("xxcodexx", "x"),
  zfill: zfill("-42", 5),
  partition: partition("alpha/beta/gamma", "/"),
  rpartition: rpartition("alpha/beta/gamma", "/"),
  splitOnce: splitOnce("alpha/beta/gamma", "/"),
  rsplitOnce: rsplitOnce("alpha/beta/gamma", "/"),
  stripPrefix: stripPrefix("v3.4.1", "v"),
  stripPrefixMissing: stripPrefix("3.4.1", "v"),
  stripSuffix: stripSuffix("report.csv", ".csv"),
  stripSuffixMissing: stripSuffix("report.txt", ".csv"),
  identity: identity({status: "new"}),
  pipe: pipe([(n) -> n + 2, (n) -> n * 3, (n) -> n - 1])(4),
  compose: compose([(n) -> n + 2, (n) -> n * 3, (n) -> n - 1])(4),
  unwrapOr: unwrapOr(null, "default"),
  unwrapOrKeepsZero: unwrapOr(0, "default"),
  unwrapOrElse: unwrapOrElse(null, () -> 40 + 2),
  unwrapOrElseLazy: unwrapOrElse("set", () -> 1 / 0),
  mapOr: mapOr(6, 0, (n) -> n * 7),
  mapOrNull: mapOr(null, 0, (n) -> n * 7),
  andThen: andThen("  Tide  ", (text) -> trim(text)),
  andThenNull: andThen(null, (text) -> trim(text)),
  orElse: orElse(null, "replacement"),
  orElseKeeps: orElse(false, "replacement")
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/extra-common.dwl
```

## Expected output

```json
{
  "enumerate": [
    {
      "index": 1,
      "value": "map"
    },
    {
      "index": 2,
      "value": "compass"
    }
  ],
  "range": [
    2,
    4,
    6,
    8
  ],
  "sorted": [
    {
      "name": "hull",
      "rank": 1
    },
    {
      "name": "sail",
      "rank": 2
    }
  ],
  "reversed": [
    7,
    5,
    3
  ],
  "chunk": [
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
  "compact": [
    "kept",
    false,
    0
  ],
  "unique": [
    {
      "code": "a"
    },
    {
      "code": "b"
    }
  ],
  "fold": 19,
  "filterMap": [
    20,
    40
  ],
  "stepBy": [
    "zero",
    "two",
    "four"
  ],
  "position": 1,
  "rposition": 3,
  "dedup": [
    "north",
    "south",
    "north"
  ],
  "windows": [
    [
      10,
      20
    ],
    [
      20,
      30
    ],
    [
      30,
      40
    ]
  ],
  "chunksExact": [
    [
      1,
      2
    ],
    [
      3,
      4
    ]
  ],
  "binarySearch": {
    "found": true,
    "index": 2
  },
  "binaryInsert": {
    "found": false,
    "index": 2
  },
  "rotateLeft": [
    "b",
    "c",
    "d",
    "a"
  ],
  "rotateRight": [
    "d",
    "a",
    "b",
    "c"
  ],
  "swap": [
    "third",
    "second",
    "first"
  ],
  "fill": [
    0,
    9,
    9,
    0
  ],
  "truncate": [
    "a",
    "b"
  ],
  "fromEntries": {
    "sku": "G-18",
    "count": 4
  },
  "assign": {
    "sku": "G-17",
    "count": 5,
    "note": "ready"
  },
  "pick": {
    "sku": "G-17",
    "note": null
  },
  "omit": {
    "sku": "G-17",
    "count": 4
  },
  "hasOwnNull": true,
  "hasOwnMissing": false,
  "parseInt": -2048,
  "parseIntRadix": 47,
  "parseFloat": 1275,
  "parseFloatMissing": null,
  "toFixed": "2.35",
  "toFixedWhole": "7.00",
  "clampLow": -1,
  "clampHigh": 9,
  "truncPositive": 3,
  "truncNegative": -3,
  "signNegative": -1,
  "signZero": 0,
  "signPositive": 1,
  "title": "Quiet Harbor Notes",
  "lstrip": "...draft  ",
  "lstripChars": "codexx",
  "rstrip": "  ...draft",
  "rstripChars": "xxcode",
  "zfill": "-0042",
  "partition": [
    "alpha",
    "/",
    "beta/gamma"
  ],
  "rpartition": [
    "alpha/beta",
    "/",
    "gamma"
  ],
  "splitOnce": [
    "alpha",
    "beta/gamma"
  ],
  "rsplitOnce": [
    "alpha/beta",
    "gamma"
  ],
  "stripPrefix": "3.4.1",
  "stripPrefixMissing": null,
  "stripSuffix": "report",
  "stripSuffixMissing": null,
  "identity": {
    "status": "new"
  },
  "pipe": 17,
  "compose": 11,
  "unwrapOr": "default",
  "unwrapOrKeepsZero": 0,
  "unwrapOrElse": 42,
  "unwrapOrElseLazy": "set",
  "mapOr": 42,
  "mapOrNull": 0,
  "andThen": "Tide",
  "andThenNull": null,
  "orElse": "replacement",
  "orElseKeeps": false
}
```

## Functions shown

`enumerate`, `range`, `sorted`, `reversed`, `chunk`, `compact`, `unique`, `fromEntries`, `assign`, `pick`, `omit`, `hasOwn`, `parseInt`, `parseFloat`, `toFixed`, `clamp`, `trunc`, `sign`, `title`, `lstrip`, `rstrip`, `zfill`, `partition`, `rpartition`, `identity`, `pipe`, `compose`, `fold`, `filterMap`, `stepBy`, `position`, `rposition`, `dedup`, `windows`, `chunksExact`, `binarySearch`, `rotateLeft`, `rotateRight`, `swap`, `fill`, `truncate`, `splitOnce`, `rsplitOnce`, `stripPrefix`, `stripSuffix`, `unwrapOr`, `unwrapOrElse`, `mapOr`, `andThen`, `orElse`
