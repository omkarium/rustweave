# Objects functions in one script

One object is inspected as entries, names, keys, and values, then merged, divided, and consumed while a predicate remains true.

This script uses all 9 functions listed for `dw::core::Objects`.

Import `dw::core::Objects` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Objects
output application/json
---
do {
  var crate = {x: 4, y: 5, z: 6}
  var extra = {y: 9, w: 10}
  ---
  {
    entries: entrySet(crate),
    names: nameSet(crate),
    keys: keySet(crate),
    values: valueSet(crate),
    every: crate everyEntry ($ > 0),
    some: crate someEntry ($ == 6),
    merge: mergeWith(crate, extra),
    divide: divideBy(crate, 2),
    take: crate takeWhile ($ < 6)
  }
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/objects.dwl
```

## Expected output

```json
{
  "entries": [
    {
      "key": "x",
      "value": 4,
      "attributes": {
        
      }
    },
    {
      "key": "y",
      "value": 5,
      "attributes": {
        
      }
    },
    {
      "key": "z",
      "value": 6,
      "attributes": {
        
      }
    }
  ],
  "names": [
    "x",
    "y",
    "z"
  ],
  "keys": [
    "x",
    "y",
    "z"
  ],
  "values": [
    4,
    5,
    6
  ],
  "every": true,
  "some": true,
  "merge": {
    "x": 4,
    "y": 9,
    "z": 6,
    "w": 10
  },
  "divide": [
    {
      "x": 4,
      "y": 5
    },
    {
      "z": 6
    }
  ],
  "take": {
    "x": 4,
    "y": 5
  }
}
```

## Functions shown

`entrySet`, `nameSet`, `keySet`, `valueSet`, `everyEntry`, `someEntry`, `mergeWith`, `divideBy`, `takeWhile`
