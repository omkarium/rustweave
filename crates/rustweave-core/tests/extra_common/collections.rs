// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn fold_filter_and_positions() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  total: fold([4, 6, 8], 10, (item, acc) -> acc + item),
  emptyFold: fold([], 5, (item, acc) -> acc + item),
  mapped: filterMap([1, 2, 3, 4], (n) -> if ((n mod 2) == 0) n * 10 else null),
  noMatches: filterMap([1, 3], (n) -> null),
  stepped: stepBy([0, 1, 2, 3, 4, 5, 6], 3),
  stepPastEnd: stepBy([9, 8], 5),
  firstEven: position([3, 5, 8, 10], (n) -> (n mod 2) == 0),
  absent: position([1, 3], (n) -> n > 10),
  lastEven: rposition([3, 5, 8, 10, 12], (n) -> (n mod 2) == 0),
  noLast: rposition([], (n) -> true),
  deduped: dedup([1, 1, 2, 2, 1, 3, 3]),
  dedupKeyed: dedup([{id: 1, tag: "a"}, {id: 1, tag: "b"}, {id: 2, tag: "c"}], (item) -> item.id),
  dedupEmpty: dedup([])
}"####;
    let expected = r####"{
  "total": 28,
  "emptyFold": 5,
  "mapped": [
    20,
    40
  ],
  "noMatches": [
    
  ],
  "stepped": [
    0,
    3,
    6
  ],
  "stepPastEnd": [
    9
  ],
  "firstEven": 2,
  "absent": -1,
  "lastEven": 4,
  "noLast": -1,
  "deduped": [
    1,
    2,
    1,
    3
  ],
  "dedupKeyed": [
    {
      "id": 1,
      "tag": "a"
    },
    {
      "id": 2,
      "tag": "c"
    }
  ],
  "dedupEmpty": [
    
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn windows_search_rotate_and_edit() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  windows: windows([1, 2, 3, 4, 5], 3),
  windowTooBig: windows([1, 2], 3),
  chunks: chunksExact([1, 2, 3, 4, 5], 2),
  chunksTooBig: chunksExact([1, 2], 3),
  findLower: binarySearch([10, 20, 20, 30], 20),
  findMissing: binarySearch([10, 20, 30], 25),
  findBefore: binarySearch([10, 20, 30], 5),
  findAfter: binarySearch([10, 20, 30], 40),
  left: rotateLeft([1, 2, 3, 4, 5], 2),
  leftNegative: rotateLeft([1, 2, 3, 4, 5], -2),
  leftWrap: rotateLeft([1, 2, 3], 7),
  right: rotateRight([1, 2, 3, 4, 5], 2),
  rightNegative: rotateRight([1, 2, 3], -1),
  emptyRotate: rotateLeft([], 3),
  swapped: swap([1, 2, 3], 0, 2),
  sameSwap: swap([1, 2], 1, 1),
  filled: fill([0, 0, 0, 0, 0], 9, 1, 3),
  fillTail: fill([1, 2, 3], 7, 1),
  wholeFill: fill([1, 2], 0),
  truncated: truncate([1, 2, 3, 4], 2),
  truncateLong: truncate([1, 2], 5),
  truncateZero: truncate([1, 2], 0)
}"####;
    let expected = r####"{
  "windows": [
    [
      1,
      2,
      3
    ],
    [
      2,
      3,
      4
    ],
    [
      3,
      4,
      5
    ]
  ],
  "windowTooBig": [
    
  ],
  "chunks": [
    [
      1,
      2
    ],
    [
      3,
      4
    ]
  ],
  "chunksTooBig": [
    
  ],
  "findLower": {
    "found": true,
    "index": 1
  },
  "findMissing": {
    "found": false,
    "index": 2
  },
  "findBefore": {
    "found": false,
    "index": 0
  },
  "findAfter": {
    "found": false,
    "index": 3
  },
  "left": [
    3,
    4,
    5,
    1,
    2
  ],
  "leftNegative": [
    4,
    5,
    1,
    2,
    3
  ],
  "leftWrap": [
    2,
    3,
    1
  ],
  "right": [
    4,
    5,
    1,
    2,
    3
  ],
  "rightNegative": [
    2,
    3,
    1
  ],
  "emptyRotate": [
    
  ],
  "swapped": [
    3,
    2,
    1
  ],
  "sameSwap": [
    1,
    2
  ],
  "filled": [
    0,
    9,
    9,
    0,
    0
  ],
  "fillTail": [
    1,
    7,
    7
  ],
  "wholeFill": [
    0,
    0
  ],
  "truncated": [
    1,
    2
  ],
  "truncateLong": [
    1,
    2
  ],
  "truncateZero": [
    
  ]
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn collection_argument_errors() {
    for script in [
        r####"fold(null, 0, (item, acc) -> acc)"####,
        r####"fold([1], 0, 1)"####,
        r####"filterMap(null, (n) -> n)"####,
        r####"filterMap([1], 1)"####,
        r####"stepBy([1], 0)"####,
        r####"stepBy([1], 1.5)"####,
        r####"stepBy(null, 1)"####,
        r####"position(null, (n) -> true)"####,
        r####"position([1], 1)"####,
        r####"rposition(null, (n) -> true)"####,
        r####"dedup(null)"####,
        r####"dedup([1], 1)"####,
        r####"windows([1], 0)"####,
        r####"windows([1], 1.5)"####,
        r####"windows(null, 1)"####,
        r####"chunksExact([1], 0)"####,
        r####"chunksExact([1], 1.5)"####,
        r####"chunksExact(null, 1)"####,
        r####"binarySearch(null, 1)"####,
        r####"binarySearch([1, "a"], 1)"####,
        r####"rotateLeft(null, 1)"####,
        r####"rotateLeft([1], 1.5)"####,
        r####"rotateRight(null, 1)"####,
        r####"swap(null, 0, 0)"####,
        r####"swap([1], 0, 1)"####,
        r####"swap([1], -1, 0)"####,
        r####"swap([1], 0, 0.5)"####,
        r####"fill(null, 0)"####,
        r####"fill([1], 0, -1)"####,
        r####"fill([1], 0, 0, 2)"####,
        r####"fill([1], 0, 1, 0)"####,
        r####"fill([1], 0, 0, 0.5)"####,
        r####"truncate(null, 1)"####,
        r####"truncate([1], -1)"####,
        r####"truncate([1], 1.5)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}
