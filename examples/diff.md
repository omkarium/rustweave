# Diff functions in one script

The diff helper compares missing keys, nested values, equal objects, and equal objects with different key order.

This script uses all 1 functions listed for `dw::util::Diff`.

Import `diff` from `dw::util::Diff`.

## Script

```dataweave
%dw 2.0
import diff from dw::util::Diff
output application/json
---
{
  missing: diff({x: 2}, {y: 2}),
  nested: diff([{x: 2}], [{x: 3}]),
  equal: diff({x: 2}, {x: 2}),
  unordered: diff({x: 1, y: 2}, {y: 2, x: 1}, {unordered: true})
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/diff.dwl
```

## Expected output

```json
{
  "missing": {
    "matches": false,
    "diffs": [
      {
        "expected": "Entry (root).y with type Number",
        "actual": "was not present in object.",
        "path": "(root).y"
      }
    ]
  },
  "nested": {
    "matches": false,
    "diffs": [
      {
        "expected": "3",
        "actual": "2",
        "path": "(root)[0].x"
      }
    ]
  },
  "equal": {
    "matches": true,
    "diffs": [
      
    ]
  },
  "unordered": {
    "matches": true,
    "diffs": [
      
    ]
  }
}
```

## Functions shown

`diff`
