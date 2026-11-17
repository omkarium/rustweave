// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for Core.

mod common;

const CASES: &[(&str, &str)] = &[
    (
        r####"payload.orders map $.id"####,
        r####"[
  1,
  2,
  3,
  4
]"####,
    ),
    (
        r####"payload.orders map (o, i) -> { n: i, who: o.customer }"####,
        r####"[
  {
    "n": 0,
    "who": "ana"
  },
  {
    "n": 1,
    "who": "ben"
  },
  {
    "n": 2,
    "who": "ana"
  },
  {
    "n": 3,
    "who": "cara"
  }
]"####,
    ),
    (
        r####"payload.orders filter $.paid map $.customer"####,
        r####"[
  "ana",
  "ana",
  "cara"
]"####,
    ),
    (
        r####"payload.orders filter (o) -> o.total > 100"####,
        r####"[
  {
    "id": 1,
    "customer": "ana",
    "region": "north",
    "total": 120.5,
    "paid": true
  },
  {
    "id": 4,
    "customer": "cara",
    "region": "east",
    "total": 300.1,
    "paid": true
  }
]"####,
    ),
    (
        r####"{ a: sizeOf(payload.orders), b: sizeOf(payload.empty), c: sizeOf(payload.nothing) }"####,
        r####"{
  "a": 4,
  "b": 0,
  "c": null
}"####,
    ),
    (
        r####"{ a: isEmpty(payload.orders), b: isEmpty(payload.empty), c: isEmpty(payload.nothing) }"####,
        r####"{
  "a": false,
  "b": true,
  "c": true
}"####,
    ),
    (r####"payload.nothing map $"####, r####"null"####),
    (
        r####"payload.orders.total reduce (t, acc) -> t + acc"####,
        r####"515.85"####,
    ),
    (
        r####"payload.orders.total reduce (t, acc = 100) -> t + acc"####,
        r####"615.85"####,
    ),
    (r####"[1, 2, 3, 4] reduce ($ + $$)"####, r####"10"####),
    (
        r####"payload.orders reduce (o, acc = []) -> acc ++ [o.customer]"####,
        r####"[
  "ana",
  "ben",
  "ana",
  "cara"
]"####,
    ),
    (r####"payload.empty reduce ($ + $$)"####, r####"null"####),
    (r####"payload.nothing reduce ($ + $$)"####, r####"null"####),
    (r####"sum(payload.orders.total)"####, r####"515.85"####),
    (r####"sum([])"####, r####"0"####),
    (
        r####"payload.orders groupBy $.region"####,
        r####"{
  "north": [
    {
      "id": 1,
      "customer": "ana",
      "region": "north",
      "total": 120.5,
      "paid": true
    },
    {
      "id": 3,
      "customer": "ana",
      "region": "north",
      "total": 15.25,
      "paid": true
    }
  ],
  "south": [
    {
      "id": 2,
      "customer": "ben",
      "region": "south",
      "total": 80,
      "paid": false
    }
  ],
  "east": [
    {
      "id": 4,
      "customer": "cara",
      "region": "east",
      "total": 300.1,
      "paid": true
    }
  ]
}"####,
    ),
    (
        r####"payload.orders groupBy (o, i) -> o.paid"####,
        r####"{
  "true": [
    {
      "id": 1,
      "customer": "ana",
      "region": "north",
      "total": 120.5,
      "paid": true
    },
    {
      "id": 3,
      "customer": "ana",
      "region": "north",
      "total": 15.25,
      "paid": true
    },
    {
      "id": 4,
      "customer": "cara",
      "region": "east",
      "total": 300.1,
      "paid": true
    }
  ],
  "false": [
    {
      "id": 2,
      "customer": "ben",
      "region": "south",
      "total": 80,
      "paid": false
    }
  ]
}"####,
    ),
    (
        r####"[1, 1.0, 2, 2.50] groupBy $"####,
        r####"{
  "1": [
    1,
    1
  ],
  "2": [
    2
  ],
  "2.5": [
    2.5
  ]
}"####,
    ),
    (
        r####"(payload.orders groupBy $.customer).ana.id"####,
        r####"[
  1,
  3
]"####,
    ),
    (
        r####"flatten([[1, 2], [3], 4])"####,
        r####"[
  1,
  2,
  3,
  4
]"####,
    ),
    (
        r####"[1, 2, 2, 3] distinctBy $"####,
        r####"[
  1,
  2,
  3
]"####,
    ),
    (
        r####"[3, 1, 2] orderBy $"####,
        r####"[
  1,
  2,
  3
]"####,
    ),
    (
        r####"[1, 2, 3, 2] find 2"####,
        r####"[
  1,
  3
]"####,
    ),
    (r####"[1, 2, 3] contains 2"####, r####"true"####),
    (r####"["a", "b"] joinBy ",""####, r####""a,b""####),
    (r####"1 then ($ + 10)"####, r####"11"####),
    (r####"null then ($ + 1)"####, r####"null"####),
    (
        r####"[1, 2] flatMap (n) -> [n, n]"####,
        r####"[
  1,
  1,
  2,
  2
]"####,
    ),
    (
        r####"zip([1, 2], ["a", "b"])"####,
        r####"[
  [
    1,
    "a"
  ],
  [
    2,
    "b"
  ]
]"####,
    ),
    (r####"[1, 2, 1] indexOf 1"####, r####"0"####),
    (r####"[1, 2, 1] lastIndexOf 1"####, r####"2"####),
    (
        r####"{ u: unzip([[1, 2, 3], [4, 5, 6]]), n: [null onNull () -> "was null", "x" onNull () -> "was null"] }"####,
        r####"{
  "u": [
    [
      1,
      4
    ],
    [
      2,
      5
    ],
    [
      3,
      6
    ]
  ],
  "n": [
    "was null",
    "x"
  ]
}"####,
    ),
    (
        r####"output application/json
---
{ s: sizeOf("abc"), o: sizeOf({a:1,b:2}), n: sizeOf(123), es: isEmpty(""), eo: isEmpty({}), io: indexOf("abcabc","bc"), lio: lastIndexOf("abcabc","bc"), then: 5 then ($+1), on: null onNull () -> "x", blank: [isBlank(null), isBlank("  "), isBlank("x")], minBy: [{n:2},{n:1}] minBy $.n, maxBy: [{n:2},{n:1}] maxBy $.n, avg: avg([1,2,3]) }"####,
        r####"{
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
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(common::run(script), *expected, "script: {script}");
    }
}
