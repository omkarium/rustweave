// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the Arrays module.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
     { paid: payload.orders countBy $.paid, total: payload.orders sumBy $.total,        none: payload.empty countBy $.paid, zero: payload.empty sumBy $.total, n: payload.nothing sumBy $ }"####,
        r####"{
  "paid": 3,
  "total": 515.85,
  "none": 0,
  "zero": 0,
  "n": null
}"####,
    ),
    (
        r####"%dw 2.0
import countBy from dw::core::Arrays
---
[1, 2, 3, 4, 5] countBy (not ($ < 3))"####,
        r####"3"####,
    ),
    (
        r####"%dw 2.0
import countBy from dw::core::Arrays
---
[1, 2, 3, 4, 5] countBy ($ > 2)"####,
        r####"3"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
     { some: [1, 2, 3] some ($ == 2), every: [2, 4] every (($ mod 2) == 0),        take: [1, 2, 3, 4] take 2, drop: [1, 2, 3, 4] drop 2,        at: [10, 20, 30] splitAt 1, tw: [1, 2, 3, 1] takeWhile ($ < 3),        first: [1, 2, 3] firstWith ($ > 1) }"####,
        r####"{
  "some": true,
  "every": true,
  "take": [
    1,
    2
  ],
  "drop": [
    3,
    4
  ],
  "at": {
    "l": [
      10
    ],
    "r": [
      20,
      30
    ]
  },
  "tw": [
    1,
    2
  ],
  "first": 2
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
output application/json
---
     { append: [1, 2] << 3, remove: [1, 2, 1, 3] -- [1],        div: divideBy([1, 2, 3, 4, 5], 2),        where: splitWhere([1, 2, 3, 4], (x) -> x > 2) }"####,
        r####"{
  "append": [
    1,
    2,
    3
  ],
  "remove": [
    2,
    3
  ],
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
  "where": {
    "l": [
      1,
      2
    ],
    "r": [
      3,
      4
    ]
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Arrays
     var users = [{id: "1", name: "Mariano"}, {id: "2", name: "Leandro"}, {id: "3", name: "Julian"}]
     var products = [{ownerId: "1", name: "DataWeave"}, {ownerId: "1", name: "BAT"}, {ownerId: "3", name: "DataSense"}, {ownerId: "4", name: "SmartConnectors"}]
     output application/json
---
     { j: join(users, products, (u) -> u.id, (p) -> p.ownerId),        lj: leftJoin(users, products, (u) -> u.id, (p) -> p.ownerId),        oj: outerJoin(users, products, (u) -> u.id, (p) -> p.ownerId) }"####,
        r####"{
  "j": [
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "DataWeave"
      }
    },
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "BAT"
      }
    },
    {
      "l": {
        "id": "3",
        "name": "Julian"
      },
      "r": {
        "ownerId": "3",
        "name": "DataSense"
      }
    }
  ],
  "lj": [
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "DataWeave"
      }
    },
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "BAT"
      }
    },
    {
      "l": {
        "id": "2",
        "name": "Leandro"
      }
    },
    {
      "l": {
        "id": "3",
        "name": "Julian"
      },
      "r": {
        "ownerId": "3",
        "name": "DataSense"
      }
    }
  ],
  "oj": [
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "DataWeave"
      }
    },
    {
      "l": {
        "id": "1",
        "name": "Mariano"
      },
      "r": {
        "ownerId": "1",
        "name": "BAT"
      }
    },
    {
      "l": {
        "id": "2",
        "name": "Leandro"
      }
    },
    {
      "l": {
        "id": "3",
        "name": "Julian"
      },
      "r": {
        "ownerId": "3",
        "name": "DataSense"
      }
    },
    {
      "r": {
        "ownerId": "4",
        "name": "SmartConnectors"
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
{ take: ([1,2,3] take 5), drop: ([1,2,3] drop 5), slice: slice([1,2,3,4],1,3), iw: ([1,2,3] indexWhere ($ > 1)), dw: ([1,2,3] dropWhile ($ < 3)), p: ([1,2,3,4] partition (($ mod 2) == 0)), fw: ([1,2,3] firstWith ($ > 1)), div: divideBy([1,2,3,4,5],2), sw: splitWhere([1,2,3,4], (x) -> x>2), none: ([1,2,3] indexWhere ($ > 9)) }"####,
        r####"{
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
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
