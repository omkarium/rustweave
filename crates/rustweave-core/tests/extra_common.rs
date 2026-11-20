// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the `dw::extra::Common` extension module.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[])
}

#[test]
fn enumerate_and_range() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  negativeStart: enumerate(["ash", "birch"], -2),
  empty: enumerate([]),
  forward: range(4),
  sliced: range(2, 6),
  descending: range(5, 0, -2),
  mismatch: range(0, 5, -1),
  single: range(3, 3)
}"####;
    let expected = r####"{
  "negativeStart": [
    {
      "index": -2,
      "value": "ash"
    },
    {
      "index": -1,
      "value": "birch"
    }
  ],
  "empty": [
    
  ],
  "forward": [
    0,
    1,
    2,
    3
  ],
  "sliced": [
    2,
    3,
    4,
    5
  ],
  "descending": [
    5,
    3,
    1
  ],
  "mismatch": [
    
  ],
  "single": [
    
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn sorted_and_reversed() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  plain: sorted([3, 1, 2]),
  nullsAsc: sorted([3, null, 1]),
  nullsDesc: sorted([3, null, 1], (x) -> x, true),
  keyed: sorted([{n: 2, id: "a"}, {n: 1, id: "b"}, {n: 2, id: "c"}], (item) -> item.n),
  strings: sorted(["pear", "apple", "fig"]),
  booleans: sorted([true, false, true]),
  reversed: reversed([4, 5, 6]),
  reversedEmpty: reversed([])
}"####;
    let expected = r####"{
  "plain": [
    1,
    2,
    3
  ],
  "nullsAsc": [
    1,
    3,
    null
  ],
  "nullsDesc": [
    null,
    3,
    1
  ],
  "keyed": [
    {
      "n": 1,
      "id": "b"
    },
    {
      "n": 2,
      "id": "a"
    },
    {
      "n": 2,
      "id": "c"
    }
  ],
  "strings": [
    "apple",
    "fig",
    "pear"
  ],
  "booleans": [
    false,
    true,
    true
  ],
  "reversed": [
    6,
    5,
    4
  ],
  "reversedEmpty": [
    
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn chunk_compact_unique() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  chunks: chunk([1, 2, 3, 4, 5], 2),
  exact: chunk([1, 2, 3, 4], 2),
  empty: chunk([], 2),
  compact: compact([null, false, 0, "", [], {}, null, 3]),
  unique: unique([1, 1.0, 2, 2.00, 3]),
  uniqueObjects: unique([{a: 1}, {a: 1.0}, {b: 2}]),
  uniqueArrays: unique([[1], [1.0], [2]]),
  uniqueEmpty: unique([])
}"####;
    let expected = r####"{
  "chunks": [
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
  "exact": [
    [
      1,
      2
    ],
    [
      3,
      4
    ]
  ],
  "empty": [
    
  ],
  "compact": [
    false,
    0,
    "",
    [
      
    ],
    {
      
    },
    3
  ],
  "unique": [
    1,
    2,
    3
  ],
  "uniqueObjects": [
    {
      "a": 1
    },
    {
      "b": 2
    }
  ],
  "uniqueArrays": [
    [
      1
    ],
    [
      2
    ]
  ],
  "uniqueEmpty": [
    
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn array_argument_errors() {
    for script in [
        r####"enumerate(null)"####,
        r####"enumerate([1], 1.5)"####,
        r####"range(1.5)"####,
        r####"range(0, 5, 0)"####,
        r####"sorted(null)"####,
        r####"sorted([1, 2], (x) -> x, "yes")"####,
        r####"sorted([[1], [2]])"####,
        r####"reversed(null)"####,
        r####"chunk([1], 0)"####,
        r####"chunk([1], 1.5)"####,
        r####"compact(null)"####,
        r####"unique(null)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}
