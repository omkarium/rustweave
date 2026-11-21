# Core functions in one script

A single crew-list pipeline plus a second set of fields that exercises the rest of the Core surface. Random, UUID, and clock helpers are shown as stable checks instead of unstable values.

This script uses all 74 functions listed for `dw::Core`.

Run from the repository root because `readUrl` reads `examples/data/orders.json` through a local file URL. Log helpers also write their values to standard error while returning them.

## Script

```dataweave
%dw 2.0
output application/json
---
do {
  var crew = [
    {name: "Nora", team: "pilot", score: 7, active: true},
    {name: "Silas", team: "pilot", score: 4, active: false},
    {name: "Ivy", team: "nav", score: 9, active: true},
    {name: "Rook", team: "nav", score: 6, active: true}
  ]
  var grouped = crew groupBy $.team
  var r = random()
  var n = randomInt(10)
  ---
  {
    map: crew map {name: upper($.name), team: $.team},
    filter: crew filter $.active map $.name,
    reduce: crew reduce (item, acc = 0) -> acc + item.score,
    groupBy: grouped pluck $$,
    groupSizes: (grouped pluck $) map (items) -> sizeOf(items),
    sizeOf: [sizeOf(crew), sizeOf("harbor"), sizeOf({a: 1, b: 2})],
    isEmpty: [isEmpty([]), isEmpty(""), isEmpty({}), isEmpty(null)],
    sum: sum(crew map $.score),
    mapObject: {a: 1, b: 2} mapObject { ($$): $ + 10 },
    pluckKeys: {alpha: 10, beta: 20} pluck $$,
    pluckValues: {alpha: 10, beta: 20} pluck $,
    filterObject: {a: 1, b: 2, c: 3} filterObject $ > 1,
    flatMap: crew flatMap (item) -> [item.name, item.team],
    entriesOf: entriesOf({a: 1}),
    namesOf: namesOf({a: 1, b: 2}),
    keysOf: keysOf({a: 1, b: 2}),
    valuesOf: valuesOf({a: 1, b: 2}),
    flatten: flatten([[1, 2], [3], 4]),
    distinctBy: [1, 1.0, 2, 3] distinctBy $,
    orderBy: [3, 1, 2] orderBy $,
    find: "a1b2a3" find /a[0-9]/,
    indexOf: [indexOf("harbor", "rb"), indexOf([4, 5, 6], 5)],
    lastIndexOf: [lastIndexOf("harbor", "r"), lastIndexOf([4, 5, 4], 4)],
    contains: [contains("harbor", "bor"), contains([1, 2], 2), contains({a: 1}, "a")],
    to: 2 to 4,
    zip: zip([1, 2], ["a", "b"]),
    unzip: unzip([[1, "a"], [2, "b"]]),
    upperLower: [upper("Tide"), lower("TIDE")],
    trim: trim("  tide  "),
    startsEnds: [startsWith("harbor", "har"), endsWith("harbor", "bor")],
    splitBy: "a1b2" splitBy /[0-9]/,
    with: with({a: 1}, {b: 2}),
    replace: ["pepper" replace "p" with "P", "x7y8" replace /[0-9]/ with "Z"],
    joinBy: ["north", "south"] joinBy "-",
    absCeil: [abs(-3.5), ceil(2.1)],
    floorRound: [floor(2.9), round(2.5)],
    powSqrtMod: [pow(2, 5), sqrt(49), mod(17, 5)],
    numberChecks: [isInteger(2.0), isDecimal(2.5), isOdd(5), isEven(-6)],
    blank: [isBlank(null), isBlank("  "), isBlank("x")],
    minMax: [min([4, 2, 9]), max([4, 2, 9])],
    by: [([{v: 9}, {v: 4}] minBy $.v).v, ([{v: 9}, {v: 4}] maxBy $.v).v],
    avg: avg([2, 4, 9]),
    then: 5 then ($ * 2),
    onNull: null onNull () -> "fallback",
    matches: ["b7" matches /[a-z][0-9]/, "abc" matches /b/],
    match: "x7" match /([a-z])([0-9])/,
    scan: "a1b2" scan /([a-z])([0-9])/,
    typeOf: [typeOf(1), typeOf("x"), typeOf([]), typeOf({}), typeOf(null)],
    random: r >= 0 and r < 1,
    randomInt: n >= 0 and n < 10,
    uuid: sizeOf(uuid()) == 36,
    log: [
      log(7, "core"),
      logDebug("debug", 6),
      logInfo("info", 5),
      logWarn("warn", 4),
      logError("error", 3),
      logWith("WARN", "with", 2)
    ],
    dates: [daysBetween(|2020-02-29|, |2021-03-01|), isLeapYear(|2020-01-01|)],
    now: typeOf(now()),
    readWrite: [
      read("{\"a\":1}", "application/json").a,
      write({a: 1}, "application/json", {indent: false})
    ],
    readUrl: readUrl("file://examples/data/orders.json", "application/json").orders[0].id,
    plusMinus: [plusPlus([1, 2], [3]), minusMinus([1, 2, 1], [1])]
  }
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/core.dwl
```

