// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for Core.

const SAMPLES: &str = r####"{
  "events": [
    { "seq": 10, "site": "ridge", "celsius": 21.5, "ok": true },
    { "seq": 11, "site": "delta", "celsius": 18, "ok": false },
    { "seq": 12, "site": "ridge", "celsius": 3.25, "ok": true },
    { "seq": 13, "site": "harbor", "celsius": 42.75, "ok": true }
  ],
  "blank": [],
  "absent": null
}"####;

fn run(script: &str) -> String {
    rustweave_core::run(script, &[("samples", SAMPLES)]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"samples.events map $.seq"####,
        r####"[
  10,
  11,
  12,
  13
]"####,
    ),
    (
        r####"samples.events map (e, i) -> { position: i, site: e.site }"####,
        r####"[
  {
    "position": 0,
    "site": "ridge"
  },
  {
    "position": 1,
    "site": "delta"
  },
  {
    "position": 2,
    "site": "ridge"
  },
  {
    "position": 3,
    "site": "harbor"
  }
]"####,
    ),
    (
        r####"samples.events filter $.ok map $.site"####,
        r####"[
  "ridge",
  "ridge",
  "harbor"
]"####,
    ),
    (
        r####"samples.events filter (e) -> e.celsius > 20"####,
        r####"[
  {
    "seq": 10,
    "site": "ridge",
    "celsius": 21.5,
    "ok": true
  },
  {
    "seq": 13,
    "site": "harbor",
    "celsius": 42.75,
    "ok": true
  }
]"####,
    ),
    (
        r####"{ a: sizeOf(samples.events), b: sizeOf(samples.blank), c: sizeOf(samples.absent) }"####,
        r####"{
  "a": 4,
  "b": 0,
  "c": null
}"####,
    ),
    (
        r####"{ a: isEmpty(samples.events), b: isEmpty(samples.blank), c: isEmpty(samples.absent) }"####,
        r####"{
  "a": false,
  "b": true,
  "c": true
}"####,
    ),
    (r####"samples.absent map $"####, r####"null"####),
    (
        r####"samples.events.celsius reduce (t, acc) -> t + acc"####,
        r####"85.5"####,
    ),
    (
        r####"samples.events.celsius reduce (t, acc = 50) -> t + acc"####,
        r####"135.5"####,
    ),
    (r####"[4, 8, 15, 16] reduce ($ + $$)"####, r####"43"####),
    (
        r####"samples.events reduce (e, acc = []) -> acc ++ [e.site]"####,
        r####"[
  "ridge",
  "delta",
  "ridge",
  "harbor"
]"####,
    ),
    (r####"samples.blank reduce ($ + $$)"####, r####"null"####),
    (r####"samples.absent reduce ($ + $$)"####, r####"null"####),
    (r####"sum(samples.events.celsius)"####, r####"85.5"####),
    (r####"sum([])"####, r####"0"####),
    (
        r####"samples.events groupBy $.site"####,
        r####"{
  "ridge": [
    {
      "seq": 10,
      "site": "ridge",
      "celsius": 21.5,
      "ok": true
    },
    {
      "seq": 12,
      "site": "ridge",
      "celsius": 3.25,
      "ok": true
    }
  ],
  "delta": [
    {
      "seq": 11,
      "site": "delta",
      "celsius": 18,
      "ok": false
    }
  ],
  "harbor": [
    {
      "seq": 13,
      "site": "harbor",
      "celsius": 42.75,
      "ok": true
    }
  ]
}"####,
    ),
    (
        r####"samples.events groupBy (e, i) -> e.ok"####,
        r####"{
  "true": [
    {
      "seq": 10,
      "site": "ridge",
      "celsius": 21.5,
      "ok": true
    },
    {
      "seq": 12,
      "site": "ridge",
      "celsius": 3.25,
      "ok": true
    },
    {
      "seq": 13,
      "site": "harbor",
      "celsius": 42.75,
      "ok": true
    }
  ],
  "false": [
    {
      "seq": 11,
      "site": "delta",
      "celsius": 18,
      "ok": false
    }
  ]
}"####,
    ),
    (
        r####"[3, 3.0, 5, 5.50] groupBy $"####,
        r####"{
  "3": [
    3,
    3
  ],
  "5": [
    5
  ],
  "5.5": [
    5.5
  ]
}"####,
    ),
    (
        r####"(samples.events groupBy $.site).ridge.seq"####,
        r####"[
  10,
  12
]"####,
    ),
    (
        r####"flatten([[5, 6], [7], 8])"####,
        r####"[
  5,
  6,
  7,
  8
]"####,
    ),
    (
        r####"[5, 6, 6, 7] distinctBy $"####,
        r####"[
  5,
  6,
  7
]"####,
    ),
    (
        r####"[7, 5, 6] orderBy $"####,
        r####"[
  5,
  6,
  7
]"####,
    ),
    (
        r####"[5, 6, 7, 6] find 6"####,
        r####"[
  1,
  3
]"####,
    ),
    (r####"[5, 6, 7] contains 6"####, r####"true"####),
    (r####"["x", "y"] joinBy "-""####, r####""x-y""####),
    (r####"5 then ($ + 10)"####, r####"15"####),
    (r####"null then ($ + 1)"####, r####"null"####),
    (
        r####"[3, 4] flatMap (n) -> [n, n]"####,
        r####"[
  3,
  3,
  4,
  4
]"####,
    ),
    (
        r####"zip([5, 6], ["p", "q"])"####,
        r####"[
  [
    5,
    "p"
  ],
  [
    6,
    "q"
  ]
]"####,
    ),
    (r####"[7, 8, 7] indexOf 7"####, r####"0"####),
    (r####"[7, 8, 7] lastIndexOf 7"####, r####"2"####),
    (
        r####"{ u: unzip([[10, 20, 30], [40, 50, 60]]), n: [null onNull () -> "none", "y" onNull () -> "none"] }"####,
        r####"{
  "u": [
    [
      10,
      40
    ],
    [
      20,
      50
    ],
    [
      30,
      60
    ]
  ],
  "n": [
    "none",
    "y"
  ]
}"####,
    ),
    (
        r####"output application/json
---
{ s: sizeOf("abcd"), o: sizeOf({x: 1, y: 2, z: 3}), n: sizeOf(456), es: isEmpty(""), eo: isEmpty({}), io: indexOf("abracadabra", "cad"), lio: lastIndexOf("abracadabra", "ab"), then: 7 then ($ * 2), on: null onNull () -> "fallback", blank: [isBlank(null), isBlank("  "), isBlank("x")], minBy: [{v: 9}, {v: 4}] minBy $.v, maxBy: [{v: 9}, {v: 4}] maxBy $.v, avg: avg([2, 4, 6, 8]) }"####,
        r####"{
  "s": 4,
  "o": 3,
  "n": 3,
  "es": true,
  "eo": true,
  "io": 4,
  "lio": 7,
  "then": 14,
  "on": "fallback",
  "blank": [
    true,
    true,
    false
  ],
  "minBy": {
    "v": 4
  },
  "maxBy": {
    "v": 9
  },
  "avg": 5
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
