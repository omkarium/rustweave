# Arrays functions in one script

One inventory and task list is queried, split, grouped, and joined. The result shows the different shapes returned by counting, slicing, partitioning, and the three join styles.

This script uses all 19 functions listed for `dw::core::Arrays`.

Import `dw::core::Arrays` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Arrays
output application/json
---
do {
  var orders = [
    {sku: "r1", qty: 7, active: true},
    {sku: "r2", qty: 2, active: false},
    {sku: "r3", qty: 11, active: true},
    {sku: "r4", qty: 4, active: true}
  ]
  var people = [
    {key: "p1", label: "Nora"},
    {key: "p2", label: "Silas"},
    {key: "p3", label: "Ivy"}
  ]
  var tasks = [
    {owner: "p1", title: "Tide chart"},
    {owner: "p1", title: "Soil grid"},
    {owner: "p3", title: "Wind map"},
    {owner: "p9", title: "Unassigned"}
  ]
  ---
  {
    countBy: orders countBy $.active,
    sumBy: orders sumBy $.qty,
    some: orders some ($.qty > 10),
    every: orders every ($.qty > 0),
    take: orders take 2 map $.sku,
    drop: orders drop 2 map $.sku,
    slice: slice([10, 20, 30, 40, 50], 1, 4),
    indexOf: indexOf(["r1", "r2", "r3"], "r2"),
    indexWhere: orders indexWhere ($.qty >= 10),
    splitAt: do {
      var parts = orders splitAt 2
      ---
      {left: parts.l map $.sku, right: parts.r map $.sku}
    },
    takeWhile: orders takeWhile ($.qty < 10) map $.sku,
    dropWhile: orders dropWhile ($.qty < 10) map $.sku,
    partition: orders partition ($.active),
    firstWith: (orders firstWith ($.qty > 10)).sku,
    divideBy: divideBy([1, 2, 3, 4, 5, 6, 7], 3),
    splitWhere: splitWhere([1, 2, 3, 4, 5], (x) -> x > 3),
    join: join(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> pair.l.label ++ ": " ++ pair.r.title,
    leftJoin: leftJoin(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> pair.l.label,
    outerJoin: outerJoin(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> if (pair.l == null) "unassigned" else pair.l.label
  }
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/arrays.dwl
```

## Expected output

```json
{
  "countBy": 3,
  "sumBy": 24,
  "some": true,
  "every": true,
  "take": [
    "r1",
    "r2"
  ],
  "drop": [
    "r3",
    "r4"
  ],
  "slice": [
    20,
    30,
    40
  ],
  "indexOf": 1,
  "indexWhere": 2,
  "splitAt": {
    "left": [
      "r1",
      "r2"
    ],
    "right": [
      "r3",
      "r4"
    ]
  },
  "takeWhile": [
    "r1",
    "r2"
  ],
  "dropWhile": [
    "r3",
    "r4"
  ],
  "partition": {
    "success": [
      {
        "sku": "r1",
        "qty": 7,
        "active": true
      },
      {
        "sku": "r3",
        "qty": 11,
        "active": true
      },
      {
        "sku": "r4",
        "qty": 4,
        "active": true
      }
    ],
    "failure": [
      {
        "sku": "r2",
        "qty": 2,
        "active": false
      }
    ]
  },
  "firstWith": "r3",
  "divideBy": [
    [
      1,
      2,
      3
    ],
    [
      4,
      5,
      6
    ],
    [
      7
    ]
  ],
  "splitWhere": {
    "l": [
      1,
      2,
      3
    ],
    "r": [
      4,
      5
    ]
  },
  "join": [
    "Nora: Tide chart",
    "Nora: Soil grid",
    "Ivy: Wind map"
  ],
  "leftJoin": [
    "Nora",
    "Nora",
    "Silas",
    "Ivy"
  ],
  "outerJoin": [
    "Nora",
    "Nora",
    "Silas",
    "Ivy",
    "unassigned"
  ]
}
```

## Functions shown

`countBy`, `sumBy`, `some`, `every`, `take`, `drop`, `slice`, `indexOf`, `indexWhere`, `splitAt`, `takeWhile`, `dropWhile`, `partition`, `firstWith`, `divideBy`, `splitWhere`, `join`, `leftJoin`, `outerJoin`
