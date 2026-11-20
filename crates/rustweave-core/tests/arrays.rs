// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Arrays module.

const STOCK: &str = r####"{
  "items": [
    { "sku": "r1", "qty": 7, "open": true },
    { "sku": "r2", "qty": 2, "open": false },
    { "sku": "r3", "qty": 11, "open": true }
  ],
  "empty": [],
  "none": null
}"####;

fn run(script: &str) -> String {
    rustweave_core::run(script, &[("stock", STOCK)]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{ open: stock.items countBy $.open, total: stock.items sumBy $.qty, none: stock.empty countBy $.open, zero: stock.empty sumBy $.qty, n: stock.none sumBy $ }"####,
        r####"{
  "open": 2,
  "total": 20,
  "none": 0,
  "zero": 0,
  "n": null
}"####,
    ),
    (
        r####"%dw 2.0
import countBy from dw::core::Arrays
---
[2, 4, 6, 8] countBy ($ >= 4)"####,
        r####"3"####,
    ),
    (
        r####"%dw 2.0
import countBy from dw::core::Arrays
---
[1, 2, 4, 6] countBy ($ > 3)"####,
        r####"2"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{ some: [5, 7, 9] some ($ == 7), every: [6, 9] every (($ mod 3) == 0), take: [4, 5, 6, 7] take 3, drop: [4, 5, 6, 7] drop 3, at: [14, 15, 16] splitAt 2, tw: [2, 4, 6, 5] takeWhile ($ < 6), first: [2, 4, 6] firstWith ($ > 3) }"####,
        r####"{
  "some": true,
  "every": true,
  "take": [
    4,
    5,
    6
  ],
  "drop": [
    7
  ],
  "at": {
    "l": [
      14,
      15
    ],
    "r": [
      16
    ]
  },
  "tw": [
    2,
    4
  ],
  "first": 4
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{ append: [4, 5] << 6, remove: [2, 4, 2, 6] -- [2], div: divideBy([2, 4, 6, 8, 10], 3), where: splitWhere([2, 4, 6, 8], (x) -> x > 4) }"####,
        r####"{
  "append": [
    4,
    5,
    6
  ],
  "remove": [
    4,
    6
  ],
  "div": [
    [
      2,
      4,
      6
    ],
    [
      8,
      10
    ]
  ],
  "where": {
    "l": [
      2,
      4
    ],
    "r": [
      6,
      8
    ]
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
     var people = [{key: "p1", label: "Nora"}, {key: "p2", label: "Silas"}, {key: "p3", label: "Ivy"}]
     var tasks = [{owner: "p1", title: "Tide chart"}, {owner: "p1", title: "Soil grid"}, {owner: "p3", title: "Wind map"}, {owner: "p9", title: "Unassigned"}]
     output application/json
---
     { j: join(people, tasks, (p) -> p.key, (t) -> t.owner), lj: leftJoin(people, tasks, (p) -> p.key, (t) -> t.owner), oj: outerJoin(people, tasks, (p) -> p.key, (t) -> t.owner) }"####,
        r####"{
  "j": [
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Tide chart"
      }
    },
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Soil grid"
      }
    },
    {
      "l": {
        "key": "p3",
        "label": "Ivy"
      },
      "r": {
        "owner": "p3",
        "title": "Wind map"
      }
    }
  ],
  "lj": [
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Tide chart"
      }
    },
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Soil grid"
      }
    },
    {
      "l": {
        "key": "p2",
        "label": "Silas"
      }
    },
    {
      "l": {
        "key": "p3",
        "label": "Ivy"
      },
      "r": {
        "owner": "p3",
        "title": "Wind map"
      }
    }
  ],
  "oj": [
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Tide chart"
      }
    },
    {
      "l": {
        "key": "p1",
        "label": "Nora"
      },
      "r": {
        "owner": "p1",
        "title": "Soil grid"
      }
    },
    {
      "l": {
        "key": "p2",
        "label": "Silas"
      }
    },
    {
      "l": {
        "key": "p3",
        "label": "Ivy"
      },
      "r": {
        "owner": "p3",
        "title": "Wind map"
      }
    },
    {
      "r": {
        "owner": "p9",
        "title": "Unassigned"
      }
    }
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{ take: ([5, 6, 7] take 5), drop: ([5, 6, 7] drop 5), slice: slice([10, 20, 30, 40, 50], 1, 4), iw: ([2, 4, 6] indexWhere ($ > 3)), dw: ([2, 4, 6] dropWhile ($ < 6)), p: ([2, 4, 6, 8] partition (($ mod 4) == 0)), fw: ([2, 4, 6] firstWith ($ > 2)), div: divideBy([2, 4, 6, 8, 10], 2), sw: splitWhere([2, 4, 6, 8], (x) -> x > 4), none: ([2, 4, 6] indexWhere ($ > 20)) }"####,
        r####"{
  "take": [
    5,
    6,
    7
  ],
  "drop": [
    
  ],
  "slice": [
    20,
    30,
    40
  ],
  "iw": 1,
  "dw": [
    6
  ],
  "p": {
    "success": [
      4,
      8
    ],
    "failure": [
      2,
      6
    ]
  },
  "fw": 4,
  "div": [
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
  "sw": {
    "l": [
      2,
      4
    ],
    "r": [
      6,
      8
    ]
  },
  "none": -1
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