## Expected output

```json
{
  "map": [
    {
      "name": "NORA",
      "team": "pilot"
    },
    {
      "name": "SILAS",
      "team": "pilot"
    },
    {
      "name": "IVY",
      "team": "nav"
    },
    {
      "name": "ROOK",
      "team": "nav"
    }
  ],
  "filter": [
    "Nora",
    "Ivy",
    "Rook"
  ],
  "reduce": 26,
  "groupBy": [
    "pilot",
    "nav"
  ],
  "groupSizes": [
    2,
    2
  ],
  "sizeOf": [
    4,
    6,
    2
  ],
  "isEmpty": [
    true,
    true,
    true,
    true
  ],
  "sum": 26,
  "mapObject": {
    "a": 11,
    "b": 12
  },
  "pluckKeys": [
    "alpha",
    "beta"
  ],
  "pluckValues": [
    10,
    20
  ],
  "filterObject": {
    "b": 2,
    "c": 3
  },
  "flatMap": [
    "Nora",
    "pilot",
    "Silas",
    "pilot",
    "Ivy",
    "nav",
    "Rook",
    "nav"
  ],
  "entriesOf": [
    {
      "key": "a",
      "value": 1,
      "attributes": {
        
      }
    }
  ],
  "namesOf": [
    "a",
    "b"
  ],
  "keysOf": [
    "a",
    "b"
  ],
  "valuesOf": [
    1,
    2
  ],
  "flatten": [
    1,
    2,
    3,
    4
  ],
  "distinctBy": [
    1,
    2,
    3
  ],
  "orderBy": [
    1,
    2,
    3
  ],
  "find": [
    [
      0
    ],
    [
      4
    ]
  ],
  "indexOf": [
    2,
    1
  ],
  "lastIndexOf": [
    5,
    2
  ],
  "contains": [
    true,
    true,
    true
  ],
  "to": [
    2,
    3,
    4
  ],
  "zip": [
    [
      1,
      "a"
    ],
    [
      2,
      "b"
    ]
  ],
  "unzip": [
    [
      1,
      2
    ],
    [
      "a",
      "b"
    ]
  ],
  "upperLower": [
    "TIDE",
    "tide"
  ],
  "trim": "tide",
  "startsEnds": [
    true,
    true
  ],
  "splitBy": [
    "a",
    "b"
  ],
  "with": {
    "a": 1,
    "b": 2
  },
  "replace": [
    "PePPer",
    "xZyZ"
  ],
  "joinBy": "north-south",
  "absCeil": [
    3.5,
    3
  ],
  "floorRound": [
    2,
    3
  ],
  "powSqrtMod": [
    32,
    7,
    2
  ],
  "numberChecks": [
    true,
    true,
    true,
    true
  ],
  "blank": [
    true,
    true,
    false
  ],
  "minMax": [
    2,
    9
  ],
  "by": [
    4,
    9
  ],
  "avg": 5,
  "then": 10,
  "onNull": "fallback",
  "matches": [
    true,
    false
  ],
  "match": [
    "x7",
    "x",
    "7"
  ],
  "scan": [
    [
      "a1",
      "a",
      "1"
    ],
    [
      "b2",
      "b",
      "2"
    ]
  ],
  "typeOf": [
    "Number",
    "String",
    "Array",
    "Object",
    "Null"
  ],
  "random": true,
  "randomInt": true,
  "uuid": true,
  "log": [
    7,
    6,
    5,
    4,
    3,
    2
  ],
  "dates": [
    366,
    true
  ],
  "now": "DateTime",
  "readWrite": [
    1,
    "{\"a\": 1}"
  ],
  "readUrl": "A-104",
  "plusMinus": [
    [
      1,
      2,
      3
    ],
    [
      2
    ]
  ]
}
```

## Functions shown

`map`, `filter`, `reduce`, `groupBy`, `sizeOf`, `isEmpty`, `sum`, `mapObject`, `pluck`, `filterObject`, `flatMap`, `entriesOf`, `namesOf`, `keysOf`, `valuesOf`, `flatten`, `distinctBy`, `orderBy`, `find`, `indexOf`, `lastIndexOf`, `contains`, `to`, `zip`, `unzip`, `upper`, `lower`, `trim`, `startsWith`, `endsWith`, `splitBy`, `with`, `replace`, `joinBy`, `abs`, `ceil`, `floor`, `round`, `pow`, `sqrt`, `mod`, `isInteger`, `isDecimal`, `isOdd`, `isEven`, `isBlank`, `minBy`, `maxBy`, `max`, `min`, `avg`, `then`, `onNull`, `matches`, `match`, `scan`, `typeOf`, `random`, `randomInt`, `uuid`, `log`, `daysBetween`, `isLeapYear`, `now`, `read`, `write`, `readUrl`, `logDebug`, `logInfo`, `logWarn`, `logError`, `logWith`, `plusPlus`, `minusMinus`
